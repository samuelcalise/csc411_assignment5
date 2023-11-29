#[derive(Debug)]

pub struct Instruction {
    pub op: Opcode,
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub value: Option<u32>
}

use bitpack::bitpack::{getu};

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
  
    let op = getu(instruction, 28, 4);

    if op == 0{
        Opcode::CMov
    }
    else if op == 1{
        Opcode::Load
    }
    else if op == 2{
        Opcode::Store
    }
    else if op == 3{
        Opcode::Add
    }
    else if op == 4{
        Opcode::Mul
    }
    else if op == 5{
        Opcode::Div
    }
    else if op == 6{
        Opcode::Nand
    }
    else if op == 7{
        Opcode::Halt
    }
    else if op == 8{
        Opcode::MapSegment
    }
    else if op == 9{
        Opcode::UnmapSegment
    }
    else if op == 10{
        Opcode::Output
    }
    else if op == 11{
        Opcode::Input
    }
    else if op == 12{
        Opcode::LoadProgram
    }
    else if op == 13{
        Opcode::LoadValue
    }
    else{
        Opcode::Err
    }
}

pub fn get_a(instruction: u64, op: &Opcode) -> u32 {
    if *op == Opcode::LoadValue{
        return getu(instruction, 25, 3) as u32;
    }
    else{
        return getu(instruction, 6, 3) as u32;
    }
}

pub fn get_b(instruction: u64, op: &Opcode) -> Option<u32> {
    if *op == Opcode::LoadValue{
        return None;
    }
    else{
        return Some(getu(instruction, 3, 3).try_into().unwrap());
    }

}

pub fn get_c(instruction: u32, op: &Opcode) -> Option<u32> {
    if *op == Opcode::LoadValue{
        return None
    }
    else{
        return Some(getu(instruction.into(), 0, 3).try_into().unwrap());
    }
    
}

pub fn get_value(instruction: u32, op: &Opcode) -> Option<u32> {
    if *op == Opcode::LoadValue{
        return Some(getu(instruction.into(), 0, 25).try_into().unwrap());
    }
    else{
        return None
    }
}



impl Instruction {

    pub fn new(instruction: u32) -> Instruction {
        let op = get_opcode(instruction.into());
        let a = get_a(instruction.into(), &op);
        let b = get_b(instruction.into(), &op);
        let c = get_c(instruction, &op);
        let value = get_value(instruction, &op);

        //our instruction struct is given new values for each new segment
        Instruction {
            op,
            a,
            b,
            c,
            value
        }
    }
}