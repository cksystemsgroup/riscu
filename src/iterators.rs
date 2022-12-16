use byteorder::{ByteOrder, LittleEndian};

use crate::{decode, instruction_length, Instruction};

/// An iterator for all PC values where an instruction begins.
pub struct LocationIter<'a> {
    memory_view: &'a [u8],
    current_index: u64,
    address: u64,
}

impl LocationIter<'_> {
    pub fn new(memory_view: &[u8], address: u64) -> LocationIter<'_> {
        LocationIter {
            memory_view,
            current_index: 0,
            address,
        }
    }

    fn current_hword(&self) -> u16 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 2];
        LittleEndian::read_u16(begin)
    }
}

impl Iterator for LocationIter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current_index as usize) >= self.memory_view.len() {
            // Ensure that we never go out of bound.
            return None;
        }

        match instruction_length(self.current_hword()) {
            2 => {
                self.current_index += 2;
                Some(self.address + self.current_index - 2)
            }
            4 => {
                if (self.current_index as usize) <= self.memory_view.len() - 4 {
                    self.current_index += 4;
                    Some(self.address + self.current_index - 4)
                } else {
                    // There should be a 4 byte instruction but the end of the
                    // slice is less than 4 byte long!
                    panic!("Unaligned instruction!");
                }
            }
            l => panic!("Unimplemented instruction length: {}", l),
        }
    }
}

/// An iterator for all instructions in the program.
pub struct InstructionIter<'a> {
    memory_view: &'a [u8],
    current_index: u64,
}

impl InstructionIter<'_> {
    pub fn new(memory_view: &[u8]) -> InstructionIter<'_> {
        InstructionIter {
            memory_view,
            current_index: 0,
        }
    }

    fn current_hword(&self) -> u16 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 2];

        LittleEndian::read_u16(begin)
    }

    fn fetch_hword(&mut self) -> u16 {
        let half_word = self.current_hword();

        self.current_index += 2;

        half_word
    }

    fn fetch_word(&mut self) -> u32 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 4];

        self.current_index += 4;

        LittleEndian::read_u32(begin)
    }
}

impl Iterator for InstructionIter<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.memory_view.len() as u64 {
            return None;
        }

        Some(
            decode(match instruction_length(self.current_hword()) {
                2 => self.fetch_hword().into(),
                4 => self.fetch_word(),
                l => panic!("Unimplemented instruction length: {}", l),
            })
            .expect("valid instruction"),
        )
    }
}
