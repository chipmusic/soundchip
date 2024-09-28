use hound::{WavSpec, WavWriter};
use mini_sdl::SdlResult;
use soundchip::prelude::Sample;
use std::{env::var_os, fs::File, io::BufWriter, path::PathBuf};

pub struct WaveWriter {
    writer: Option<WavWriter<BufWriter<File>>>,
}

impl WaveWriter {
    pub fn new(sample_rate: u32) -> Self {
        // Writing in mono for debugging simplicity. Ensure no pan is set in the channel!
        let target_file: Option<PathBuf> = match var_os("CARGO_MANIFEST_DIR") {
            Some(os_var) => {
                let dir: PathBuf = os_var.into();
                Some(dir.join("target/output.wav"))
            }
            None => None,
        };
        println!("Saving wav file to: {:?}", target_file);
        let wav_spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = if let Some(target_file) = target_file {
            WavWriter::create(target_file, wav_spec).ok()
        } else {
            None
        };
        Self { writer }
    }

    pub fn write(&mut self, sample:Sample<i16>) -> SdlResult {
        if let Some(writer) = &mut self.writer {
            writer
                .write_sample(i16::from(sample.left))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn save_file(self) -> SdlResult{
        // I have this path set to a ram disk on my machine,
        // since I'm saving the wave file for debugging purposes.
        if let Some(writer) = self.writer {
            writer.finalize().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

}
