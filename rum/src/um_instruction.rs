use bitpack::bitpack::{getu};

#[derive(Debug)]
pub struct Instruction {
    pub op: Opcode,
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub value: Option<u32>
}

impl Instruction {

    pub fn new(some_instruction: u32) -> Instruction 
    {
        let op = find_opcode(some_instruction);
        let a = get_aBit(some_instruction, &op);
        let b = get_bBit(some_instruction, &op);
        let c = get_cBit(some_instruction, &op);
        let value = get_value(some_instruction, &op);

        Instruction {
            op,
            a,
            b,
            c,
            value
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode { //Recommended by TA Nick
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

pub fn find_opcode(some_instruction: u32) -> Opcode
{
    let this_opcode = getu(some_instruction, 28, 4);

    //we have various 'if-else' statements to return which opcode is found
    if this_opcode == 0{
        Opcode::CMov
    }
    else if this_opcode == 1{
        Opcode::Load
    }
    else if this_opcode == 2{
        Opcode::Store
    }
    else if this_opcode == 3{
        Opcode::Add
    }
    else if this_opcode == 4{
        Opcode::Mul
    }
    else if this_opcode == 5{
        Opcode::Div
    }
    else if this_opcode == 6{
        Opcode::Nand
    }
    else if this_opcode == 7{
        Opcode::Halt
    }
    else if this_opcode == 8{
        Opcode::MapSegment
    }
    else if this_opcode == 9{
        Opcode::UnmapSegment
    }
    else if this_opcode == 10{
        Opcode::Output
    }
    else if this_opcode == 11{
        Opcode::Input
    }
    else if this_opcode == 12{
        Opcode::LoadProgram
    }
    else if this_opcode == 13{
        Opcode::LoadValue
    }
    else{
        Opcode::Err
    }
} 

pub fn get_aBit(some_instruction: u32, opcode: &Opcode) -> u32
{
    if *opcode == Opcode::LoadValue
    {
        getu(some_instruction, 25, 3)
    }
    else
    {
        getu(some_instruction, 6, 3)
    }
}

pub fn get_bBit(some_instruction: u32, opcode: &Opcode) -> Option<u32>
{
    if *opcode == Opcode::LoadValue
    {
        None
    }
    else
    {
        Some(getu(some_instruction, 3, 3));
    }
}

pub fn get_cBit(some_instruction: u32, opcode: &Opcode) -> Option<u32>
{
    if *opcode == Opcode::LoadValue
    {
        None
    }
    else
    {
        Some(getu(some_instruction, 0, 3))
    }
}

pub fn get_value(some_instruction: u32, opcode: &Opcode) -> Option<u32>
{
    if *opcode == Opcode::LoadValue
    {
        Some(getu(some_instruction, 0, 25))
    }
    else
    {
        None
    }
}

