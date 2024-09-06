use hound::{WavSpec, WavWriter};
use mini_sdl::*;
use soundchip::*;
use std::path::Path;

fn main() -> SdlResult {
    let env_step = 1.0 / 60.0;
    let ch = 0;
    let mut chip = SoundChip::new_msx_scc(48000);
    let mut app = App::new(
        "chip",
        320,
        240,
        Timing::VsyncLimitFPS(60.0),
        Scaling::StretchToWindow,
        chip.sample_rate,
    )?;
    app.audio_start();

    if let Some(channel) = chip.channel(ch) {
        channel.play();
    }

    let wav_spec = WavSpec {
        channels: 2,
        sample_rate: chip.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer =
        WavWriter::create(Path::new("output.wav"), wav_spec).map_err(|e| e.to_string())?;

    // let mut play_note_time = Instant::now();
    while !app.quit_requested {
        app.frame_start()?;

        // Use Key arrows to pitch note up or down
        if let Some(channel) = chip.channel(ch) {
            if channel.is_playing() {
                let vol = channel.volume();
                channel.set_volume((vol - env_step).clamp(0.0, 1.0));
            }

            let note = channel.note();
            if app.gamepad.is_just_pressed(Button::Up) {
                channel.set_note(4, note + 1, false);
                channel.set_volume(1.0);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(4, note - 1, false);
                channel.set_volume(1.0);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
        }

        let mut audio_input = app.audio_device.lock();
        let sample_count = audio_input.frames_available();
        for sample in chip.iter(sample_count) {
            audio_input.push_sample(StereoFrame {
                left: sample.left,
                right: sample.right,
            });
            writer
                .write_sample(i16::from(sample.left))
                .map_err(|e| e.to_string())?;
            writer
                .write_sample(i16::from(sample.right))
                .map_err(|e| e.to_string())?;
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}
