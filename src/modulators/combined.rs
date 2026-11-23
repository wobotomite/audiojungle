use crate::modulator::Modulator;

struct CombinedModulator {
    modules: Vec<Box<dyn Modulator>>,
}

impl Modulator for CombinedModulator {
    fn sample(
        &mut self,
        cfg: &cpal::SupportedStreamConfig,
        note: &crate::synth::Note,
        semitones: f32,
        velocity: f32
    ) -> f32 {
        self.modules
            .iter_mut()
            .fold(0.0, |acc, val| { acc + val.sample(cfg, note, semitones, velocity) })
    }
}

pub fn from(modulators: Vec<Box<dyn Modulator>>) -> impl Modulator {
    CombinedModulator { modules: modulators }
}

#[macro_export]
macro_rules! combine_modulators {
    ($($x:expr),+ $(,)?) => {
        vec![$(Box::new($x)),+]
    };
}
