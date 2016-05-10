extern crate rand;

struct Chip8 {
    opcode      : u16,
    memory      : [u8; 4096],
    regV        : [u8; 16],
    indexReg    : u16,
    progCount   : u16,
    gfx         : [u8; 64 * 32],
    delay_timer : u8,
    sound_timer : u8,
    stack       : [u16; 16],
    sp          : u16,
    key         : [u8; 16],

    fontset     : [u8; 80],
    drawFlag    : bool,
    keyPressed  : bool,
}

impl Chip8 {
    fn initialize(&self) {
        self.indexReg = 0;
        self.progCount = 0;
        self.sp = 0;

        for i in self.memory.iter(){
            i = 0;
        }
        for i in 0usize..16 {
            self.regV[i] = 0;
            self.stack[i] = 0;
            self.key[i] = 0;
        }
        self.fontset =
        [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        

    }
    fn emulateCycle(&self) {
        self.opcode = self.memory[self.pc] << 8 | self.memory[self.pc + 1];
        match self.opcode & 0xF000 {
            0x0000 => self.opcode0xxx(), 
            0x1000 => self.opcode1xxx(), 
            0x2000 => self.opcode0xxx(), 
            0x3000 => self.opcode0xxx(), 
            0x4000 => println!("Calls subroutine at NNN."),
            0x5000 => println!("Sets I to the address NNN."),
            0x6000 => println!("Jumps to the address NNN plus V0."),
            0x7000 => self.opcode0xxx(), 
            0x8000 => println!("Jumps to address NNN."),
            0x9000 => println!("Calls subroutine at NNN."),
            0xA000 => println!("N."),
            0xB000 => println!("Calls subroutine at NNN."),
            0xC000 => println!("Sets I to the address NNN."),
            0xD000 => println!("Jumps to the address NNN plus V0."),
            0xE000 => println!("Jumps to the address NNN plus V0."),
            0xF000 => println!("Jumps to the address NNN plus V0."),

            _ => println!(""),
        }
    }
    fn opcode0xxx(&self) {
        match self.opcode & 0x000F{
            0x0000 => for i in self.gfx.iter() {
                        i = 0;
                      },
            0x000E => {
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                      },
            _      => println!("Unknown opcode"),
        }
    }
    fn opcode1xxx(&self) {
        self.pc = self.opcode & 0x0FFF
    }
    fn opcode2xxx(&self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        pc = self.opcode & 0x0FFF;
    }
    fn opcode3xxx(&self) {
        let index :usize = (self.opcode & 0x0F00) >> 8;
        let addrt = self.opcode & 0x00FF;
        self.pc += 2;
        if regV[index] == addrt {
            self.pc += 2;
        }
       
    }
    fn opcode4xxx(&self) {
        let index :usize = (self.opcode & 0x0F00) >> 8;
        let addrt = self.opcode & 0x00FF;
        self.pc += 2;
        if regV[index] != addrt {
            self.pc += 2;
        }
    }
    fn opcode5xxx(&self) {
        let indexX :usize = (self.opcode & 0x0F00) >> 8;
        let indexY :usize = (self.opcode & 0x00F0) >> 4;
        self.pc += 2;
        if self.regV[indexX] == self.regV[indexY] {
            self.pc += 2;
        }
    }

    fn opcode6xxx(&self) {
        let indexX :usize = (self.opcode & 0x0F00) >> 8;
        self.regV[indexX] = self.opecode & 0x00FF;
        self.pc += 2;
    }

    fn opcode7xxx(&self) {
        let indexX :usize = (self.opcode & 0x0F00) >> 8;
        self.regV[indexX] += self.opecode & 0x00FF;
        self.pc += 2;
    }
    fn opcode8xxx(&self) {
        
        let indexX :usize = (self.opcode & 0x0F00) >> 8;
        let indexY :usize = (self.opcode & 0x00F0) >> 4;
        self.pc += 2;
        
        match self.opcode & 0x000F {
            0x0000 => self.regV[indexX] = self.regV[indexY],
            0x0001 => self.regV[indexX] = self.regV[indexX] | self.regV[indexY],
            0x0002 => self.regV[indexX] = self.regV[indexX] & self.regV[indexY],
            0x0003 => self.regV[indexX] = self.regV[indexX] ^ self.regV[indexY],
            0x0004 => {
                        if self.regV[indexX] + self.regV[indexY] > 0xFF {
                            self.regV[0xF as usize] = 1;
                        }
                        else {
                            self.regV[0xF as usize] = 0;
                        }
                        self.regV[indexX] += self.regV[indexY];
                      },
            0x0005 => {
                        if self.regV[indexX] - self.regV[indexY] < 0x00 {
                            self.regV[0xF as usize] = 0;
                        }
                        else {
                            self.regV[0xF as usize] = 1;
                        }
                        self.regV[indexX] += self.regV[indexY];
                      },
            0x0006 => {
                        self.regV[0xF as usize] = self.regV[indexX] & 1;
                        self.regV[indexX] = self.regV[indexX] >> 1;
                      },
            0x0007 => {
                        if self.regV[indexY] - self.regV[indexX] < 0x00 {
                            self.regV[0xF as usize] = 0;
                        }
                        else {
                            self.regV[0xF as usize] = 1;
                        }
                        self.regV[indexX] = self.regV[indexY] - self.regV[indexX];
                      },
            0x000E => {
                        self.regV[0xF as usize] = self.regV[indexX] >> 15;
                        self.regV[indexX] = self.regV[indexX] << 1;
                      },
        }
    }
    fn opcode9xxx(&self) {
        let indexX :usize = (self.opcode & 0x0F00) >> 8;
        let indexY :usize = (self.opcode & 0x00F0) >> 4;
        self.pc += 2;
        if self.regV[indexX] != self.regV[indexY] {
            self.pc += 2;
        }

    }
    fn opcodeAxxx(&self) {
        indexReg = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    fn opcodeBxxx(&self) {
        self.pc = (self.opcode & 0x0FFF) + self.regV[0];
    }
    fn opcodeCxxx(&self) {
        let indexX: usize = (self.opcode & 0x0F00) >> 8;
        use rand::Rng;
        self.regV[indexX] = (self.opcode & 0x00FF) & rand::thread_rng().gen();
        self.pc += 2;
    } 
    fn opcodeDxxx(&self) {
        let indexX: usize = (self.opcode & 0x0F00) >> 8;
        let indexY: usize = (self.opcode & 0x00F0) >> 4;
        let iX = self.regV[indexX];
        let iY = self.regV[indexY];
        let height = self.opcode & 0x000F;
        let mut pixel = 0;

        self.regV[0xF as usize] = 0;
        for y in 0..height {
            pixel = self.memory[(self.indexReg + y) as usize];
            for x in 0..8 {
                if (self.pixel & (0x80 >> x)) != 0 {
                    if self.gfx[(iX + x + ((y + iY) * 64)) as usize] == 1 {
                        self.regV[0xF as usize] = 1;
                    }
                    self.gfx[(iX + x + ((y + iY) * 64)) as usize] ^= 1;
                }
            }
        }

        self.drawFlag = true;
        self.pc += 2;
    }
    fn opcodeExxx(&self) {
        match self.opcode & 0x000F{
            0x000E => {
                if self.key[self.regV[((self.opcode & 0x0F00) >> 8) as usize] as usize] != 0 {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x0001 => {
                if self.key[self.regV[((self.opcode & 0x0F00) >> 8) as usize] as usize] == 0 {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            _      => println!("Unknown opcode"),
        }
    }
    fn opcodeFxxx(&self) {
        match self.opcode & 0x00FF{
            0x0007 => {
                let indexX: usize = (self.opcode & 0x0F00) >> 8;
                self.regV[indexX] = self.delay_timer;
                self.pc += 2;
            },
            0x000A => println!("Wait for key pressed"),
            0x0015 => {
                let indexX: usize = (self.opcode & 0x0F00) >> 8;
                self.delay_timer = self.regV[indexX];
            },
            0x0018 => {
                let indexX: usize = (self.opcode & 0x0F00) >> 8;
                self.sound_timer = self.regV[indexX];
            },
            0x001E => {
                let indexX: usize = (self.opcode & 0x0F00) >> 8;
                self.indexReg += self.regV[indexX];
            },
            0x0029 => println!("Return from sub routine"),
            0x0033 => println!("Return from sub routine"),
            0x0055 => println!("Return from sub routine"),
            0x0065 => println!("Return from sub routine"),
            _      => println!("Unknown opcode"),
        }
    }
    fn loadProgram(&self, buffer : &mut Vec<u8>) {
       for i in 0usize..buffer.len() {
           self.memory[i + 512] = buffer[i];
       }
    }
}
