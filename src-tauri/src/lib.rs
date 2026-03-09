mod analysis;
mod application;
mod audio;
mod commands;
mod state;

use state::TemplateState;

pub fn run() {
    tauri::Builder::default()
        .manage(TemplateState::new())
        .invoke_handler(tauri::generate_handler![
            commands::starter::starter_overview,
            commands::starter::audio_transport_status,
            commands::starter::audio_transport_start,
            commands::starter::audio_transport_stop
        ])
        .run(tauri::generate_context!())
        .expect("failed to run tauri audio starter");
}
