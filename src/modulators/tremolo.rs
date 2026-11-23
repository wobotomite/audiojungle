use crate::modulator::Modulator;

impl Modulator for BendModulator {
    fn sample(
        &mut self,
        cfg: &cpal::SupportedStreamConfig,
        note: &crate::synth::Note,
        semitones: f32,
        velocity: f32
    ) -> f32 {
        self.counter += 1;
        self.counter %= u32::MAX;

        let tremolo_effect = ((self.counter as f32) / 4096.0).sin();

        crate::modulator::default_sample(cfg, note, semitones + tremolo_effect, velocity)
    }
}

pub struct BendModulator {
    counter: u32,
}

pub fn new() -> impl Modulator {
    BendModulator { counter: 0 }
}
