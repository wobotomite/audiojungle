#![allow(dead_code)]

pub const OCTAVES: u8 = 6; // 3 up and down
pub const OCTAVE_OFFSET: i8 = -2; // offsets the 3 up and down by n. example: -2 makes it 1 up and 5 down.
pub const NOTES_PER_OCTAVE: u8 = 12;
pub const MAX_BANK: u8 = OCTAVES * NOTES_PER_OCTAVE;
pub const A_FREQUENCY: f64 = 440_f64;

// defaults: 0,0
pub fn get_note_index(octave: i8, note: u8) -> usize {
    let offset_octaves =
        (octave as i16 - OCTAVE_OFFSET as i16 + OCTAVES as i16 / 2).rem_euclid(OCTAVES as i16);

    (offset_octaves * NOTES_PER_OCTAVE as i16 + note.rem_euclid(NOTES_PER_OCTAVE) as i16) as usize
}

pub fn get_frequency(semitones: f64) -> f64 {
    A_FREQUENCY * (2f64).powf(semitones / (NOTES_PER_OCTAVE as f64))
}
