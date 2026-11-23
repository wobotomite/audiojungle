use std::collections::HashMap;
use std::sync::atomic::{ Ordering::Relaxed };

use winit::{ event::ElementState, keyboard::KeyCode };

use crate::modulators::bend;
use crate::{ settings, synth::Synth };
const C_NOTE: u8 = 3;
pub struct KeyPiano {
    octave: i8,
    held_keys: HashMap<KeyCode, usize>,
}

impl Default for KeyPiano {
    fn default() -> Self {
        Self {
            octave: settings::OCTAVE_OFFSET,
            held_keys: HashMap::new(),
        }
    }
}

impl KeyPiano {
    pub fn handle_key(&mut self, key: KeyCode, state: ElementState, repeat: bool, synth: &Synth) {
        if key == KeyCode::Space {
            bend::BENDING.store(state == ElementState::Pressed, Relaxed);
            return;
        }

        if state == ElementState::Released {
            if
                let Some(note_index) = self.held_keys.remove(&key) &&
                !self.held_keys.values().any(|&held| held == note_index)
            {
                synth.set_note_state(note_index, false);
            }
            return;
        }

        if repeat || self.held_keys.contains_key(&key) {
            return;
        }

        let min_octave = settings::OCTAVE_OFFSET - ((settings::OCTAVES / 2) as i8);
        let max_octave = min_octave + (settings::OCTAVES as i8) - 1;
        match key {
            KeyCode::KeyZ => {
                self.octave = (self.octave - 1).max(min_octave);
            }
            KeyCode::KeyX => {
                self.octave = (self.octave + 1).min(max_octave);
            }
            _ => {
                let Some(semitone) = key_semitone(key) else {
                    return;
                };
                let note_index = settings::get_note_index(self.octave, C_NOTE) + semitone;
                if note_index < (settings::MAX_BANK as usize) {
                    self.held_keys.insert(key, note_index);
                    synth.set_note_state(note_index, true);
                }
            }
        }
    }

    pub fn release_all(&mut self, synth: &Synth) {
        bend::BENDING.store(false, Relaxed);
        for (_, note_index) in self.held_keys.drain() {
            synth.set_note_state(note_index, false);
        }
    }
}

fn key_semitone(key: KeyCode) -> Option<usize> {
    Some(match key {
        KeyCode::KeyA => 0,
        KeyCode::KeyW => 1,
        KeyCode::KeyS => 2,
        KeyCode::KeyE => 3,
        KeyCode::KeyD => 4,
        KeyCode::KeyF => 5,
        KeyCode::KeyT => 6,
        KeyCode::KeyG => 7,
        KeyCode::KeyY => 8,
        KeyCode::KeyH => 9,
        KeyCode::KeyU => 10,
        KeyCode::KeyJ => 11,
        KeyCode::KeyK => 12,
        KeyCode::KeyO => 13,
        KeyCode::KeyL => 14,
        KeyCode::KeyP => 15,
        KeyCode::Semicolon => 16,
        KeyCode::Quote => 17,
        _ => {
            return None;
        }
    })
}
