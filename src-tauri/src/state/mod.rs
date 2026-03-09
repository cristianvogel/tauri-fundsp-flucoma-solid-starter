use crate::analysis::service::AnalysisRuntime;
use crate::audio::engine::AudioRuntime;

pub struct TemplateState {
    pub analysis: AnalysisRuntime,
    pub audio: AudioRuntime,
}

impl TemplateState {
    pub fn new() -> Self {
        Self {
            analysis: AnalysisRuntime::new(),
            audio: AudioRuntime::new(),
        }
    }
}
