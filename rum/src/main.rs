use std::env;
use std::process;
use std::convert::TryInto;
mod rum;
mod segments;
mod registers;
mod um_instruction;

//function take from past lab
pub fn load_instruction(input: Option<&str>) -> Vec<u32> 
{
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut buf = Vec::<u8>::new(); 
    raw_reader.read_to_end(&mut buf).unwrap();

    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap())) 
        .collect();
    
    instructions
}

fn main() 
{
    let command_line: Vec<String> = env::args().collect();

    let command_file = &command_line[1];

    let runtime_instruction = load_instruction(Some(command_file));

    let mut rum = rum::Rum::new(&runtime_instruction);

    let instruction_count = 0 as usize;

    for i in 0..usize::MAX{

        let this_instruction = rum.get_instruction(instruction_count);

        instruction_count += 1;

        if instruction.op == um_instruction::Opcode::CMov{
            rum.cmov(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Load {
            rum.load(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Store {
            rum.store(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Add {
            rum.add(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Mul {
            rum.mul(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Div {
            rum.div(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Nand {
            rum.nand(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Halt {
            process::exit(0);
        }
        else if instruction.op == um_instruction::Opcode::MapSegment {
            rum.mapsegment(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::UnmapSegment {
            rum.unmapsegment(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Output {
            rum.output(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Input {
            rum.input(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::LoadProgram {
            counter = rum.loadprogram(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::LoadValue {
            rum.loadvalue(this_instruction);
        }
        else if instruction.op == um_instruction::Opcode::Err {
            panic!("Unknown opcode for instruction {:?}", this_instruction)
        }
    }
}
