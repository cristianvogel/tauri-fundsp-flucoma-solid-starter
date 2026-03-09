use flucoma_core::{AnalysisParams, AnalysisResult, Segment};
use flucoma_rs::data::Normalize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisPreview {
    pub sample_segments: usize,
    pub normalized_points: usize,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct AnalysisRuntime;

impl AnalysisRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn preview(&self) -> AnalysisPreview {
        let mut normalizer =
            Normalize::new(0.0, 1.0).expect("starter normalizer configuration should be valid");
        let data = vec![0.1, 0.3, 0.7, 0.2, 0.6, 0.9];
        let normalized = normalizer
            .fit_transform(&data, 3, 2)
            .expect("starter normalization example should succeed");

        let params = AnalysisParams {
            window_size: 1024,
            hop_size: 512,
            fft_size: 2048,
            novelty_kernel: 8,
            novelty_filter: 3,
            novelty_threshold: 0.25,
            min_slice_length: 2048,
            max_simultaneous_grains: 32,
        };

        let sample_result = AnalysisResult {
            segments: vec![
                Segment {
                    start: 0,
                    end: 2048,
                },
                Segment {
                    start: 2048,
                    end: 4096,
                },
            ],
            features: vec![vec![0.1, 0.3], vec![0.7, 0.2]],
            points: vec![[0.0, 0.0], [1.0, 1.0]],
            positions: vec![0.0, 0.5],
            novelty: vec![0.2, 0.8],
            rms: vec![0.4, 0.6],
        };

        AnalysisPreview {
            sample_segments: sample_result.segments.len(),
            normalized_points: normalized.len() / 2,
            status: format!(
                "flucoma-rs is wired through a starter normalization pass; analysis params boot with fft_size {}.",
                params.fft_size
            ),
        }
    }
}
