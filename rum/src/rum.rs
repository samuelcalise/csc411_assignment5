use std::io::{stdin, Read};
use crate::{register::Register, segment::Segment, um_instruction::Instruction};



pub struct Rum{

    segment: Segment,
    register: Register
}


impl Rum{

    pub fn new (some_instruction: &Vec<u32>) -> Rum
    {
        Rum{

            segment: Segment::new(&some_instruction),
            register: Register::new()
        }
    }

    pub fn get_instruction(&self, c: usize) -> Instruction
    {
        self.segment.find_instruction(c)
    }

    pub fn conditional_move(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        if self.register.get_registerValue(c_bit) != 0{
            
            let value = self.register.get_registerValue(b_bit);

            self.register.set_registerValue(a_bit, value);
        }
    }

    pub fn segment_load(){}

    pub fn segment_store(){}

    pub fn addition(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = self.register.get_registerValue(b_bit).wrapping_add(self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn multiplication(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = self.register.get_registerValue(b_bit).wrapping_mul(self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn division(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap();

        let c_bit = some_instruction.c.unwrap();

        let value = self.register.get_registerValue(b_bit as usize).wrapping_div(self.register.get_registerValue(c_bit as usize));

        self.register.get_registerValue(a_bit, value as u32);
    }

    pub fn bit_NAND(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = !(self.register.get_registerValue(b_bit) & self.register.get_registerValue(c_bit));

        self.register.set_registerValue(a_bit, value);
    }

    pub fn map_segment(&mut self, some_instruction: Instruction)
    {
        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let new_size = self.register.get_registerValue(c_bit) as usize;

        let new_address =  self.segment.map_segment(new_size);

        self.register.set_registerValue(b_bit, new_address as u32);
    }

    pub fn unmap_segment(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        let this_address = self.register.get_registerValue(c_bit) as usize;

        self.segment.unmap_segment(this_address);
    }

    pub fn output(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        let c_value = self.register.get_registerValue(c_bit);

        if c_value > 255{
            panic!("The value is outside of [0-255]")
        }

        print!("{}", char::from_u32(c_value).unwrap());
    }

    pub fn user_input(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        if let Some(Ok(value)) = stdin().bytes().next()
        {
            if value as char == '\n'
            {
                self.register.set_registerValue(c_bit, std::u32::MAX);
            }
            else
            {
                self.register.set_registerValue(c_bit, value as u32);
            }
        }
        else
        {
            panic!("Error: Invalid Input");
        }
    }

    pub fn load_program(&mut self, some_instruction: Instruction) -> usize
    {
        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        if self.register.get_registerValue(b_bit) != 0
        {
            self.segment.insert_value(self.register.get_registerValue(b_bit) as usize);
        }

        self.register.get_registerValue(c_bit) as usize
    }

    pub fn load_value(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let value = some_instruction.value.unwrap();

        self.register.set_registerValue(a_bit, value);
    }
}