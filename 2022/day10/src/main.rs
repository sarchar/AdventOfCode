enum CpuOp {
    NOOP,
    ADDX,
}

struct Cpu {
    opcode    : CpuOp,
    operand   : i32,
    cycle     : u32,
    inst_cycle: u8,

    reg_x     : i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            opcode    : CpuOp::NOOP,
            operand   : -1,
            cycle     : 1,
            inst_cycle: 0,

            reg_x     : 1,
        }
    }

    fn set_opcode(&mut self, opcode: CpuOp) { 
        self.opcode = opcode;
    }

    fn set_operand(&mut self, operand: i32) {
        self.operand = operand;
    }

    fn step(&mut self) -> u8 {
        self.cycle      += 1;
        self.inst_cycle += 1;

        match self.opcode {
            CpuOp::ADDX => {
                if self.inst_cycle == 2 {
                    self.reg_x += self.operand;
                    self.inst_cycle = 0;
                }
            },

            CpuOp::NOOP => {
                if self.inst_cycle == 1 {
                    self.inst_cycle = 0;
                }
            },
        }

        self.inst_cycle
    }
}

fn main() {
    // could encapsulate the screen but I don't wanna.
    let mut screen = vec![vec!['.'; 40]; 6];
    let mut screen_x = 0usize;
    let mut screen_y = 0usize;

    let mut cpu = Cpu::new();
    let mut signal = 0i32;

    // parse the file line by line
    for line in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        let mut words = line.split_whitespace();
        let inst      = words.next().unwrap();

        match inst {
            "addx" => {
                cpu.set_opcode(CpuOp::ADDX);
                cpu.set_operand(words.next().unwrap().parse::<i32>().unwrap());
            },

            "noop" => {
                cpu.set_opcode(CpuOp::NOOP);
            }

            _ => {
                panic!("invalid opcode");
            }
        }

        loop {
            // compute the current signal before executing the instruction
            if cpu.cycle >= 20 && cpu.cycle <= 220 && ((cpu.cycle - 20) % 40) == 0 {
                signal += cpu.reg_x * (cpu.cycle as i32);
            }

            // first: draw the sprite
            let spr = (cpu.reg_x + 1) - screen_x as i32;
            if spr >= 0 && spr < 3 { 
                let screen_row = screen.get_mut(screen_y).unwrap();
                screen_row[screen_x] = '#';
            }

            // increment render location
            screen_x += 1;
            if screen_x >= screen[0].len() {
                screen_x = 0;
                screen_y += 1;
                if screen_y >= screen.len() {
                    screen_y = 0;
                    break;
                }
            }

            let inst_cycle = cpu.step();
            if inst_cycle == 0 { break; }
        }
    }

    println!("signal: {signal}");

    // display the screen
    for screen_row in screen.iter() {
        for c in screen_row.iter() {
            print!("{}", c);
        }
        println!("");
    }
}
