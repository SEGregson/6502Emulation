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
    sp: u16,
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
    fn read_sp(&mut self, mem: &mut Memory, cycles: &mut u32, pop: bool) -> u8 {
        *cycles -= 1;
        let temp = mem.read_data(self.sp);
        if pop {
            mem.write_data(self.sp, 0);
            self.sp -= 1;
        }  
        return temp;
    }

    fn write_sp(&mut self, mem: &mut Memory, cycles: &mut u32, data: u8, push: bool) {
        *cycles -= 1;
        if push {
            self.sp += 1;
        }
        mem.write_data(self.sp, data);
        self.pc -= 1;  
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

        // store
        let STA_AB: u8 = 0x85;  // Store ACC absolute
        let STX_AB: u8 = 0x8E;  // Store X absolute
        let STY_AB: u8 = 0x8C;  // Store Y absolute

        // register transfers
        let TAX: u8 = 0xAA;     // transfer acc to x
        let TAY: u8 = 0xA8;     // transfer acc to y
        let TXA: u8 = 0x8A;     // transfer x to acc
        let TYA: u8 = 0x98;     // transfer y to acc

        // Stack operations
        let TSX: u8 = 0xBA;     // transfer sp to x
        let TXS: u8 = 0x9A;     // transfer x to sp
        let PHA: u8 = 0x48;     // push acc onto stack
        let PHP: u8 = 0x08;     // push process status to stack
        let PLA: u8 = 0x68;     // pull from stack to acc
        let PLP: u8 = 0x28;     // pull process status from stack

        // Logical operations
        let AND_IM: u8 = 0x29;  // bitwise AND immidiate
        let EOR_IM: u8 = 0x49;  // bitwise XOR immidiate (why does it have to be EOR not XOR?)
        let ORA_IM: u8 = 0x09;  // bitwise OR immidiate
        let BIT_AB: u8 = 0x2C;  // bit test absolute (needs research)

        



        while cycles > &mut 0 {
            let ins:u8 = Self::read_byte(self, mem, cycles);

            match ins {
                // Load/Store Operations
                0xA9 => {   // load acc immidiate
                    println!("began load ACC immidiate");
                    let value = Self::read_byte(self, mem, cycles);
                    // load value
                    self.acc = value;
                    println!("new acc value: {}", self.acc);

                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0xA2 => {   // load x immidiate
                    println!("began load X immidiate");
                    let value = Self::read_byte(self, mem, cycles);
                    // load value
                    self.x = value;

                    // set flags
                    self.z = (self.x == 0);
                    self.n = (self.x & 0b10000000) > 0;
                },
                0xA0 => {   // load y immidiate
                    println!("began load Y immidiate");
                    let value = Self::read_byte(self, mem, cycles);
                    // load value
                    self.y = value;

                    // set flags
                    self.z = (self.y == 0);
                    self.n = (self.y & 0b10000000) > 0;

                },
                0x85 => {   // store acc absolute
                    println!("began store ACC absolute");
                    // write value
                    let mut addr: u16 = Self::read_byte(self, mem, cycles) as u16;
                    addr = addr << 8;
                    addr += Self::read_byte(self, mem, cycles) as u16;
                    Self::write_byte(self, mem, cycles, self.acc, addr);

                },
                0x8E => {   // store x absolute
                    println!("began store X absolute");
                    // write value
                    let mut addr: u16 = Self::read_byte(self, mem, cycles) as u16;
                    addr = addr << 8;
                    addr += Self::read_byte(self, mem, cycles) as u16;
                    Self::write_byte(self, mem, cycles, self.x, addr);


                },
                0x8C => {   // store y absolute
                    println!("began store Y absolute");
                    // write value
                    let mut addr: u16 = Self::read_byte(self, mem, cycles) as u16;
                    addr = addr << 8;
                    addr += Self::read_byte(self, mem, cycles) as u16;
                    Self::write_byte(self, mem, cycles, self.y, addr);

                },
                // transfers
                0xAA => {   // transfer acc to x
                    println!("Began transfer acc to x");
                    // move value
                    self.x = self.acc;
                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0xA8 => {   // transfer acc to y
                    println!("Began transfer acc to y");
                    // move value
                    self.y = self.acc;
                    // set flags
                    self.z = (self.y == 0);
                    self.n = (self.y & 0b10000000) > 0;
                },
                0x8A => {   // transfer x to acc
                    println!("Began transfer x to acc");
                    // move value
                    self.acc = self.x;
                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;

                },
                0x98 => {   // transfer y to acc
                    println!("Began transfer y to acc");
                    // move value
                    self.x = self.y;
                    // set flags
                    self.z = (self.x == 0);
                    self.n = (self.x & 0b10000000) > 0;
                },
                // stack operations
                0xBA => {   // transfer sp to x
                    println!("Began transfer sp to x");
                    // move value
                    self.x = Self::read_sp(self, mem, cycles, false);
                    // set flags
                    self.z = (self.x == 0);
                    self.n = (self.x & 0b10000000) > 0;
                },
                0x9A => {   // transfer x to sp
                    println!("Began transfer x to sp");
                    // move value
                    Self::write_sp(self, mem, cycles, self.x, false);
                    // set flags
                    self.z = (self.x == 0);
                    self.n = (self.x & 0b10000000) > 0;
                },
                0x48 => {   // push acc to stack
                    println!("began push acc to stack");
                    // push value
                    Self::write_sp(self, mem, cycles, self.acc, true);
                },
                0x08 => {   // process state to stack 
                    println!("began push process state to stack");
                    // research inner workings
                },
                0x68 => {   // pop stack to acc
                    println!("began pop sp to acc");
                    // pop data
                    self.acc = Self::read_sp(self, mem, cycles, true);
                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0x28 => {   // pop process status from stack
                    println!("began pop status from stack");
                    // research inner workings
                },
                // logical expressions
                0x29 => {   // bitwise AND immidiate
                    println!("began immidiate AND");
                    let arg = Self::read_byte(self, mem, cycles);
                    self.acc = self.acc & arg;
                    
                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0x49 => {   // bitwise EOR immidiate (why isn't it XOR?)
                    println!("began immidiate EOR");
                    let arg = Self::read_byte(self, mem, cycles);
                    self.acc = self.acc ^ arg;

                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0x09 => {   // bitwise OR immidiate
                    println!("began immidiate OR");
                    let arg = Self::read_byte(self, mem, cycles);
                    self.acc = self.acc | arg;

                    // set flags
                    self.z = (self.acc == 0);
                    self.n = (self.acc & 0b10000000) > 0;
                },
                0x2C => {   // bit test (research)
                    println!("began bit test");
                },
                // arithmetic
                0x69 => { // immidiate add with carry
                    let arg = Self::read_byte(self, mem, cycles);
                    self.acc += arg; 

                    // set flags
                    self.c = (self.acc > 0b1111111);
                    self.z = (self.acc == 0);
                    self.v = (self.acc > 0b11111111);
                    self.n = (self.acc > 0b1000000);
                },
                0xE9 => {   // immidiate subtract with carry
                    let arg = Self::read_byte(self, mem, cycles);
                    self.acc -= arg;

                    // set flags
                    self.c = (self.acc > 0b1111111);
                    self.z = (self.acc == 0);
                    self.v = (self.acc > 0b11111111);
                    self.n = (self.acc > 0b1000000);
                },
                0xC9 => {   // immidiate compare acc
                    let arg = Self::read_byte(self, mem, cycles);
                    
                    // comapre
                    self.c = (self.acc >= arg);
                    self.z = (self.acc == arg);
                    self.n = (self.acc > 0b1000000);
                },
                0xE0 => {   // immidiate compare x
                    let arg = Self::read_byte(self, mem, cycles);

                    // compare
                    self.c = (self.x >= arg);
                    self.z = (self.x == arg);
                    self.n = (self.x > 0b1000000);
                },
                0xC0 => {   // immidiate compare y
                    let arg = Self::read_byte(self, mem, cycles);

                    // compare
                    self.c = (self.y >= arg);
                    self.z = (self.y == arg);
                    self.n = (self.y > 0b1000000);
                },
                // increment + decrement
                _=> {
                    println!("{} is not a recognised opcode", ins);
                }
            
            }
        }
    }
    
}