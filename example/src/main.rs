mod wave_writer;

use mini_sdl::*;
use soundchip::{math::*, prelude::*, presets::*};

fn main() -> SdlResult {
    println!("Use up and down arrows to change octaves.");
    println!("Use left and right arrows to play different notes.");
    println!("Hold the arrow keys to sustain, let them go to release.");
    println!("Hit return to toggle noise/tone .");

    // Set up main structs
    let mut app = App::default()?;
    app.audio_start();
    let mix_rate = app.audio_mixrate();
    let mut chip = SoundChip::new(mix_rate);

    // Define sound
    let sound = Sound {
        volume: 1.0,
        pitch: Note::C.frequency(4),
        waveform: Some(Envelope::from(KNOTS_WAVE_SAWTOOTH)),
        noise_env: None,
        // noise_env: Some(
        //     Envelope::from(KNOTS_VOL_DOWN).scale_time(1.0/30.0)
        // ),
        volume_env: Some(
            Envelope::from(KNOTS_VOL_DOWN).set_loop(LoopKind::LoopPoints {
                loop_in: 1,
                loop_out: 1,
            }).scale_time(0.5).echo(0.5.into()),
        ),
        pitch_env: None,
        tremolo: Some(TREMOLO_SUBTLE),
        vibratto: Some(VIBRATTO_SUBTLE),
    };

    // Add and configure channel with custom specs, start playback.
    let ch = 0;
    chip.channels.push(Channel::from(SPEC_CHIP_PCE));
    if let Some(channel) = chip.channels.get_mut(ch) {
        channel.set_sound(&sound);
        // channel.set_noise(true);
        channel.play();
        channel.release();
    }

    // Set up audio file writing for debugging, check "wave_writer" mod.
    let mut wav_file = wave_writer::WaveWriter::new(app.audio_mixrate());

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
                channel.reset();
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(octave - 1, note);
                channel.reset();
                println!("Octave:{}", channel.octave());
            }
            if app.gamepad.is_just_pressed(Button::Right) {
                channel.set_midi_note(midi_note + 1.0);
                channel.reset();
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Left) {
                channel.set_midi_note(midi_note - 1.0);
                channel.reset();
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            // Release envelopes when arrow buttons are lifted
            if app.gamepad.is_just_released(Button::Up) {
                channel.release();
            }
            if app.gamepad.is_just_released(Button::Down) {
                channel.release();
            }
            if app.gamepad.is_just_released(Button::Right) {
                channel.release();
            }
            if app.gamepad.is_just_released(Button::Left) {
                channel.release();
            }
        }

        // Write audio samples to mini_sdl & wave file.
        let mut audio_input = app.audio_device.lock();
        let sample_count = audio_input.frames_available();
        for sample in chip.iter(sample_count) {
            audio_input.push_sample(StereoFrame {
                left: sample.left,
                right: sample.right,
            });
            wav_file.write(sample)?;
        }

        drop(audio_input);
        app.frame_finish()?;
    }

    wav_file.save_file()?;
    Ok(())
}
