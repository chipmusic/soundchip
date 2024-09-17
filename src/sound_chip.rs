use crate::*;

const MAX_I16: f32 = (i16::MAX - 1) as f32;

/// Contains multiple sound channels, and can render and mix them all at once.
pub struct SoundChip {
    /// The sampling rate at which mixing is performed. Should match your audio playback device,
    /// but can be lower for improved performance. Usually 44100 or 48000.
    pub sample_rate: u32,
    /// Applies a correction per channel to help avoid clipping the samples beyond -1.0 to 1.0.
    pub auto_prevent_clipping: bool,
    /// Vector containing sound channels. You can directly manipulate it to add/remove channels.
    pub channels: Vec<Channel>,
    sample_head: usize,
    last_sample_time: f32,
}

impl Default for SoundChip {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            sample_rate: 44100,
            auto_prevent_clipping: true,
            sample_head: 0,
            last_sample_time: 0.0,
        }
    }
}

impl SoundChip {
    /// Creates a SoundChip with no sound channels.
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            ..Default::default()
        }
    }

    /// Creates a SoundChip configured to replicate an AY-3-8910 sound chip with 3 square wave channels.
    pub fn new_msx(sample_rate: u32) -> Self {
        // println!("New MSX sound chip");
        Self {
            channels: (0..3)
                .map(|i| match i {
                    0 => Channel::new_psg(true),
                    _ => Channel::new_psg(false),
                })
                .collect(),
            sample_rate,
            ..Default::default()
            // sample_head: 0,
            // last_sample_time: 0.0,
        }
    }

    /// Creates a SoundChip configured to replicate an AY-3-8910 sound chip with 3 square wave channels plus
    /// an SCC chip with 5 wavetable channels (32 byte samples).
    pub fn new_msx_scc(sample_rate: u32) -> Self {
        // println!("New MSX-SCC sound chip");
        Self {
            channels: (0..8)
                .map(|i| match i {
                    0 => Channel::new_psg(true),
                    1 | 2 => Channel::new_psg(false),
                    _ => Channel::new_scc(),
                })
                .collect(),
            sample_rate,
            ..Default::default()
            // sample_head: 0,
            // last_sample_time: 0.0,
        }
    }

    /// Initializes every channel, and optionally starts playing them.
    pub fn channel_init_all(&mut self, play: bool) {
        for channel in &mut self.channels {
            channel.reset_time();
            channel.set_note(4, Note::C);
            if play {
                channel.play()
            } else {
                channel.calculate_multipliers();
            }
        }
    }

    /// Stops playback of all channels at once.
    pub fn channel_stop_all(&mut self) {
        for channel in &mut self.channels {
            channel.stop();
        }
    }

    /// Renders a given number of samples on demand. Normally the requested sample count
    /// should be 'sample_rate * elapsed_time';
    pub fn iter(&mut self, sample_count: usize) -> SoundChipIter {
        SoundChipIter {
            chip: self,
            head: 0,
            sample_count,
        }
    }

    /// Process a single sample, advancing internal timer.
    pub fn process_sample(&mut self) -> Sample<i16> {
        let mut left: f32 = 0.0;
        let mut right: f32 = 0.0;

        let time = self.sample_head as f32/ self.sample_rate as f32;
        let delta_time = time - self.last_sample_time;
        self.last_sample_time = time;

        for channel in &mut self.channels {
            let sample = channel.sample(delta_time);
            left += sample.left;
            right += sample.right;
        }

        self.sample_head += 1;
        // TODO: Move out of this function
        let vol = if self.auto_prevent_clipping {
            self.channels.len() as f32 * (2.0 / 3.0)
        } else {
            2.0
        };

        Sample {
            left: (compress_volume(left, vol).clamp(-1.0, 1.0) * MAX_I16) as i16,
            right: (compress_volume(right, vol).clamp(-1.0, 1.0) * MAX_I16) as i16,
        }
    }
}

/// Iterates a specified number of samples. Use [SoundChip::iter()] to obtain this.
pub struct SoundChipIter<'a> {
    chip: &'a mut SoundChip,
    head: usize,
    sample_count: usize,
}

impl<'a> Iterator for SoundChipIter<'a> {
    type Item = Sample<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < self.sample_count {
            self.head += 1;
            return Some(self.chip.process_sample());
        }
        None
    }
}
