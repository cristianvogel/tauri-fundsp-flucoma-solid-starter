use crate::audio::decoder::{decode_embedded_preview, DecodePreview};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use fundsp::audiounit::AudioUnit;
use fundsp::prelude::{sine_hz, Wave};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub const TRANSPORT_EVENT_NAME: &str = "audio://transport";

#[derive(Debug, Clone, Serialize)]
pub struct AudioPreview {
    pub sample_rate_hz: u32,
    pub rendered_frames: usize,
    pub peak_amplitude: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTransportStatus {
    pub running: bool,
    pub output_available: bool,
    pub device_name: Option<String>,
    pub sample_rate_hz: Option<u32>,
    pub channel_count: Option<u16>,
    pub transport_mode: String,
    pub decode_status: String,
    pub last_error: Option<String>,
    pub frames_rendered: u64,
    pub playhead_seconds: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTransportEvent {
    pub kind: String,
    pub status: AudioTransportStatus,
    pub message: Option<String>,
}

pub struct AudioRuntime {
    inner: Mutex<AudioRuntimeInner>,
}

struct AudioRuntimeInner {
    running_transport: Option<RunningTransport>,
    decode_preview: Result<DecodePreview, String>,
}

struct RunningTransport {
    device_name: String,
    sample_rate_hz: u32,
    channel_count: u16,
    stop_tx: Sender<AudioThreadCommand>,
    thread_handle: JoinHandle<()>,
    last_error: Arc<Mutex<Option<String>>>,
    frames_rendered: Arc<AtomicU64>,
}

struct TransportSnapshot<'a> {
    running: bool,
    device_name: Option<String>,
    sample_rate_hz: Option<u32>,
    channel_count: Option<u16>,
    last_error: &'a Arc<Mutex<Option<String>>>,
    frames_rendered: &'a Arc<AtomicU64>,
    transport_mode: &'a str,
    decode_status: String,
}

enum AudioThreadCommand {
    Stop,
}

struct TransportVoice {
    unit: Box<dyn AudioUnit>,
}

impl TransportVoice {
    fn new(sample_rate_hz: u32) -> Self {
        let mut unit: Box<dyn AudioUnit> =
            Box::new((sine_hz::<f32>(220.0) * 0.14) | (sine_hz::<f32>(220.7) * 0.14));
        unit.set_sample_rate(sample_rate_hz as f64);
        unit.allocate();

        Self { unit }
    }

    fn next_frame(&mut self) -> (f32, f32) {
        self.unit.get_stereo()
    }
}

impl AudioRuntime {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(AudioRuntimeInner {
                running_transport: None,
                decode_preview: decode_embedded_preview(),
            }),
        }
    }

    pub fn preview(&self) -> AudioPreview {
        let mut voice = sine_hz::<f32>(220.0) * 0.2;
        let wave = Wave::render(48_000.0, 0.05, &mut voice);

        AudioPreview {
            sample_rate_hz: 48_000,
            rendered_frames: wave.len(),
            peak_amplitude: wave.amplitude(),
        }
    }

    pub fn status(&self) -> AudioTransportStatus {
        let mut inner = self
            .inner
            .lock()
            .expect("audio runtime mutex poisoned while reading status");
        inner.reap_finished_transport();
        build_status_from_inner(&inner)
    }

    pub fn start_transport(&self, app_handle: AppHandle) -> Result<AudioTransportStatus, String> {
        let mut inner = self
            .inner
            .lock()
            .expect("audio runtime mutex poisoned while starting transport");
        inner.reap_finished_transport();

        if inner.running_transport.is_some() {
            return Ok(build_status_from_inner(&inner));
        }

        let decode_status = describe_decode_preview(&inner.decode_preview);
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "no default output device available".to_string())?;
        let device_name = device
            .name()
            .unwrap_or_else(|_| "Unnamed output device".to_string());
        let supported_config = device
            .default_output_config()
            .map_err(|error| format!("failed to get default output config: {error}"))?;
        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.config();
        let sample_rate_hz = config.sample_rate.0;
        let channel_count = config.channels;

        let (stop_tx, stop_rx) = mpsc::channel::<AudioThreadCommand>();
        let (startup_tx, startup_rx) = mpsc::sync_channel::<Result<(), String>>(1);
        let last_error = Arc::new(Mutex::new(None::<String>));
        let frames_rendered = Arc::new(AtomicU64::new(0));
        let last_error_for_thread = Arc::clone(&last_error);
        let frames_rendered_for_thread = Arc::clone(&frames_rendered);
        let app_handle_for_thread = app_handle.clone();
        let thread_device_name = device_name.clone();
        let thread_decode_status = decode_status.clone();

        let thread_handle = thread::Builder::new()
            .name("starter-audio-transport".to_string())
            .spawn(move || {
                let voice = Arc::new(Mutex::new(TransportVoice::new(sample_rate_hz)));
                let stream = match sample_format {
                    SampleFormat::F32 => build_output_stream::<f32>(
                        &device,
                        &config,
                        Arc::clone(&voice),
                        Arc::clone(&last_error_for_thread),
                        Arc::clone(&frames_rendered_for_thread),
                    ),
                    SampleFormat::I16 => build_output_stream::<i16>(
                        &device,
                        &config,
                        Arc::clone(&voice),
                        Arc::clone(&last_error_for_thread),
                        Arc::clone(&frames_rendered_for_thread),
                    ),
                    SampleFormat::U16 => build_output_stream::<u16>(
                        &device,
                        &config,
                        Arc::clone(&voice),
                        Arc::clone(&last_error_for_thread),
                        Arc::clone(&frames_rendered_for_thread),
                    ),
                    sample_format => Err(format!(
                        "unsupported output sample format for {thread_device_name}: {sample_format:?}"
                    )),
                };

                match stream {
                    Ok(stream) => {
                        if let Err(error) = stream.play() {
                            let message = format!("failed to start output stream: {error}");
                            if let Ok(mut last_error) = last_error_for_thread.lock() {
                                *last_error = Some(message.clone());
                            }
                            emit_transport_event(
                                &app_handle_for_thread,
                                "error",
                                build_transport_status_snapshot(TransportSnapshot {
                                    running: false,
                                    device_name: Some(thread_device_name.clone()),
                                    sample_rate_hz: Some(sample_rate_hz),
                                    channel_count: Some(channel_count),
                                    last_error: &last_error_for_thread,
                                    frames_rendered: &frames_rendered_for_thread,
                                    transport_mode: "stopped",
                                    decode_status: thread_decode_status.clone(),
                                }),
                                Some(message.clone()),
                            );
                            let _ = startup_tx.send(Err(message));
                            return;
                        }

                        let _ = startup_tx.send(Ok(()));
                        hold_stream_until_stop(
                            stream,
                            stop_rx,
                            &last_error_for_thread,
                            &frames_rendered_for_thread,
                            &app_handle_for_thread,
                            thread_device_name,
                            sample_rate_hz,
                            channel_count,
                            thread_decode_status,
                        );
                    }
                    Err(error) => {
                        if let Ok(mut last_error) = last_error_for_thread.lock() {
                            *last_error = Some(error.clone());
                        }
                        emit_transport_event(
                            &app_handle_for_thread,
                            "error",
                            build_transport_status_snapshot(TransportSnapshot {
                                running: false,
                                device_name: Some(thread_device_name),
                                sample_rate_hz: Some(sample_rate_hz),
                                channel_count: Some(channel_count),
                                last_error: &last_error_for_thread,
                                frames_rendered: &frames_rendered_for_thread,
                                transport_mode: "stopped",
                                decode_status: thread_decode_status,
                            }),
                            Some(error.clone()),
                        );
                        let _ = startup_tx.send(Err(error));
                    }
                }
            })
            .map_err(|error| format!("failed to spawn audio thread: {error}"))?;

        match startup_rx.recv() {
            Ok(Ok(())) => {
                inner.running_transport = Some(RunningTransport {
                    device_name,
                    sample_rate_hz,
                    channel_count,
                    stop_tx,
                    thread_handle,
                    last_error,
                    frames_rendered,
                });
                let status = build_status_from_inner(&inner);
                emit_transport_event(&app_handle, "status_changed", status.clone(), None);
                emit_transport_event(&app_handle, "device_changed", status.clone(), None);
                Ok(status)
            }
            Ok(Err(error)) => {
                let _ = thread_handle.join();
                Err(error)
            }
            Err(error) => {
                let _ = thread_handle.join();
                Err(format!(
                    "audio thread failed before startup completed: {error}"
                ))
            }
        }
    }

    pub fn stop_transport(&self, app_handle: AppHandle) -> AudioTransportStatus {
        let mut inner = self
            .inner
            .lock()
            .expect("audio runtime mutex poisoned while stopping transport");

        if let Some(transport) = inner.running_transport.take() {
            let _ = transport.stop_tx.send(AudioThreadCommand::Stop);
            let _ = transport.thread_handle.join();
        }

        let status = build_status_from_inner(&inner);
        emit_transport_event(&app_handle, "status_changed", status.clone(), None);
        status
    }
}

impl AudioRuntimeInner {
    fn reap_finished_transport(&mut self) {
        let should_reap = self
            .running_transport
            .as_ref()
            .map(|transport| transport.thread_handle.is_finished())
            .unwrap_or(false);

        if !should_reap {
            return;
        }

        if let Some(transport) = self.running_transport.take() {
            let _ = transport.thread_handle.join();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn hold_stream_until_stop(
    _stream: Stream,
    stop_rx: mpsc::Receiver<AudioThreadCommand>,
    last_error: &Arc<Mutex<Option<String>>>,
    frames_rendered: &Arc<AtomicU64>,
    app_handle: &AppHandle,
    device_name: String,
    sample_rate_hz: u32,
    channel_count: u16,
    decode_status: String,
) {
    let mut last_emitted_frames = 0u64;

    loop {
        match stop_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(AudioThreadCommand::Stop) => break,
            Err(RecvTimeoutError::Timeout) => {
                let current_frames = frames_rendered.load(Ordering::Relaxed);
                if current_frames != last_emitted_frames {
                    last_emitted_frames = current_frames;
                    emit_transport_event(
                        app_handle,
                        "timing",
                        build_transport_status_snapshot(TransportSnapshot {
                            running: true,
                            device_name: Some(device_name.clone()),
                            sample_rate_hz: Some(sample_rate_hz),
                            channel_count: Some(channel_count),
                            last_error,
                            frames_rendered,
                            transport_mode: "fundsp realtime output via cpal",
                            decode_status: decode_status.clone(),
                        }),
                        None,
                    );
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                if let Ok(mut error) = last_error.lock() {
                    *error = Some("audio control channel disconnected".to_string());
                }
                emit_transport_event(
                    app_handle,
                    "error",
                    build_transport_status_snapshot(TransportSnapshot {
                        running: false,
                        device_name: Some(device_name),
                        sample_rate_hz: Some(sample_rate_hz),
                        channel_count: Some(channel_count),
                        last_error,
                        frames_rendered,
                        transport_mode: "stopped",
                        decode_status,
                    }),
                    Some("audio control channel disconnected".to_string()),
                );
                break;
            }
        }
    }
}

fn build_output_stream<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    voice: Arc<Mutex<TransportVoice>>,
    error_state: Arc<Mutex<Option<String>>>,
    frames_rendered: Arc<AtomicU64>,
) -> Result<Stream, String>
where
    T: Sample + FromSample<f32> + SizedSample,
{
    let channel_count = usize::from(config.channels);
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _info| {
                write_output_data(data, channel_count, &voice, &frames_rendered);
            },
            move |error| {
                if let Ok(mut last_error) = error_state.lock() {
                    *last_error = Some(error.to_string());
                }
            },
            None,
        )
        .map_err(|error| format!("failed to build output stream: {error}"))
}

fn write_output_data<T>(
    data: &mut [T],
    channel_count: usize,
    voice: &Arc<Mutex<TransportVoice>>,
    frames_rendered: &Arc<AtomicU64>,
) where
    T: Sample + FromSample<f32>,
{
    let Ok(mut voice) = voice.lock() else {
        write_silence(data);
        return;
    };

    for frame in data.chunks_mut(channel_count.max(1)) {
        let (left, right) = voice.next_frame();
        for (channel_index, sample) in frame.iter_mut().enumerate() {
            let value = match channel_index {
                0 => left,
                1 => right,
                _ => (left + right) * 0.5,
            };
            *sample = T::from_sample(value);
        }
    }

    frames_rendered.fetch_add(
        data.len().saturating_div(channel_count.max(1)) as u64,
        Ordering::Relaxed,
    );
}

fn write_silence<T>(data: &mut [T])
where
    T: Sample + FromSample<f32>,
{
    for sample in data.iter_mut() {
        *sample = T::from_sample(0.0);
    }
}

fn describe_decode_preview(preview: &Result<DecodePreview, String>) -> String {
    match preview {
        Ok(preview) => format!(
            "symphonia decoded {} frames from {} at {} Hz / {} ch.",
            preview.decoded_frames,
            preview.source_name,
            preview.sample_rate_hz,
            preview.channel_count
        ),
        Err(error) => format!("symphonia decode preview failed: {error}"),
    }
}

fn default_output_device_name() -> Option<String> {
    cpal::default_host()
        .default_output_device()
        .and_then(|device| device.name().ok())
}

fn build_status_from_inner(inner: &AudioRuntimeInner) -> AudioTransportStatus {
    let last_error = inner.running_transport.as_ref().and_then(|transport| {
        transport
            .last_error
            .lock()
            .ok()
            .and_then(|error| error.clone())
    });
    let frames_rendered = inner
        .running_transport
        .as_ref()
        .map(|transport| transport.frames_rendered.load(Ordering::Relaxed))
        .unwrap_or(0);
    let sample_rate_hz = inner
        .running_transport
        .as_ref()
        .map(|transport| transport.sample_rate_hz);

    AudioTransportStatus {
        running: inner.running_transport.is_some(),
        output_available: default_output_device_name().is_some(),
        device_name: inner
            .running_transport
            .as_ref()
            .map(|transport| transport.device_name.clone())
            .or_else(default_output_device_name),
        sample_rate_hz,
        channel_count: inner
            .running_transport
            .as_ref()
            .map(|transport| transport.channel_count),
        transport_mode: if inner.running_transport.is_some() {
            "fundsp realtime output via cpal".to_string()
        } else {
            "stopped".to_string()
        },
        decode_status: describe_decode_preview(&inner.decode_preview),
        last_error,
        frames_rendered,
        playhead_seconds: sample_rate_hz
            .map(|rate| frames_rendered as f64 / f64::from(rate))
            .unwrap_or(0.0),
    }
}

fn build_transport_status_snapshot(snapshot: TransportSnapshot<'_>) -> AudioTransportStatus {
    let rendered = snapshot.frames_rendered.load(Ordering::Relaxed);

    AudioTransportStatus {
        running: snapshot.running,
        output_available: default_output_device_name().is_some(),
        device_name: snapshot.device_name,
        sample_rate_hz: snapshot.sample_rate_hz,
        channel_count: snapshot.channel_count,
        transport_mode: snapshot.transport_mode.to_string(),
        decode_status: snapshot.decode_status,
        last_error: snapshot
            .last_error
            .lock()
            .ok()
            .and_then(|error| error.clone()),
        frames_rendered: rendered,
        playhead_seconds: snapshot
            .sample_rate_hz
            .map(|rate| rendered as f64 / f64::from(rate))
            .unwrap_or(0.0),
    }
}

fn emit_transport_event(
    app_handle: &AppHandle,
    kind: &str,
    status: AudioTransportStatus,
    message: Option<String>,
) {
    let _ = app_handle.emit(
        TRANSPORT_EVENT_NAME,
        AudioTransportEvent {
            kind: kind.to_string(),
            status,
            message,
        },
    );
}
