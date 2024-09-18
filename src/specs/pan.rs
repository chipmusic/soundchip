/// The processing specs for pan values.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecsPan {
    /// Quantizes the stereo pan state, i.e. 4 bit pan register = 16 steps.
    pub steps: Option<u16>,
}

impl Default for SpecsPan {
    fn default() -> Self {
        Self {
            steps: Some(16),
        }
    }
}

impl SpecsPan {
    pub fn psg() -> Self {
        Self { steps: Some(16) }
    }

    pub fn scc() -> Self {
        Self { steps: Some(16) }
    }
}
