**_UPDATE_**:
Braking changes to Channel, all public sound properties now under a single "Sound" struct.
_Pardon the mess, this is a work-in-progress. API changes are still frequent._

The SoundChip struct contains multiple channels, each with configurable settings that can replicate old audio chips like PSGs and simple wave tables. It is **_not_** an emulator, instead it allows you to customize the sound properties of any sound channel to mimic an old sound chip.

For instance, if you're simulating a classic PSG like the AY-3-8910, the SpecsChip struct may look like this:

```rust
use soundchip::{prelude::*, presets::*};
let msx_spec = SpecsChip {
    // MSX applications usually processed the audio envelopes once per video frame.
    envelope_rate: Some(60.0),
    wavetable: SpecsWavetable {
        // Default PSG wavetable envelope can be anything as long as the first half
        // is positive and second half is negative (see "steps" below).
        default_waveform: Some(KNOTS_WAVE_SQUARE),
        // Square wave (two steps, sample output is always -1.0 or 1.0).
        steps: Some(2),
        // 8 samples would also allow "duty cycle" for the square wave,
        // even though this PSG didn't support that.
        sample_count: 8,
        // Ignored for now, the entire wave always loops.
        // May change in the future to allow playing sampled sounds.
        use_loop: true,
    },
    // "Some(0)" forces the quantization to always zero (mono).
    // "None" would mean "no quantization".
    pan: SpecsPan {
        steps: Some(0),
    },
    // Just an approximation, 4096 pitch steps in 10 octaves.
    pitch: SpecsPitch {
        multiplier: 1.0,
        range: Some(16.35 ..= 16744.04),
        steps: Some(4096),
    },
    volume: SpecsVolume {
        // Quantized to 16 volume levels. Also affects volume envelope.
        steps: Some(16),
        // Volume declines on every sample, and resets when the wavetable changes value.
        attenuation: 0.0017,
        // Non-linear volume envelope. Use 1.0 for linear.
        exponent: 3.0,
        // Some chips may need custom volume gain to sound more accurate.
        gain: 1.0,
        // Clamps the generated wave into 0.0 to 1.0 values.
        clip_negative_values: true,
    },
    // Noise settings.
    noise: SpecsNoise::Random {
        // 2 steps means a square wave (1 bit noise).
        volume_steps: 2,
        // "Maps" a C3 to G#5 range to a much higher noise frequency,
        pitch: SpecsPitch {
            multiplier: 55.0,
            steps: Some(32),
            range: Some(130.81 ..= 783.99), // C3 to G#5
        },
    },
};
```

You can use [SoundChip::iter()] to obtain individual samples, which can be pushed to your audio playback library of choice. The included example uses mini_sdl, which in turn uses SDL2's audio callback feature.

Once you start a channel it will continuously generate a sound with the current settings like pitch, volume and pan. The resulting waveform is then quantized to the chip's specs. Internally it always uses a wavetable - a simple Vec of f32 values - even for a PSG chip, but the quantization steps do a good job of making it sound right.

### Design

While the goal is to sound as close as possible to real hardware, this approach will not recreate every tiny quirk. It will, however, sound very convincing while being more flexible. You can remove limitations and use it like a more modern, "tracker-like" wavetable library. Isn't it funny how I'm calling Trackers modern?

Another interesting feature of this design choice is that a single SoundChip can contain channels with entirely different chip specs, as you can see in the [SoundChip::new_msx_scc()] function, which returns a mix of three PSG channels and five SCC channels, which had a small 32 byte wavetable each.

The API is designed to feel simple and modern, and uses functions like [Channel::set_note()] to make it easy to do things like setting the channel pitch instead of directly manipulating the chip's internals.

It doesn't require the standard library, but it still requires allocation to use Vecs which means it may not be used in some strict, bare metal cases. This requirement may be removed in the future, making it more strictly "no_std".
