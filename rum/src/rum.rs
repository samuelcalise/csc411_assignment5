use std::env;
use std::process;
use std::convert::TryInto;
mod rum;
mod segments;
mod registers;
mod instructions;

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

    //let mut rum = rum::Rum::new(&runtime_instruction);

    let instruction_count = 0 as usize;
}
