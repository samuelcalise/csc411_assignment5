use std::io::{stdin, Read};
use crate::{register::Register, segment::Segments, um_instruction::Instruction};

#[derive(Debug, Clone)]

pub struct Rum {
    segment: Segments,
    register: Register
}

impl Rum {

    pub fn new(instructions: &Vec<u32>) -> Rum {
        Rum {
            segment: Segments::new(&instructions),
            register: Register::new()
        }
    }

    pub fn get_instruction(&self, counter: usize) -> Instruction {
        self.segment.get_instruction(counter)
    }

    pub fn cmov(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        if self.register.get_reg_val(c) != 0 {
            let value = self.register.get_reg_val(b);
            self.register.set_reg_val(a, value);
        }
    }

    pub fn load(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.register.get_reg_val(b) as usize;
        let array = self.segment.get(address).unwrap();
        let index = self.register.get_reg_val(c) as usize;
        let value = array[index];

        self.register.set_reg_val(a, value);
    }

    pub fn store(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.register.get_reg_val(a) as usize;
        let index = self.register.get_reg_val(b) as usize;
        let value = self.register.get_reg_val(c);

        self.segment.set_seg_val(address, index, value);
    }

    pub fn add(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.register.get_reg_val(b).wrapping_add(self.register.get_reg_val(c));

        self.register.set_reg_val(a, value);
    }

    pub fn mul(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.register.get_reg_val(b).wrapping_mul(self.register.get_reg_val(c));

        self.register.set_reg_val(a, value);
    }

    pub fn div(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.register.get_reg_val(b).wrapping_div(self.register.get_reg_val(c));
        self.register.set_reg_val(a, value);
    }

    pub fn nand(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = !(self.register.get_reg_val(b) & self.register.get_reg_val(c));

        self.register.set_reg_val(a, value);
    }

    pub fn mapsegment(&mut self, instruction: Instruction) {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let size = self.register.get_reg_val(c) as usize;
        let address = self.segment.mapsegment(size);

        self.register.set_reg_val(b, address as u32);
    }

    pub fn unmapsegment(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;
        let address = self.register.get_reg_val(c) as usize;

        self.segment.unmapsegment(address);
    }

    pub fn output(&self, instruction: Instruction) {
        
        let c = instruction.c.unwrap() as usize;
        let value = self.register.get_reg_val(c);
        
        if value > 255{
            panic!("Value not within 0 and 255");
        }
        print!("{}", char::from_u32(value).unwrap());
    }

    pub fn input(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;

        match stdin().bytes().next().unwrap() { 
            Ok(value) if value as char == '\n' => self.register.set_reg_val(c, std::u32::MAX),
            Ok(value) if value as char != '\n' => self.register.set_reg_val(c, value as u32),
            Ok(_) => panic!("Error in input"),
            Err(_) => panic!("Error in input")
        }
    }

    pub fn loadprogram(&mut self, instruction: Instruction) -> usize {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        if self.register.get_reg_val(b) != 0 {
            self.segment.load(self.register.get_reg_val(b) as usize);
        }

        self.register.get_reg_val(c) as usize
    }

    pub fn loadvalue(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let value = instruction.value.unwrap();

        self.register.set_reg_val(a, value);
    }
}