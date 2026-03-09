use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Segment {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisParams {
    pub window_size: usize,
    pub hop_size: usize,
    pub fft_size: usize,
    pub novelty_kernel: usize,
    pub novelty_filter: usize,
    pub novelty_threshold: f32,
    pub min_slice_length: usize,
    pub max_simultaneous_grains: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub segments: Vec<Segment>,
    pub features: Vec<Vec<f32>>,
    pub points: Vec<[f32; 2]>,
    pub positions: Vec<f32>,
    pub novelty: Vec<f32>,
    pub rms: Vec<f32>,
}
