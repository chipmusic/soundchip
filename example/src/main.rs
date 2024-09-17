use presets::{ENV_LINEAR_DECAY, ENV_PIANO};
use std::{env::var_os, path::PathBuf};
use hound::{WavSpec, WavWriter};
use soundchip::*;
use mini_sdl::*;

fn main() -> SdlResult {
    // I have this path set to a ram disk on my machine,
    // since I'm saving the wave file for debugging purposes only.
    let target_file: PathBuf = var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let target_file = target_file.join("target/output.wav");
    println!("Saving wav file to: {:?}", target_file);

    let mut app = App::default()?;
    app.audio_start();

    let mix_rate = app.audio_mixrate();
    let mut chip = SoundChip::new_msx(mix_rate);

    println!("Use up and down arrows to change octaves.");
    println!("Use left and right arrows to play different notes.");
    println!("Hit return to toggle noise/tone .");

    // Add channel
    let ch = 0;
    // chip.channels.push(Channel::default());
    if let Some(channel) = chip.channels.get_mut(ch) {
        channel.volume_env = Some(ENV_PIANO);
        channel.pitch_env = Some(ENV_LINEAR_DECAY.offset(-1.0));
        channel.pitch_env_multiplier = 8.0; //plus or minus 3 full octaves (2 to the power of 3)
        channel.play();
        channel.set_noise(true);
    }

    // Writing in mono for debugging simplicity. Ensure no pan is set in the channel!
    let wav_spec = WavSpec {
        channels: 1,
        sample_rate: chip.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(target_file, wav_spec).map_err(|e| e.to_string())?;

    // mini_sdl main loop. MiniSDL assumes you want both graphics and sound!
    while !app.quit_requested {
        app.frame_start()?;

        if let Some(channel) = chip.channels.get_mut(ch) {
            // Toggle noise
            if app.gamepad.is_just_pressed(Button::Start) {
                let is_noise = channel.is_noise();
                channel.set_noise(!is_noise);
                println!("Channel noise: {}", channel.is_noise());
            }
            // Get current values
            let octave = channel.octave();
            let note = channel.note();
            let midi_note = math::get_midi_note(octave, note) as f32;
            // Play notes, change pitch
            if app.gamepad.is_just_pressed(Button::Up) {
                channel.set_note(octave + 1, note);
                // channel.reset_envelopes();
                channel.reset_time();
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(octave - 1, note);
                // channel.reset_envelopes();
                channel.reset_time();
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Right) {
                channel.set_midi_note(midi_note + 1.0);
                // channel.reset_envelopes();
                channel.reset_time();
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Left) {
                channel.set_midi_note(midi_note - 1.0);
                // channel.reset_envelopes();
                channel.reset_time();
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
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}
