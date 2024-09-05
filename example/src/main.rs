use hound::{WavSpec, WavWriter};
use std::{path::Path, time::Instant};
use mini_sdl::*;
use soundchip::*;

fn main() -> SdlResult {
    let mut chip = SoundChip::new(44100);
    let mut app = App::new(
        "chip",
        320,
        240,
        Timing::VsyncLimitFPS(60.0),
        Scaling::StretchToWindow,
    )?;
    app.audio_start();

    if let Some(channel) = chip.channel(0) {
        channel.playing = true;
        let len = 32;
        // let wave: Vec<f32> = (0..len)
        //     .map(|i| {
        //         let a = i as f32 / len as f32 * core::f32::consts::TAU;
        //         a.sin()
        //     })
        //     .collect();
        let wave: Vec<f32> = (0..len)
            .map(|i| if i > len / 2 { 1.0 } else { -1.0 })
            .collect();
        channel
            .set_wavetable(wave.as_slice())
            .map_err(|e| e.to_string())?;
    }

    // let wav_spec = WavSpec {
    //     channels: 2,
    //     sample_rate: chip.output_mix_rate,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };
    // let mut writer =
    //     WavWriter::create(Path::new("output.wav"), wav_spec).map_err(|e| e.to_string())?;

    let mut play_note_time = Instant::now();
    while !app.quit_requested {
        app.frame_start()?;

        // Use Key arrows to pitch note up or down
        if let Some(channel) = chip.channel(0) {
            if play_note_time.elapsed().as_secs_f32() > 0.5 {
                channel.playing = false;
            }

            let note = channel.note();
            if app.gamepad.is_just_pressed(Button::Up) {
                channel.set_note(4, note + 1);
                // channel.set_note(4, Note::A);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
                play_note_time = Instant::now();
                channel.playing = true;
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(4, note - 1);
                // channel.set_note(3, Note::C);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
                play_note_time = Instant::now();
                channel.playing = true;
            }
        }

        let mut audio_input = app.audio_device.lock();
        let sample_count = audio_input.frames_available();
        for sample in chip.iter(sample_count) {
            audio_input.push_sample(StereoFrame {
                left: sample.left,
                right: sample.right,
            });
            // writer
            //     .write_sample(i16::from(sample.left))
            //     .map_err(|e| e.to_string())?;
            // writer
            //     .write_sample(i16::from(sample.right))
            //     .map_err(|e| e.to_string())?;
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    // writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}
