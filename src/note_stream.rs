pub struct NoteStream {}

use std::sync::atomic::Ordering::Relaxed;

use cpal::{ Error, FromSample, SizedSample, traits::DeviceTrait };

use crate::{ modulator::Modulator, settings::MAX_BANK, synth::{ self } };

pub fn new<M: Modulator>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    mut modulator: M,
    note_array: synth::AtomicNoteArray
) -> Result<cpal::Stream, Error> {
    // xd
    macro_rules! match_sample_format {
        ($($variant:ident => $t:ty),+ $(,)?) => {
                match config.sample_format() {
                    $(
                        cpal::SampleFormat::$variant => build_stream::<$t, M>(device, config, modulator, note_array.clone()),
                    )+
                    any => panic!("Unsupported sample format: {any:?}"),
                }
        };
    }

    match_sample_format! {
            F32 => f32,
            F64 => f64,
            I8  => i8,
            I16 => i16,
            I32 => i32,
            I64 => i64,
            U8  => u8,
            U16 => u16,
            U32 => u32,
            U64 => u64,
        }
}

fn build_stream<T: FromSample<f32> + SizedSample, M: Modulator>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    mut modulator: M,
    note_array: synth::AtomicNoteArray
) -> Result<cpal::Stream, Error> {
    let channels = config.channels() as usize;
    let sample_rate = config.sample_rate() as f64;

    let attack_step: u16 = ((1.0 / (sample_rate * 0.1)) * (u16::MAX as f64)) as u16;
    let release_step: u16 = ((1.0 / (sample_rate * 0.3)) * (u16::MAX as f64)) as u16;

    let midpoint = MAX_BANK / 2;
    let callback_config = config.clone();
    let closure = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        for chunk in data.chunks_mut(channels) {
            // modulator.begin_frame(sample_rate);
            //
            let mut mix = 0f32;
            for note_index in 0..MAX_BANK {
                let note_data = &note_array[note_index as usize];
                let pressed = note_data.state.load(Relaxed);
                let velocity = note_data.velocity.update(Relaxed, Relaxed, |f| {
                    if pressed {
                        f.saturating_add(attack_step)
                    } else {
                        f.saturating_sub(release_step)
                    }
                });
                if velocity == 0 {
                    continue;
                }

                let semitones = (note_index as i32) - (midpoint as i32);

                mix += modulator.sample(
                    &callback_config,
                    &note_data,
                    semitones as f32,
                    (velocity as f32) / (u16::MAX as f32)
                );
            }
            //
            chunk.fill(T::from_sample(mix));
        }
    };

    device.build_output_stream(config.config(), closure, |_| panic!("epic fail"), None)
}
