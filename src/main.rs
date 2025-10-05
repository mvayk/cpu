use std::{thread, time};

/* sample program */
const SAMPLE_PROGRAM: [i32; 3] = [
    0x00,
    0x00,
    0x00,
];

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
            0x04 => Some(Opcode::Nop),
            0x87 => Some(Opcode::End),
            _    => None,
        }
    }
}

/* memory implementation */
struct Memory<> {
    mem: [u8; 32],
}

impl Memory {
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

    fn add(&mut self) {
        self.rax += self.rip;
    }

    fn sub(&mut self) {
        self.rax -= self.rip;
    }

    fn execute(&mut self, rsp: u8, rip: u8, memory: &Memory) {
        match Opcode::from_u8(rsp) {
            Some(Opcode::Add) => self.add(),
            Some(Opcode::Sub) => self.sub(),
            Some(Opcode::Mov) => self.rax = rip,
            Some(Opcode::Nop) => print!(""),
            Some(Opcode::End) => println!("end"),
            None => println!("unknwon"),
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
        //
        // mov 0x50 to rax
        // add 0x50 to whatever is on rax
        //
        mem: [0x01,0x32,0x02,0x32,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
              0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
              0x00,0x00,0x00,0x00,0x00,0x87]
    };

    while cpu.power == true {
        for index in 0..memory.mem.len() {
            thread::sleep(time::Duration::from_millis(10));
            cpu.rsp = memory.read(index);

            if cpu.rsp != Opcode::End as u8 {
                cpu.rip = memory.read(index + 1);
                cpu.execute(cpu.rsp, cpu.rip, &memory);
            } else {
                println!("program ended: 0x87");
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
