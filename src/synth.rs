#![allow(dead_code)]

use std::array;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

use cpal::traits::StreamTrait;

use crate::modulator::Modulator;
use crate::note_stream;
use crate::atomic_float::AtomicF64;

use crate::settings;
use crate::atomic_float;

pub struct Note {
    pub phase: atomic_float::AtomicF64,
    pub state: AtomicBool,
    pub velocity: AtomicU16,
}

impl Note {
    pub fn new() -> Self {
        Self {
            velocity: AtomicU16::new(0),
            state: AtomicBool::new(false),
            phase: AtomicF64::new(0.0),
        }
    }
}

pub type AtomicNoteArray = std::sync::Arc<[Note; settings::MAX_BANK as usize]>;

pub struct Synth {
    note_array: AtomicNoteArray,
    stream: cpal::Stream,
}

impl Synth {
    pub fn new(
        device: &cpal::Device,
        config: &cpal::SupportedStreamConfig,
        modulator: impl Modulator
    ) -> Self {
        let note_array: AtomicNoteArray = Arc::new(array::from_fn(|_| Note::new()));
        let stream = note_stream
            ::new(device, config, modulator, note_array.clone())
            .expect("could not load stream");
        stream.play().unwrap();
        Self { stream, note_array }
    }
    pub fn set_note_state(&self, note_index: usize, state: bool) {
        assert!((0..settings::MAX_BANK as usize).contains(&note_index));
        self.note_array[note_index].state.store(state, Relaxed);
    }
}
