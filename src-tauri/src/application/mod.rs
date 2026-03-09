use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ArchitectureArea {
    pub area: &'static str,
    pub purpose: &'static str,
}

pub fn starter_name() -> &'static str {
    "Tauri Audio Starter"
}

pub fn frontend_stack() -> Vec<&'static str> {
    vec!["SolidJS", "Vite", "@tauri-apps/api"]
}

pub fn backend_stack() -> Vec<&'static str> {
    vec![
        "Tauri 2",
        "fundsp",
        "flucoma-rs (WIP)",
        "shared analysis DTO crate",
    ]
}

pub fn architecture() -> Vec<ArchitectureArea> {
    vec![
        ArchitectureArea {
            area: "application",
            purpose: "Compose the starter metadata and high-level backend shape.",
        },
        ArchitectureArea {
            area: "analysis",
            purpose: "Own flucoma-rs experiments, segmentation, and feature extraction pipelines.",
        },
        ArchitectureArea {
            area: "audio",
            purpose: "Own fundsp graphs, preview rendering, and future realtime playback.",
        },
        ArchitectureArea {
            area: "commands",
            purpose: "Expose a thin Tauri boundary with serializable request/response types.",
        },
        ArchitectureArea {
            area: "state",
            purpose: "Hold long-lived runtimes and future project/session state.",
        },
    ]
}
