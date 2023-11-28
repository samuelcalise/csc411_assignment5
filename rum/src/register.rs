

#[derive(Clone, Debug)]

pub struct Register {
    register: Vec<u32>
}


impl Register{


    pub fn new() -> Register
    {
        Register{
            register: vec![0; 8];
        }
    }


    pub fn get_registerValue(&self, register: usize) -> u32
    {
        self.register[register]
    }

    pub fn set_registerValue(&self, register: usize, some_value: u32)
    {
        self.register[register] = some_value
    }
}