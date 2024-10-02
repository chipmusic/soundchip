# TO DO:

[x] Noise envelope.

[ ] Replace all public channel members with "sound" struct, containing them.

[ ] Alternate pitch quantization strategy: pitch divider (will be the main strategy for most chips).

[ ] Wavetable interpolation when copying samples from an array with different length than current specs.

[.] Additional presets: NES, PCE. Needs more research, specailly about pitch.

[x] Chip specs should optionally contain a static reference to a wave envelope (i.e. NES Triangle). Applying the specs automatically loads the correct envelope.

[.] Envelopes
--->[ ] "Step" Knot interpolation.
        Will fix the imprecision that happens attempting a sharp transition in 1/60 seconds, i.e. exactly one envelope sample. Currently we need two knots for that sharp transition and sometimes it "catches", sometimes not. With step interpolation this can be accomplished with a single new knot.
    [ ] Test random access.
    [ ] Private knots. Currently it's too easy to break an envelope by manipulating knots directly.
        [ ] Insert and Remove knot
        [ ] Find index by time. (will be used by peek()).
        [ ] Auto-sort on envelope manipulation?
    [x] Wavetables from Envelopes.
    [x] Change "channel.sample()" so that it "knows" if the current sample is a new wavetable cycle, and only changes envelope state on new cycles to avoid curve discontinuity.
    [x] Eliminate ADSR, move on to Envelopes that can be used anywhere, including for wavetables.
    [x] Envelope "release", allows it to exit loop state past loop_out.
    [x] Loop in, out, release.

[x] Prelude module with all public types, but no secondary modules (like math and rng).

[x] Optimizations
    [x] Optional envelope processing rate: will move some of the calculations out of the "hot" sample function, calculations that will only be performed tipically at 60 Hz like in early 1980's games.

[x] Additional channel processing:
    [x] Vibratto (pitch)
    [x] Tremolo (volume)

[x] readme.md

[.] Chip specs processing:
    [x] Quantization
    [x] Non linear output (f32.powf(3.0)) for channel volume, but samples should stay linear
    [x] Optional quantization, based on SpecsChip
    [x] Specs should be all that is needed to create desired chip, so they must specify wavetable data
    [x] Noise freq limits need some sort of mapping, i.e. source pitch (min, max) => target pitch (min, max) (achieved via freq range + multiplier).

[x] Output to wav file for debugging purposes (in example, not in library).


# Bare metal goals (not priority)

[x] Convert all f64 to f32. Every new note or sound played usually means resetting time to zero, so it's unlikely it will ever become a precision issue. That said, don't forget to reset the time on every new sound! Exception: global "time()" function in SoundChip. It is calculated from a usize value and does not reset per sound, so casting to f32 could be too imprecise.

[?] Remove all uses of Vec, use const generics for:
    [ ] Hard limits on wavetable array size.
    [ ] Hard limit of number of channels.
    [ ] Hard limit on envelope knots.

[x] Slimmer (and stricter) types.
    Ended up not being all that useful, since pitch envelopes actually allow values beyond 1.0 and simply use f32. Used in channel volume envelope and pan, though.
    [x] Normal Struct to set/get u16 values from an f32 in the 0.0 to 1.0 range.
    [x] NormalSigned Struct to set/get i16 values from an f32 in the -1.0 to 1.0 range.
    [x] Needs testing!
    [ ] Maybe an F16 struct with (1-5-10) bits (sign, value, 3 decimals) to replace f32 in most places? It would help to keep a lot of structs smaller, specially important for structs that are "Copy". The API can preserve f32 values for convenience.
