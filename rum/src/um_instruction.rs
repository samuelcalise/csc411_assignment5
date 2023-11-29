use bitpack::bitpack::{getu};

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub value: Option<u32>
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Err
}

pub fn get_opcode(instruction: u64) -> Opcode {
  
    let opcode_num = getu(instruction, 28, 4);

    if opcode_num == 0{
        Opcode::CMov
    }
    else if opcode_num == 1{
        Opcode::Load
    }
    else if opcode_num == 2{
        Opcode::Store
    }
    else if opcode_num == 3{
        Opcode::Add
    }
    else if opcode_num == 4{
        Opcode::Mul
    }
    else if opcode_num == 5{
        Opcode::Div
    }
    else if opcode_num == 6{
        Opcode::Nand
    }
    else if opcode_num == 7{
        Opcode::Halt
    }
    else if opcode_num == 8{
        Opcode::MapSegment
    }
    else if opcode_num == 9{
        Opcode::UnmapSegment
    }
    else if opcode_num == 10{
        Opcode::Output
    }
    else if opcode_num == 11{
        Opcode::Input
    }
    else if opcode_num == 12{
        Opcode::LoadProgram
    }
    else if opcode_num == 13{
        Opcode::LoadValue
    }
    else{
        Opcode::Err
    }
}

pub fn get_a_bit(some_instruction: u64, opcode: &Opcode) -> u32 {
    if *opcode == Opcode::LoadValue{
        return getu(some_instruction, 25, 3) as u32;
    }
    else{
        return getu(some_instruction, 6, 3) as u32;
    }
}

pub fn get_b_bit(some_instruction: u64, opcode: &Opcode) -> Option<u32> {
    if *opcode == Opcode::LoadValue{
        return None;
    }
    else{
        return Some(getu(some_instruction, 3, 3).try_into().unwrap());
    }

}

pub fn get_c_bit(some_instruction: u32, opcode: &Opcode) -> Option<u32> {
    if *opcode == Opcode::LoadValue{
        return None
    }
    else{
        return Some(getu(some_instruction.into(), 0, 3).try_into().unwrap());
    }
    
}

pub fn get_value(some_instruction: u32, opcode: &Opcode) -> Option<u32> {
    if *opcode == Opcode::LoadValue{
        return Some(getu(some_instruction.into(), 0, 25).try_into().unwrap());
    }
    else{
        return None
    }
}

impl Instruction {

    pub fn new(instruction: u32) -> Instruction {
        let opcode = get_opcode(instruction.into());
        let a = get_a_bit(instruction.into(), &opcode);
        let b = get_b_bit(instruction.into(), &opcode);
        let c = get_c_bit(instruction, &opcode);
        let value = get_value(instruction, &opcode);

        Instruction {
            opcode,
            a,
            b,
            c,
            value
        }
    }
}