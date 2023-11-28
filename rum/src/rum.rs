use std::io::{stdin, Read};
use crate::{registers::Registers, segments::Segments, um_instruction::Instruction};



pub struct Rum{

    segment: Segments,
    register: Registers
}


impl Rum{

    pub fn new (some_instruction: &Vec<u32>) -> Rum
    {
        Rum{

            segment: Segments::new(&some_instruction),
            register: Registers::new()
        }
    }

    pub fn get_instruction(&self, c: usize) -> Instruction
    {
        self.segment.get_instruction(c)
    }

    pub fn conditional_move(&mut self, some_instruction: Instruction)
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        if self.register.get_registerValue(c_bit) != 0{
            
            let value = self.register.get_registerValue(b_bit);

            self.register.set_registerValue(a, value);
        }
    }

    pub fn segment_load(){}

    pub fn segment_store(){}

    pub fn addition(&mut self, instruction: Instruction)
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        let value = self.register.get_registerValue(b).wrapping_add(self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn multiplication()
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        let value = self.register.get_registerValue(b).wrapping_mul(self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn division(&mut self, instruction: Instruction)
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap();

        let c_bit = instruction.c.unwrap();

        let value = self.register.get_registerValue(b_bit).wrapping_div(self.register.get_registerValue(c_bit));

        self.register.get_registerValue(a_bit, value);
    }

    pub fn bit_NAND()
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        let value = !(self.register.get_registerValue(b_bit) & self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn map_segment()
    {
        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        let new_size = self.register.get_registerValue(c_bit) as usize;

        let new_address =  self.segment.map_segment(new_size);

        self.register.set_registerValue(b_bit, new_address as u32);
    }

    pub fn unmap_segment()
    {
        let c_bit = instruction.c.unwrap() as usize;

        let this_address = self.register.get_registerValue(c_bit) as usize;

        self.segment.unmap_segment(this_address);
    }

    pub fn output()
    {
        let c_bit = instruction.c.unwrap() as usize;

        let c_value = self.register.get_registerValue(c_bit);

        if c_value > 255{
            panic!("The value is outside of [0-255]")
        }

        print!("{}", char::from_u32(c_value).unwrap());
    }

    pub fn user_input(){}

    pub fn load_program(){}

    pub fn load_value(){}
}