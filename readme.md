

***UPDATE***:
Noise quantizing and clamping seems to work now.
*Pardon the mess, this is a work-in-progress. API changes are still frequent.*

Soundchip doesn't require the standard library, but it still requires allocation to use Vecs which means it may not be used in some strict, bare metal cases. This requirement may be removed in the future, making it more strictly "no_std".

Notice that soundchip is *not* an emulator, it simply allows you to customize the sound properties of any sound channel to sound like an old sound chip. For instance, if you're simulating a classic PSG like the AY-3-8910, the ChipSpecs struct looks like this:

```rust
ChipSpec {
    sample_steps: 1,                // Square wave only, sample is either -1.0 or 1.0.
    volume_steps: 16,               // 4 bit volume register allows 16 volume levels.
    pan_steps: 0,                   // No stereo (quantized value is always zero).
    pitch_steps: 32,                // Just an approximation, 4096 pitch steps in 10 octaves.
    volume_attenuation: 0.0017,     // Volume declines until internal wavetable changes value.
    volume_exponent: 3.0,           // Non-linear volume envelope.
    volume_gain: 1.0,               // Some chips may need custom gain to sound more accurate.
    prevent_negative_values: true,  // Fits the generated wave into 0.0 to 1.0 values.
    noise: NoiseSpecs::Random {     // Noise settings.
        volume_steps: 1,            // 1 Means a square wave (1 bit noise).
        pitch: PitchSpecs {
            // "Maps" the C3 to G#5 range to a much higher noise frequency,
            multiplier: 55.0,
            steps: Some(32),
            range: Some(130.81 ..= 783.99),
        },
    }
}
```

You can use [SoundChip::iter()] to obtain individual samples, which can be pushed to your audio library of choice. The included example uses mini_sdl, which uses SDL2's audio callback feature.

Once you start a channel it will continuously generate a sound with the current settings like pitch, volume and pan. The resulting waveform is then quantized to the chip's specs. Internally it always uses a wavetable - a simple Vec of f32 values - even for a PSG chip, but the quantization steps do a good job of making it sound right.

While the goal is to sound as close as possible to real hardware, this approach will not recreate every tiny quirk. It will, however, sound "close enough" while retaining more flexibility which can make it more useful for modern games that want to recreate the classic feel of old games. For instance, every preset sound chip can still play in stereo if you set the pan value, even though the original chips couldn't do that. It's up to you to enforce some limitations.

Another interesting feature of this design choice is that a single SoundChip can contain channels with entirely different chip specs, as you can see in the [SoundChip::new_msx_scc()] function, which returns a mix of three PSG channels and five SCC channels, which had a small 32 byte wavetable each.

The API is designed to feel simple and modern, and uses functions like [Channel::set_note()] to make it easy to do things like setting the channel pitch instead of directly manipulating the chip's internals.
