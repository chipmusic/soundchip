use mini_sdl::*;
use soundchip::*;

fn main() -> SdlResult {
    let mut chip = SoundChip::new(44100);
    let mut app = App::new(
        "chip",
        320,
        240,
        Timing::VsyncLimitFPS(120.0),
        Scaling::StretchToWindow,
    )?;
    app.audio_start();

    let channel = chip.channel(0).unwrap();
    channel.muted = false;

    while !app.quit_requested {
        app.frame_start()?;

        // Use Key arrows to pitch note up or down
        if let Some(channel) = chip.channel(0) {
            let note = channel.note();
            if app.gamepad.is_just_pressed(Button::Up) {
                channel.set_note(4, note + 1);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
            if app.gamepad.is_just_pressed(Button::Down) {
                channel.set_note(4, note - 1);
                println!("Octave:{}, note:{}", channel.octave(), channel.note());
            }
        }

        let elapsed = app.elapsed_time_raw();

        let mut audio_input = app.audio_device.lock();
        let sample_count = audio_input.frames_available(elapsed);
        for sample in chip.iter(sample_count) {
            audio_input.push_sample(StereoFrame {
                left: sample.left,
                right: sample.right,
            });
        }
        drop(audio_input);

        app.frame_finish()?;
    }

    Ok(())
}
