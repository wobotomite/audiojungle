#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    pub fn sample(self, phase: f32) -> f32 {
        let t = phase / std::f32::consts::TAU;

        match self {
            Self::Sine => phase.sin(),
            Self::Square => {
                if t < 0.5 { 1.0 } else { -1.0 }
            }
            Self::Sawtooth => 2.0 * t - 1.0,
            Self::Triangle => 1.0 - 4.0 * (t - 0.5).abs(),
        }
    }
}
