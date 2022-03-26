use crate::keyboard::Keyboard;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    pub memory: [u8; 4096], // 4096 bytes of memory
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub stack_pointers: u8,
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
            stack_pointers: 0,
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
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.program_counter as usize + 1] as u16);
        opcode
    }

    pub fn emulate_cycle(&mut self) {
        // reading opcode
        let opicode = self.read_opcode();

        self.execute_opcode(opcode);

        if self.delay_timer > 0 { self.delay_timer -= 1}
        if self.sound_timer > 0 { self.sound_timer -= 1}
        if self.sound_timer == 1 { println!("BEEEP!!"); }
    }
}
