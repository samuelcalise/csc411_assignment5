

use std::mem;
use crate::{um_instruction::Instruction};

#[derive(Debug, Clone)]

pub struct Segment {
    addresses: Vec<usize>,
    instructions: Vec<Vec<u32>>
}

impl Segment{

    pub fn new(some_instruction: &Vec<u32>) -> Segment
    {
        Segment{
            addresses: Vec::new(),
            instructions: vec![some_instruction.to_vec()]
        }
    }

    pub fn map_segment(& mut self, size: usize) -> usize
    {
        let zero_vec =  vec![0_u32; size];

        if self.addresses.is_empty()
        {
            self.instructions.push(zero_vec);


            self.instructions.len() - 1
        }
        else
        {
            let this_address = self.addresses.pop().unwrap();
            let _new_address = mem::replace(self.instructions.get_mut(this_address).unwrap(), zero_vec);


            this_address
        }
    }

    pub fn unmap_segment(& mut self, some_address: usize)
    {
        self.addresses.push(some_address);

        let _new_address = mem::replace(self.instructions.get_mut(some_address).unwrap(), Vec::new());
    }

    pub fn get_segmentValue(&self, some_address: usize) -> Option<&Vec<u32>>
    {
        self.instructions.get(some_address)
    }

    pub fn find_instruction(&self, c: usize) -> Instruction
    {
        match self.instructions.get(0){
            Some(segment) => Instruction::new(segment[c]),
            None => panic!("No more further instructions")
        }
    }

    pub fn set_segmentValue(&mut self, some_address: usize, index: usize, value: u32)
    {
        let current_segment = self.instructions.get_mut(some_address).unwrap();

        let _new_segment = mem::replace(current_segment.get_mut(index).unwrap(), value);
    }

    pub fn insert_value(&mut self, some_address: usize)
    {
        let cloned_segment = self.instructions.get(some_address).unwrap().clone();

        let _new_segment = mem::replace(self.instructions.get_mut(0).unwrap(), cloned_segment);
    }
}