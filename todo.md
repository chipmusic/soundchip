# TO DO:

[.] Envelopes
    [ ] Add parameter to "envelope.process()" so that the envelope "knows" if the current sample is a new wavetable cycle, and only changes state on new cycles to avoid curve discontinuity

[ ] Additional channel processing:
    [ ] Vibratto (pitch)
    [ ] Tremolo (volume)

[ ] Additional presets: NES, PCEngine

[x] readme.md

[.] Chip specs processing:
    [x] Quantization
    [x] Non linear output (f32.powf(3.0)) for channel volume, but samples should stay linear
    [x] Optional quantization, based on ChipSpecs
    [x] Specs should be all that is needed to create desired chip, so they must specify wavetable data
    [x] Noise freq limits need some sort of mapping, i.e. source pitch (min, max) => target pitch (min, max) (achieved via freq range + multiplier).


[x] Output to wav file for debugging purposes (in example, not in library).
