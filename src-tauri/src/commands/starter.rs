use crate::application;
use crate::audio::engine::AudioTransportStatus;
use crate::state::TemplateState;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StarterOverview {
    pub starter_name: String,
    pub frontend_stack: Vec<String>,
    pub backend_stack: Vec<String>,
    pub architecture: Vec<application::ArchitectureArea>,
    pub flucoma_status: String,
    pub fundsp_status: String,
    pub next_steps: Vec<String>,
}

#[tauri::command]
pub fn starter_overview(state: tauri::State<'_, TemplateState>) -> StarterOverview {
    let analysis_preview = state.analysis.preview();
    let audio_preview = state.audio.preview();
    let transport_status = state.audio.status();

    StarterOverview {
        starter_name: application::starter_name().to_string(),
        frontend_stack: application::frontend_stack()
            .into_iter()
            .map(str::to_string)
            .collect(),
        backend_stack: application::backend_stack()
            .into_iter()
            .map(str::to_string)
            .collect(),
        architecture: application::architecture(),
        flucoma_status: format!(
            "{} Sample segments: {}. Normalized point rows: {}.",
            analysis_preview.status,
            analysis_preview.sample_segments,
            analysis_preview.normalized_points
        ),
        fundsp_status: format!(
            "fundsp renders a starter oscillator preview at {} Hz with {} frames and peak amplitude {:.3}. Realtime transport: {}.",
            audio_preview.sample_rate_hz,
            audio_preview.rendered_frames,
            audio_preview.peak_amplitude,
            transport_status.transport_mode
        ),
        next_steps: vec![
            "Wire the upload panel into Symphonia decode and generate a real magnitude spectrogram."
                .to_string(),
            "Run offline BufNMF in the analysis runtime and persist bases, activations, and stems."
                .to_string(),
            "Promote the NMFFilter handoff into a realtime input path that reuses saved bases."
                .to_string(),
        ],
    }
}

#[tauri::command]
pub fn audio_transport_status(state: tauri::State<'_, TemplateState>) -> AudioTransportStatus {
    state.audio.status()
}

#[tauri::command]
pub fn audio_transport_start(
    state: tauri::State<'_, TemplateState>,
    app_handle: tauri::AppHandle,
) -> Result<AudioTransportStatus, String> {
    state.audio.start_transport(app_handle)
}

#[tauri::command]
pub fn audio_transport_stop(
    state: tauri::State<'_, TemplateState>,
    app_handle: tauri::AppHandle,
) -> AudioTransportStatus {
    state.audio.stop_transport(app_handle)
}
