use std::sync::atomic::{ AtomicBool, Ordering::Relaxed };

use crate::modulator::Modulator;

const BEND_SEMITONES: f64 = 3.0;
const BEND_FADE_SECONDS: f64 = 0.1;
pub static BENDING: AtomicBool = AtomicBool::new(false);

impl Modulator for BendModulator {
    // fn begin_frame(&mut self, sample_rate: f64) {
    //     let target = if BENDING.load(Relaxed) { 1.0 } else { 0.0 };
    //     if self.bend == target {
    //         return;
    //     }

    //     let step = 1.0 / (sample_rate * BEND_FADE_SECONDS);
    //     self.bend += (target - self.bend).clamp(-step, step);
    //     self.phase_scale = (2f64).powf(
    //         (BEND_SEMITONES * self.bend) / (settings::NOTES_PER_OCTAVE as f64)
    //     );
    // }

    // fn sample(&self, phase: &AtomicF64, phase_step: f64, velocity: f32) -> f32 {
    //     let phase = phase.fetch_increment_mod_of_tau(phase_step * self.phase_scale, Relaxed);
    //     Waveform::Sine.sample(phase as f32) * velocity * 0.2f32
    // }

    fn sample(
        &mut self,
        cfg: &cpal::SupportedStreamConfig,
        note: &crate::synth::Note,
        semitones: f32,
        velocity: f32
    ) -> f32 {
        let target = if BENDING.load(Relaxed) { 1.0 * (BEND_SEMITONES as f32) } else { 0.0 };
        if self.pitch_shift != target {
            let step = (1.0 / ((cfg.sample_rate() as f64) * BEND_FADE_SECONDS)) as f32;
            self.pitch_shift += (target - self.pitch_shift).clamp(-step, step);
        }

        // self.counter += 1;
        // self.counter %= u32::MAX;

        // let tremolo_effect = ((self.counter as f32) / 4096.0).sin();

        crate::modulator::default_sample(
            cfg,
            note,
            semitones + self.pitch_shift /* +tremolo_effect */,
            velocity
        )
    }
}

pub struct BendModulator {
    pitch_shift: f32,
    //  counter: u32,
}

pub fn new() -> impl Modulator {
    BendModulator { pitch_shift: 0.0 /*counter: 0*/ }
}
