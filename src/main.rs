use std::error::Error;

use cpal::traits::{ DeviceTrait, HostTrait };

mod app;
mod atomic_float;
mod key_piano;
mod note_stream;
mod settings;
mod synth;
mod waves;
mod modulator;
mod modulators;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("no device found")?;
    let cfg = device.default_output_config()?;
    app::run(
        synth::Synth::new(
            &device,
            &cfg,
            modulators::combined::from(
                combine_modulators![modulators::bend::new(), modulators::tremolo::new()]
            )
        )
    )?;

    Ok(())
}
