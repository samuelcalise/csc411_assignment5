#[derive(Clone, Debug)]

pub struct Register {
    vec_registers: Vec<u32>,

}

impl Register {

    pub fn new() -> Register {
        Register {
            vec_registers: vec![0; 8],
        }
    }

    pub fn get_register_value(&self, register: usize) -> u32 {
        self.vec_registers[register]
    }

    pub fn set_register_value(&mut self, register: usize, value: u32) {
        self.vec_registers[register] = value
    }
}