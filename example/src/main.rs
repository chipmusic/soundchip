use hound::{WavSpec, WavWriter};
use mini_sdl::*;
use soundchip::*;
use std::{env::var_os, path::PathBuf};

fn main() -> SdlResult {
    let target_file: PathBuf = var_os("CARGO_MANIFEST_DIR").unwrap().into();
    // I have this path set to a ram disk on my machine,
    // since I'm saving the wave file for debugging purposes only.
    let target_file = target_file.join("target/output.wav");
    println!("Saving wav file to: {:?}", target_file);

    let mut app = App::default()?;
    let mut chip = SoundChip::new(app.audio_mixrate() as u32);
    app.audio_start();

    println!("Use up and down arrows to change octaves.");
    println!("Use left and right arrows to play different notes.");
    println!("Hit return to toggle noise/tone .");
    println!("Channels: {}", chip.channels().len());

    let ch = 0;
    if let Some(channel) = chip.channel(ch) {
        channel.play();
        channel.set_noise(true);
    }

    // Test
    let specs = PitchSpecs {
        multiplier: 1.0,
        steps: Some(32),
        range: Some(2.5 .. 7.5),
    };

    for n in -5 ..= 15 {
        let value = n as f32;
        println!("{:.2} => {:.2}", value, specs.get(value));
    }

    // Writing in mono for simplicity! Ensure no pan is set in the channel!
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

        if let Some(channel) = chip.channel(ch) {
            // Lower the volume on every frame, simulating a 1 second volume envelope.
            // Notice how this envelope is quantized to 16 steps, per chip settings.
            let vol = channel.volume();
            let env_step = app.elapsed_time() as f32;
            channel.set_volume(vol - env_step);
            // Toggle noise
            if app.gamepad.is_just_pressed(Button::Start) {
                if channel.is_noise(){
                    channel.set_noise(false);
                } else {
                    channel.set_noise(true);
                }
                println!("Channel noise: {}", channel.is_noise());
            }
            // Input
            let note = channel.note();
            let octave = channel.octave();
            if app.gamepad.is_just_pressed(Button::Up) {
                channel.set_note(octave + 1, note, false);
                channel.set_volume(1.0);
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(octave - 1, note, false);
                channel.set_volume(1.0);
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Right) {
                channel.set_note(octave, note + 1, false);
                channel.set_volume(1.0);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Left) {
                channel.set_note(octave, note - 1, false);
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
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}
