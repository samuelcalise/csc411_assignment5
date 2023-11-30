use std::io::{stdin, Read};
use crate::{register::Register, segment::Segment, um_instruction::Instruction};
use std::io::stdout;
use std::io::Write;
#[derive(Debug, Clone)]

pub struct Rum {
    segment: Segment,
    register: Register
}

impl Rum {

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

        if self.register.get_register_value(c_bit) != 0{
            
            let value = self.register.get_register_value(b_bit);

            self.register.set_register_value(a_bit, value);
        }
    }

    pub fn segment_load(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let this_address = self.register.get_register_value(b_bit) as usize;

        let vec = self.segment.get_segment_value(this_address).unwrap();

        let reg_index = self.register.get_register_value(c_bit) as usize;

        let value = vec[reg_index];

        self.register.set_register_value(a_bit, value);
    }

    pub fn segment_store(&mut self, instruction: Instruction) 
    {
        let a_bit = instruction.a as usize;

        let b_bit = instruction.b.unwrap() as usize;

        let c_bit = instruction.c.unwrap() as usize;

        let this_address = self.register.get_register_value(a_bit) as usize;

        let index = self.register.get_register_value(b_bit) as usize;

        let value = self.register.get_register_value(c_bit);

        self.segment.set_segment_value(this_address, index, value);
    }

    pub fn addition(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = self.register.get_register_value(b_bit).wrapping_add(self.register.get_register_value(c_bit));

        self.register.set_register_value(a_bit, value);
    }

    pub fn multiplication(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = self.register.get_register_value(b_bit).wrapping_mul(self.register.get_register_value(c_bit));

        self.register.set_register_value(a_bit, value);
    }

    pub fn division(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap();

        let c_bit = some_instruction.c.unwrap();

        let value = self.register.get_register_value(b_bit as usize).wrapping_div(self.register.get_register_value(c_bit as usize)) as u32;

        self.register.set_register_value(a_bit, value);
    }

    pub fn bit_nand(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = !(self.register.get_register_value(b_bit) & self.register.get_register_value(c_bit));

        self.register.set_register_value(a_bit, value);
    }

    pub fn map_segment(&mut self, some_instruction: Instruction)
    {
        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let new_size = self.register.get_register_value(c_bit) as usize;

        let new_address =  self.segment.map_segment(new_size);

        self.register.set_register_value(b_bit, new_address as u32);
    }

    pub fn unmap_segment(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        let this_address = self.register.get_register_value(c_bit) as usize;

        self.segment.unmap_segment(this_address);
    }

    pub fn output_program(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        let c_value = self.register.get_register_value(c_bit);

        if c_value > 255
        {
            panic!("The value is outside of [0-255]")
        }

        print!("{}", char::from_u32(c_value).unwrap());
        //stdout().flush().unwrap();
    }

    pub fn user_input(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        //try to implment match statement when iteratoring through a std input in terminal**
        if let Some(Ok(value)) = stdin().bytes().next()
        {
            
            self.register.set_register_value(c_bit, value as u32);
        }
        else
        {
            self.register.set_register_value(c_bit, std::u32::MAX);
        }
    }

    pub fn load_program(&mut self, some_instruction: Instruction) -> usize
    {
        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        if self.register.get_register_value(b_bit) != 0
        {
            self.segment.insert_value(self.register.get_register_value(b_bit) as usize);
        }

        self.register.get_register_value(c_bit) as usize
    }

    pub fn load_value(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let value = some_instruction.value.unwrap();

        self.register.set_register_value(a_bit, value);
    }
}