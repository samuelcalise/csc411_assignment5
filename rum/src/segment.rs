

use std::mem;
use crate::{um_instruction::Instruction};


#[derive(Debug, Clone)]
pub struct Segments {
    free_addresses: Vec<usize>,
    instruc_to_do: Vec<Vec<u32>>
}

impl Segments {

    pub fn new(instructions: &Vec<u32>) -> Segments {
        Segments {
            free_addresses: Vec::new(),
            instruc_to_do: vec![instructions.to_vec()]
        }
    }

    pub fn map_segment(&mut self, size: usize) -> usize {
        let zeros = vec![0_u32; size];
        match self.free_addresses.len(){
            0 => {
                self.instruc_to_do.push(zeros);
                self.instruc_to_do.len() - 1
            }
            _ => {
                let address = self.free_addresses.pop().unwrap();
            let _new_add = mem::replace(self.instruc_to_do.get_mut(address).unwrap(),zeros);
            address
            }
        }
       
    }

    pub fn unmap_segment(&mut self, address: usize) {
        self.free_addresses.push(address);
        let _new_add = mem::replace(self.instruc_to_do.get_mut(address).unwrap(), Vec::new());
    }

    pub fn get(&self, address: usize) -> Option<&Vec<u32>> {
        self.instruc_to_do.get(address)
    }

    pub fn find_instruction(&self, counter: usize) -> Instruction {
        match self.instruc_to_do.get(0) {
            Some(seg) => Instruction::new(seg[counter]),
            None => panic!("No more instructions")
        }
    }

    pub fn set_seg_val(&mut self, address: usize, index: usize, value: u32) {
        let segments = self.instruc_to_do.get_mut(address).unwrap();
        let _new_add = mem::replace(segments.get_mut(index).unwrap(),value);
    }

    pub fn insert_value(&mut self, address: usize) {
        let seg = self.instruc_to_do.get(address).unwrap().clone();
        let _new_add = mem::replace(self.instruc_to_do.get_mut(0).unwrap(),seg);
    }
}