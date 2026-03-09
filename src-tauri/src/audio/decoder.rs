use serde::Serialize;
use std::f32::consts::PI;
use std::io::{Cursor, ErrorKind};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

#[derive(Debug, Clone, Serialize)]
pub struct DecodePreview {
    pub sample_rate_hz: u32,
    pub channel_count: usize,
    pub decoded_frames: usize,
    pub source_name: String,
}

pub fn decode_embedded_preview() -> Result<DecodePreview, String> {
    let media_source = MediaSourceStream::new(
        Box::new(Cursor::new(build_preview_wav_bytes())),
        Default::default(),
    );

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let probed = get_probe()
        .format(
            &hint,
            media_source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|error| format!("symphonia probe failed: {error}"))?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| "symphonia found no default track in embedded preview".to_string())?;
    let track_id = track.id;
    let codec_params = &track.codec_params;
    let sample_rate_hz = codec_params
        .sample_rate
        .ok_or_else(|| "embedded preview reported no sample rate".to_string())?;
    let channel_count = codec_params
        .channels
        .map(|channels| channels.count())
        .unwrap_or(1);

    let mut decoder = get_codecs()
        .make(codec_params, &DecoderOptions::default())
        .map_err(|error| format!("symphonia decoder init failed: {error}"))?;

    let mut decoded_frames = 0usize;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            Err(SymphoniaError::ResetRequired) => {
                return Err("embedded preview requested unsupported decoder reset".to_string());
            }
            Err(error) => return Err(format!("symphonia packet read failed: {error}")),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder
            .decode(&packet)
            .map_err(|error| format!("symphonia decode failed: {error}"))?;

        let mut sample_buffer =
            SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        sample_buffer.copy_interleaved_ref(decoded);
        decoded_frames += sample_buffer.samples().len() / channel_count.max(1);
    }

    Ok(DecodePreview {
        sample_rate_hz,
        channel_count,
        decoded_frames,
        source_name: "embedded wav preview".to_string(),
    })
}

fn build_preview_wav_bytes() -> Vec<u8> {
    const SAMPLE_RATE_HZ: u32 = 44_100;
    const CHANNELS: u16 = 1;
    const BITS_PER_SAMPLE: u16 = 16;
    const DURATION_SECONDS: f32 = 0.2;
    const FREQUENCY_HZ: f32 = 220.0;
    const AMPLITUDE: f32 = 0.25;

    let total_frames = (SAMPLE_RATE_HZ as f32 * DURATION_SECONDS) as usize;
    let block_align = CHANNELS * (BITS_PER_SAMPLE / 8);
    let byte_rate = SAMPLE_RATE_HZ * u32::from(block_align);

    let mut pcm_bytes = Vec::with_capacity(total_frames * usize::from(block_align));
    for frame_index in 0..total_frames {
        let phase = 2.0 * PI * FREQUENCY_HZ * frame_index as f32 / SAMPLE_RATE_HZ as f32;
        let sample = (phase.sin() * AMPLITUDE * i16::MAX as f32) as i16;
        pcm_bytes.extend_from_slice(&sample.to_le_bytes());
    }

    let data_size = pcm_bytes.len() as u32;
    let mut wav_bytes = Vec::with_capacity(44 + pcm_bytes.len());
    wav_bytes.extend_from_slice(b"RIFF");
    wav_bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav_bytes.extend_from_slice(b"WAVE");
    wav_bytes.extend_from_slice(b"fmt ");
    wav_bytes.extend_from_slice(&16u32.to_le_bytes());
    wav_bytes.extend_from_slice(&1u16.to_le_bytes());
    wav_bytes.extend_from_slice(&CHANNELS.to_le_bytes());
    wav_bytes.extend_from_slice(&SAMPLE_RATE_HZ.to_le_bytes());
    wav_bytes.extend_from_slice(&byte_rate.to_le_bytes());
    wav_bytes.extend_from_slice(&block_align.to_le_bytes());
    wav_bytes.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    wav_bytes.extend_from_slice(b"data");
    wav_bytes.extend_from_slice(&data_size.to_le_bytes());
    wav_bytes.extend_from_slice(&pcm_bytes);
    wav_bytes
}
