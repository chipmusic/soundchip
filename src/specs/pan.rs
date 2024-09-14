use crate::quantize_steps_f32;

/// The processing specs for pan values.
#[derive(Debug, Clone, PartialEq)]
pub struct PanSpecs {
    /// Quantizes the stereo pan state, i.e. 4 bit pan register = 16 steps.
    pub steps: Option<u16>,
}

impl Default for PanSpecs {
    fn default() -> Self {
        Self {
            steps: Some(4096),
        }
    }
}

impl PanSpecs {

    pub fn psg() -> Self {
        Self { steps: Some(16) }
    }

    pub fn scc() -> Self {
        Self { steps: Some(16) }
    }

    pub fn get(&self, value:f32) -> f32 {
        match self.steps {
            Some(steps) => quantize_steps_f32(value, steps),
            None => value,
        }
    }
}
