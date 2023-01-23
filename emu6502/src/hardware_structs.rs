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
    pc: u16,        // Program Counter
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
    fn read_byte(&mut self, mem: &Memory, cycles: &mut u32) -> u8 {
        *cycles -= 1;
        let temp = mem.read_data(self.pc);
        println!("fetched byte {} with value {}", self.pc, temp);
        self.pc -= 1;
        return temp;

    }

    fn write_byte(&mut self, mem: &mut Memory, cycles: &mut u32, data: u8, addr: u16) {
        *cycles -= 1;
        mem.write_data(addr, data);
        self.pc -= 1;  
    }




    pub fn execute(&mut self, mem: &mut Memory, cycles: &mut u32) {
        // opcodes
        // load
        let LDA_IM: u8 = 0xA9;  // Load ACC immidiate
        let LDX_IM: u8 = 0xA2;  // Load X immidiate
        let LDY_IM: u8 = 0xA0;  // Load Y immidiate

        // store
        let STA_AB: u8 = 0x85;  // Store ACC absolute
        let STX_AB: u8 = 0x8E;  // Store X absolute
        let STY_AB: u8 = 0x8C;  // Store Y absolute



        while cycles > &mut 0 {
            let ins = Self::read_byte(self, mem, cycles);

            // Load/Store Operations
            if ins == LDA_IM {
                println!("began load ACC immidiate");
                let value = Self::read_byte(self, mem, cycles);
                // load value
                self.acc = value;
                println!("new acc value: {}", self.acc);

                // set flags
                self.z = (self.acc == 0);
                self.n = (self.acc & 0b10000000) > 0;
            } else if ins == LDX_IM {
                println!("began load X immidiate");
                let value = Self::read_byte(self, mem, cycles);
                // load value
                self.x = value;

                // set flags
                self.z = (self.acc == 0);
                self.n = (self.acc & 0b10000000) > 0;


            } else if ins == LDY_IM {
                println!("began load Y immidiate");
                let value = Self::read_byte(self, mem, cycles);
                // load value
                self.y = value;

                // set flags
                self.z = (self.acc == 0);
                self.n = (self.acc & 0b10000000) > 0;

            } else if ins == STA_AB {
                println!("began store ACC absolute");
                // write value
                let mut addr: u16 = Self::read_byte(self, mem, cycles) as u16;
                addr = addr << 8;
                addr += Self::read_byte(self, mem, cycles) as u16;
                Self::write_byte(self, mem, cycles, self.acc, addr);

            } else if ins == STX_AB {
                println!("began store X absolute");


            } else if ins == STY_AB {
                println!("began store Y absolute");

            }
                    
            
        }


    }

    
}