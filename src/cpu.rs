use crate::keyboard::Keyboard;
use crate::display::{Display, FONT_SET};

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    pub memory: [u8; 4096], // 4096 bytes of memory
    pub register: [u8; 16],
    pub stack: [u16; 16],
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub index_register: u16,
    pub program_counter: u16,
    pub display: [u8; 64 * 32],
    pub keyboard: Keyboard,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            index_register: 0,
            program_counter: PROGRAM_START,
            display: [0; 64 * 32],
            keyboard: Keyboard::new(),
        }
    }

    fn read_opcode(&self) -> u16 {
        // read a 16 bit word from ram
        let opcode: u16 = (self.memory[self.program_counter as usize] as u16) << 8
            | (self.memory[self.program_counter as usize + 1] as u16);
        opcode
    }

    pub fn emulate_cycle(&mut self) {
        // reading opcode
        let opicode = self.read_opcode();

        self.execute_opcode(opcode);

        if self.delay_timer > 0 { self.delay_timer -= 1 }
        if self.sound_timer > 0 { self.sound_timer -= 1 }
        if self.sound_timer == 1 { println!("BEEEP!!"); }
    }

    fn execute_opcode(&mut self, opcode: u16) {

        let addr = opcode & 0x0FFF; // last 12 bits
        let nibble = opcode & 0x00FF; // last 4 bits
        let x = ((opcode & 0x0F00) >> 8) as usize; // lower 4 bits of the upper byte
        let y = ((opcode & 0x00F0) >> 4) as usize; // upper 4 btis of the upper byte
        let byte = opcode & 0x00FF as u8; // last 8 bits

        // separating the digits
        op_1 = (opcode & 0xF000) >> 12;
        op_2 = (opcode & 0x0F00) >> 8;
        op_3 = (opcode & 0x00F0) >> 4;
        op_4 = opcode & 0x000F;

        self.program_counter += 2;

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xE, 0) => {
                //CLS
                self.display.cls();
            }

            (0, 0, 0xE, 0xE) => {
                //return
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }

            (1, _, _, _) => {
                // Jump to addr
                self.program_counter = addr;
            }

            (2, _, _, _) => {
                // CALL addr
                /*
                    1. Increment the stack stack pointer
                    2. add the current program counter on top of the stack
                    3. move the program counter to addr
                */
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = addr;
            }

            (3, _, _, _) => {
                // Skip the next instruction if xth register is equal to byte
                self.stack_pointer += if self.registers[x] == byte { 2 } else { 0 };
            }

            (4, _, _, _) => {
                // Skip the next instruction if xth register is not equal to byte
                self.stack_pointer += if self.registers[x] != byte { 2 } else { 0 };
            }

            (5, _, _, 0) => {
                // Skip the next instruction if the xth register equals the yth register
                self.stack_pointer += if self.registers[x] == self.registers[y] { 2 } else { 0 };
            }

            (6, _, _, _) => {
                // sets the xth register to be byte
                self.registers[x] = byte;
            }

            (7, _, _, _) => {
                // Adds byte to the register
                self.registers[x] += byte;
            }
            
            (8, _, _, 0) => {
                // Stores the value of register y in register x
                self.register[x] = self.registers[y];
            }

            (8, _, _, 1) => {
                // stores bitwise or of register x and register y in register x
                self.register[x] |= self.registers[y];
            }

            (8, _, _, 2) => {
                // stores the bitwise and of register x and register y in register x
                self.register[x] &= self.registers[y];
            }

            (8, _, _, 3) => {
                // stores the bitwise and of register x and register y in register x
                self.register[x] ^= self.registers[y];
            }

            (8, _, _, 4) => {
                // store the sum of registers y and x in register x
                let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                match overflow {
                    true => self.registers[0xF] = 1,
                    false => self.registers[0xF] = 0,
                }

                self.registers[x] = sum;
            }

            (8, _, _, 5) => {
                // store the sum of registers y and x in register x
                let (diff, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                match overflow {
                    true => self.registers[0xF] = 0,
                    false => self.registers[0xF] = 1,
                }

                self.registers[x] = diff;
            }

            (8, _, _, 6) => {
                // if least significant digit of the xth register is 1
                // then register F is set to 1 else 0 and divide register x by 2
                self.registers[0xF] = self.registers[x] % 2;
                self.registers[x] = self.registers[x] >> 1;
            }
            
            (8, _, _, 7) => {
                // store the sum of registers y and x in register x
                let (diff, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                match overflow {
                    true => self.registers[0xF] = 0,
                    false => self.registers[0xF] = 1,
                }

                self.registers[x] = diff;
            }

            (8, _, _, 0xE) => {
                // if the most significant bit is 1 then register F is set to 1
                // then register x is multiplied by 2
                self.registers[0xF] = self.registers[x] & 0x80;
                self.registers[x] = self.registers[x] < 1;
            }

            (9, _, _, 0) => {
                // Skip the next instruction if register x is not equal to register y
                self.program_counter += if registers[x] != registers[y] { 2 } else { 0 };
            }

        }
    }
}
