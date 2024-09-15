

***UPDATE***:
Noise quantizing and clamping seems to work now.
*Pardon the mess, this is a work-in-progress. API changes are still frequent.*

Soundchip doesn't require the standard library, but it still requires allocation to use Vecs which means it may not be used in some strict, bare metal cases. This requirement may be removed in the future, making it more strictly "no_std".

Soundchip is *not* an emulator, it simply allows you to customize the sound properties of any sound channel to mimic an old sound chip. For instance, if you're simulating a classic PSG like the AY-3-8910, the ChipSpecs struct may look like this:

```rust
let msx_spec = ChipSpecs {
    wavetable: WavetableSpecs {
        // Two steps, sample is either -1.0 or 1.0.
        steps: Some(2),
        // 8 samples would also allow "duty cycle" for the square wave
        sample_count: 8,
        // Wave is always assumed to loop.
        // May change in the future to allow playing sampled sounds.
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
        // 16 volume envelope levels.
        steps: Some(16),
        // Volume declines on every sample, until wavetable changes value.
        attenuation: 0.0017,
        // Non-linear volume envelope. Use 1.0 for linear.
        exponent: 3.0,
        // Some chips may need custom gain to sound more accurate.
        gain: 1.0,
        // Fits the generated wave into 0.0 to 1.0 values.
        prevent_negative_values: true,
    },
    // Noise settings.
    noise: NoiseSpecs::Random {
        // 2 steps means a square wave (1 bit noise).
        volume_steps: 2,
        // "Maps" a C3 to G#5 range to a much higher noise frequency,
        pitch: PitchSpecs {
            multiplier: 55.0,
            steps: Some(32),
            range: Some(130.81 ..= 783.99),
        },
    },
};
```

You can use [SoundChip::iter()] to obtain individual samples, which can be pushed to your audio playback library of choice. The included example uses mini_sdl, which in turn uses SDL2's audio callback feature.

Once you start a channel it will continuously generate a sound with the current settings like pitch, volume and pan. The resulting waveform is then quantized to the chip's specs. Internally it always uses a wavetable - a simple Vec of f32 values - even for a PSG chip, but the quantization steps do a good job of making it sound right.

While the goal is to sound as close as possible to real hardware, this approach will not recreate every tiny quirk. It will, however, sound very close while being more flexible.

Another interesting feature of this design choice is that a single SoundChip can contain channels with entirely different chip specs, as you can see in the [SoundChip::new_msx_scc()] function, which returns a mix of three PSG channels and five SCC channels, which had a small 32 byte wavetable each.

The API is designed to feel simple and modern, and uses functions like [Channel::set_note()] to make it easy to do things like setting the channel pitch instead of directly manipulating the chip's internals.
