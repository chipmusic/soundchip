# TO DO:

[x] Optimizations
    [x] Optional envelope processing rate: will move some of the calculations out of the "hot" sample function, calculations that will only be performed tipically at 60 Hz like in early 1980's games.

[.] Envelopes
    [ ] Add parameter to "envelope.process()" so that the envelope "knows" if the current sample is a new wavetable cycle, and only changes state on new cycles to avoid curve discontinuity

[ ] Additional channel processing:
    [ ] Vibratto (pitch)
    [ ] Tremolo (volume)

[ ] Additional presets: NES, PCE

[x] readme.md

[.] Chip specs processing:
    [x] Quantization
    [x] Non linear output (f32.powf(3.0)) for channel volume, but samples should stay linear
    [x] Optional quantization, based on SpecsChip
    [x] Specs should be all that is needed to create desired chip, so they must specify wavetable data
    [x] Noise freq limits need some sort of mapping, i.e. source pitch (min, max) => target pitch (min, max) (achieved via freq range + multiplier).


[x] Output to wav file for debugging purposes (in example, not in library).


# Bare metal goals (not priority)

[x] Convert all f64 to f32. Every new note or sound played usually means resetting time to zero, so it's unlikely it will ever become a precision issue. That said, don't forget to reset the time on every new sound!

[ ] Remove all uses of Vec, use const generics for:
    [ ] Hard limits on wavetable array size.
    [ ] Hard limit of number of channels.

[?] Uf16 Struct to set/get u16 values from an f32 in the 0.0 to 1.0 range.

[?] If16 Struct to set/get i16 values from an f32 in the -1.0 to 1.0 range.
