use hound::{WavSpec, WavWriter};
use mini_sdl::*;
use soundchip::{math::*, prelude::*, presets::*};
use std::{env::var_os, path::PathBuf};

fn main() -> SdlResult {
    let mut app = App::default()?;
    app.audio_start();

    let mix_rate = app.audio_mixrate();
    let mut chip = SoundChip::new(mix_rate);

    println!("Use up and down arrows to change octaves.");
    println!("Use left and right arrows to play different notes.");
    println!("Hit return to toggle noise/tone .");

    // Add and configure channel with custom specs (PSG wave, TIA-like noise)
    let ch = 0;
    chip.channels.push(Channel::default());
    if let Some(channel) = chip.channels.get_mut(ch) {
        channel.set_specs(SpecsChip {
            wavetable: SpecsWavetable::psg(),
            pan: SpecsPan::psg(),
            pitch: SpecsPitch::psg(),
            volume: SpecsVolume::default(),
            noise: SpecsNoise::default(),
        });
        // channel.envelope_rate = None;
        channel.volume_env = Some(ENVELOPE_LINEAR.scale_time(2.0));
        channel.tremolo = Some(TREMOLO_SUBTLE);
        channel.vibratto = Some(VIBRATTO_SUBTLE);
        // channel.pitch_env = Some(
        //     ENVELOPE_LINEAR
        //         .scale_time(2.0)
        //         .offset(-1.0)           // Offset before scaling to fit values in 0 to -1
        //         .scale_values(2.0)      // Scale pushes the max values to -2
        // );
        // println!("{:#?}", channel.pitch_env);
        channel.set_noise(true);
        channel.play();
    }

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
        sample_rate: chip.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = if let Some(target_file) = target_file {
        Some(WavWriter::create(target_file, wav_spec).map_err(|e| e.to_string())?)
    } else {
        None
    };

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
            let midi_note = get_midi_note(octave, note) as f32;
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

        // Write audio samples to mini_sdl
        let mut audio_input = app.audio_device.lock();
        let sample_count = audio_input.frames_available();
        for sample in chip.iter(sample_count) {
            audio_input.push_sample(StereoFrame {
                left: sample.left,
                right: sample.right,
            });
            if let Some(writer) =&mut  writer {
                writer
                    .write_sample(i16::from(sample.left))
                    .map_err(|e| e.to_string())?;
            }
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    // I have this path set to a ram disk on my machine,
    // since I'm saving the wave file for debugging purposes.
    if let Some(writer) = writer {
        writer.finalize().map_err(|e| e.to_string())?;
    }
    Ok(())
}
