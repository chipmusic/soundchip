# TO DO:

[ ] Additional presets: NES, PCEngine

[x] readme.md

[.] Chip specs processing:
  [x] Quantization
  [x] Non linear output (f32.powf(3.0)) for channel volume, but samples should stay linear
  [ ] Optional quantization, based on ChipSpecs
  [ ] Specs should be all that is needed to create desired chip, so they must specify wavetable data
  [ ] Noise freq limits need some sort of mapping, i.e. source pitch (min, max) => target pitch (min, max)

[x] Output to wav file for debugging purposes (in example, not in library).
