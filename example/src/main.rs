use std::{env::var_os, path::PathBuf};
use hound::{WavSpec, WavWriter};
use soundchip::*;
use mini_sdl::*;

fn main() -> SdlResult {
    let target_file: PathBuf = var_os("CARGO_MANIFEST_DIR").unwrap().into();
    // I have this path set to a ram disk on my machine,
    // since I'm saving the wave file for debugging purposes only.
    let target_file = target_file.join("target/output.wav");
    println!("Saving wav file to: {:?}", target_file);

    let mut app = App::default()?;
    let mut chip = SoundChip::new_msx(app.audio_mixrate() as u32);
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

    let msx_spec = ChipSpecs {
        // Square wave only, sample is either -1.0 or 1.0.
        wavetable: WavetableSpecs {
            steps: Some(1),
            sample_count: 8,
            use_loop: true,
        },
        // No stereo (quantized pan value is always zero).
        pan: PanSpecs {
            steps: Some(0),
        },
        // Just an approximation, 4096 pitch steps in 10 octaves.
        pitch: PitchSpecs {
            multiplier: 1.0,
            range: Some(16.35 ..= 16744.04),
            steps: Some(4096),
        },
        volume: VolumeSpecs {
            // 4 bit volume register allows 16 volume levels.
            steps: Some(16),
            // Volume declines until internal wavetable changes value.
            attenuation: 0.0017,
            // Non-linear volume envelope.
            exponent: 3.0,
            // Some chips may need custom gain to sound more accurate.
            gain: 1.0,
            // Fits the generated wave into 0.0 to 1.0 values.
            prevent_negative_values: true,
        },
        // Noise settings.
        noise: NoiseSpecs::Random {
            // 1 Means a square wave (1 bit noise).
            volume_steps: 1,
             // "Maps" a C3 to G#5 range to a much higher noise frequency,
            pitch: PitchSpecs {
                multiplier: 55.0,
                steps: Some(32),
                range: Some(130.81 ..= 783.99),
            },
        },
    };

    // Quantization Test
    // for n in -10 ..= 10 {
    //     let value = n as f32 / 10.0;
    //     let result = soundchip::math::quantize_range_f32(value, 5, -1.0 ..= 1.0);
    //     println!("{:.3} => {:.3}", value, result);
    // }

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
            let octave = channel.octave();
            let note = channel.note();
            let midi_note = get_midi_note(octave, note) as f32;
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
                channel.set_midi_note(midi_note + 1.0, false);
                channel.set_volume(1.0);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Left) {
                channel.set_midi_note(midi_note - 1.0, false);
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
