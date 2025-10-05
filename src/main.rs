use std::fs;
use std::{thread, time};

const FILE_PATH: &str = "executable.g";
const MEM_SIZE: usize = 32;

/* opcodes */
#[repr(u8)]
enum Opcode {
    Mov = 0x01,
    Add = 0x02,
    Sub = 0x03,
    Nop = 0x00,

    End = 0x87,
}

impl Opcode {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Opcode::Mov),
            0x02 => Some(Opcode::Add),
            0x03 => Some(Opcode::Sub),
            0x00 => Some(Opcode::Nop),
            0x87 => Some(Opcode::End),
            _    => None,
        }
    }
}

/* memory implementation */
struct Memory<> {
    mem: [u8; MEM_SIZE],
}

impl Memory {
    fn load(&mut self, values: [u8; MEM_SIZE]) {
        for index in 0..self.mem.len() {
            self.mem[index] = values[index];
        }
    }

    fn write(&mut self, index: usize, value: u8) {
        self.mem[index] = value;
    }

    fn read(&self, index: usize) -> u8 {
        self.mem[index]
    }
}

/* cpu implementation */
struct CPU {
    power: bool, /* on or off */
    current_cycle: i32,

    /* registers */
    rip: u8,
    rsp: u8,

    rax: u8,
}

impl CPU {
    fn cycle(&mut self) {
        self.current_cycle += 1;
    }

    fn add(&mut self) -> usize {
        self.rax += self.rip;
        2
    }

    fn sub(&mut self) -> usize {
        self.rax -= self.rip;
        2
    }

    fn set_rax(&mut self, value: u8) -> usize {
        self.rax = value;
        2
    }

    fn execute(&mut self, rsp: u8, rip: u8, memory: &Memory) -> usize {
        match Opcode::from_u8(rsp) {
            Some(Opcode::Add) => self.add(),
            Some(Opcode::Sub) => self.sub(),
            Some(Opcode::Mov) => self.set_rax(rip),
            Some(Opcode::Nop) => 1,
            Some(Opcode::End) => 1,
            None => 1,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut cpu = CPU {
        power: true,
        current_cycle: 0,

        /* registers */
        rip: 0,
        rsp: 0,
        rax: 0,
    };

    let mut memory = Memory {
        mem: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    };

    let executable = std::fs::read_to_string(FILE_PATH)
        .expect("failed to find executable file");

    for (index, token) in executable.split_whitespace().enumerate() {
        if index >= MEM_SIZE {
            break;
        }

        let hex = token.trim_start_matches("0x");
        memory.mem[index] = u8::from_str_radix(hex, 16).unwrap();
    }

    let mut index = 0;
    let mut increment = 0;

    while cpu.power == true {
        while index < memory.mem.len() {
            index = index + increment;
            println!("{index}");
            thread::sleep(time::Duration::from_millis(10));
            cpu.rsp = memory.read(index);

            if cpu.rsp != Opcode::End as u8 && index < memory.mem.len() {
                cpu.rip = memory.read(index + 1);
                increment = cpu.execute(cpu.rsp, cpu.rip, &memory);
            } else {
                println!("program ended");
                cpu.power = false; break
            }

            cpu.cycle();
        }
    }

    println!("[+] CPU Cycled: {}", cpu.current_cycle);
    println!("RIP: {}", cpu.rip);
    println!("RSP: {}", cpu.rsp);
    println!("RAX: {}", cpu.rax);

    Ok(())
}
