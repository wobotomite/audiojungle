use std::sync::atomic::Ordering::Relaxed;

use cpal::SupportedStreamConfig;

use crate::{ settings::get_frequency, synth::Note, waves };

#[allow(unused)]
pub trait Modulator: Send + 'static {
    fn sample(
        &mut self,
        cfg: &SupportedStreamConfig,
        note: &Note,
        semitones: f32,
        velocity: f32
    ) -> f32 {
        default_sample(cfg, note, semitones, velocity)
    }
}

pub fn default_sample(
    cfg: &SupportedStreamConfig,
    note: &Note,
    semitones: f32,
    velocity: f32
) -> f32 {
    let freq = get_frequency(semitones as f64);
    let phase_step: f64 = ((std::f32::consts::TAU as f64) * freq) / (cfg.sample_rate() as f64);
    let phase_result = note.phase.fetch_increment_mod_of_tau(phase_step, Relaxed) as f32;
    waves::Waveform::Sawtooth.sample(phase_result) * velocity * 0.2 // last is volume
}

struct DefaultModulator {}
impl Modulator for DefaultModulator {}

pub fn default() -> impl Modulator {
    DefaultModulator {}
}
