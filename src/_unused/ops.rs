use crate::presets::*;

pub enum ChipOp {
    PlayAllChannels,
    StopAllChannels,
    EnvelopeLoad{slot:u16, preset:PresetEnv},
    EnvelopeScale{slot:u16, scale:f32},
    EnvelopeOffset{slot:u16, offset:f32},
}

pub enum ChannelOp {
    Select(u16),
    Stop,
    Play,
    Noise,
    Tone,
    Reset,
    ResetTime,
    ResetEnvelope,
    Volume(f32),
    Pan(f32),
    Note(f32),
    LoadVolumeEnvelope(u16),
    LoadPitchEnvelope(u16),
    LoadVibratto(PresetVibratto),
    LoadTremolo(PresetTremolo),
}
