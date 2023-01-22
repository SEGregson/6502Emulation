pub const SIZE:usize = 1024*64;

pub struct Memory {
    pub data: [u8; 1024*64]
}

impl Memory {
    pub fn init() -> Memory {
        Memory {
            data: [0; SIZE]
        }
    }

    pub fn read_data(&self, addr: u16) -> u8 {
        return self.data[addr as usize];
    }

    pub fn write_data(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }

    pub fn out(&self)  {
        for i in 0..SIZE {
            print!("{} ", self.read_data( i.try_into().unwrap()));
        }
        print!("\n");
        
    }
}

pub struct CPU {
    // Registers
    pub pc: u16,        // Program Counter
    sp: u16,            // Stack Pointer
    acc: u8,            // Accumulator
    x: u8,              // Index Register X
    y: u8,              // Index Register Y

    // Flags
    c: bool,            // Carry Flag
    z: bool,            // Zero Flag
    i: bool,            // Interupt Disable
    d: bool,            // Decimal Mode Flag
    b: bool,            // Break Command
    v: bool,            // Overflow Flag
    n: bool,            // Negitive Flag

}

impl CPU {
    pub fn reset() -> CPU {
        CPU {
            pc: (0xFFFC),
            sp: (0x0100),
            acc: (0),
            x: (0),
            y: (0),

            c: (false),
            z: (false),
            i: (false),
            d: (false),
            b: (false),
            v: (false),
            n: (false)
        }
    }
    fn fetch_byte(&mut self, mem: &mut Memory, cycles: &mut u32) -> u8 {
        self.pc += 1;
        *cycles -= 1;

        return mem.read_data(self.pc);
    }

    pub fn execute(&mut self, mem: &mut Memory, cycles: &mut u32) {
        // opcodes
        let LDA_IM: u8 = 0xA9;


        while cycles > &mut 0 {
            let ins = Self::fetch_byte(self, mem, cycles);
            println!("fetched byte {} with value {}", self.pc-1, ins);

            if ins == LDA_IM {
                println!("began load immidiate instruction");
                let value = Self::fetch_byte(self, mem, cycles);
                // load value
                self.acc = value;

                // set flags
                self.z = (self.acc == 0);
                self.n = (self.acc & 0b10000000) > 0;
            }
                    
            
        }


    }

    
}