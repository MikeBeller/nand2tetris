use crate::asm::{Comp,Dest,Jump,Command,Asm,ParserError};

pub struct Emul {
    pub a: i16,
    pub d: i16,
    pc: usize,
    pub ram: [i16; 32768],
}

impl Emul {
    pub fn new() -> Emul {
        Emul{a: 0, d: 0,pc: 0, ram: [0; 32768]}
    }

    fn m(&self) -> i16 {
        if self.a < 0 {
            panic!("Invalid address in A register: {}", self.a);
        }
        self.ram[self.a as usize]
    }

    fn do_comp(&self, comp: &Comp) -> i16 {
        match comp {
            Comp::Zero => 0,
            Comp::One => 1,
            Comp::MinusOne => -1,
            Comp::D => self.d,
            Comp::A => self.a,
            Comp::NotD => !self.d,
            Comp::NotA => !self.a,
            Comp::MinusD => -self.d,
            Comp::MinusA => -self.a,
            Comp::DPlusOne => self.d + 1,
            Comp::APlusOne => self.a + 1,
            Comp::DMinusOne => self.d - 1,
            Comp::AMinusOne => self.a - 1,
            Comp::DPlusA => self.d + self.a,
            Comp::DMinusA => self.d - self.a,
            Comp::AMinusD => self.a - self.d,
            Comp::DAndA => self.d & self.a,
            Comp::DOrA => self.d | self.a,
            Comp::M => self.m(),
            Comp::NotM => !self.m(),
            Comp::MinusM => -self.m(),
            Comp::MPlusOne => self.m() + 1,
            Comp::MMinusOne => self.m() - 1,
            Comp::DPlusM => self.d + self.m(),
            Comp::DMinusM => self.d - self.m(),
            Comp::MMinusD => self.m() - self.d,
            Comp::DAndM => self.d & self.m(),
            Comp::DOrM => self.d | self.m(),
        }
    }

    pub fn do_dest(&mut self, dest: &Dest, res: i16) {
        if *dest == Dest::M || *dest == Dest::MD || *dest == Dest::AM || *dest == Dest::AMD {
            if self.a < 0 {
                panic!("Invalid address in A register: {}", self.a);
            }
            println!("Setting M({}) to {}", self.a, res);
            self.ram[self.a as usize] = res;
        }
        if *dest == Dest::D || *dest == Dest::MD || *dest == Dest::AD || *dest == Dest::AMD {
            println!("Setting D to {}", res);
            self.d = res;
        }
        if *dest == Dest::A || *dest == Dest::AM || *dest == Dest::AD || *dest == Dest::AMD {
            println!("Setting A to {}", res);
            self.a = res;
        }
    }

    pub fn do_jump(&mut self, jump: &Jump, res: i16) {
        let jmp: bool = match jump {
            Jump::Null => false,
            Jump::JGT => res > 0,
            Jump::JEQ => res == 0,
            Jump::JGE => res >= 0,
            Jump::JLT => res < 0,
            Jump::JNE => res != 0,
            Jump::JLE => res <= 0,
            Jump::JMP => true,
        };

        if jmp {
            self.pc = self.a as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn run(&mut self, prog: Vec<Command>, maxticks: i32) {
        let mut n_ticks = 0i32;
        while self.pc < prog.len() && n_ticks < maxticks {
            println!("Doing command: {:?}", prog[self.pc]);
            match &prog[self.pc] {
                Command::A(n) => {
                    self.a = *n;
                    self.pc += 1;
                },
                Command::C(ref dest, ref comp, ref jump) => {
                    let res = self.do_comp(comp);
                    self.do_dest(dest, res);
                    self.do_jump(jump, res);
                },
                _ => {panic!("WTF?  Can't emulate command: {:?}", prog[self.pc]);}
            }
            n_ticks += 1;
        }

        if self.pc > prog.len() {
            panic!("Attempt to access non-existent instruction {}", self.pc);
        }
    }

    pub fn run_code(&mut self, code: &str, maxticks: i32) -> Result<(), ParserError> {
        let mut asm = Asm::new();
        let cmds = asm.parse_code_str(code)?;
        self.run(cmds, maxticks);
        Ok(())
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut em = Emul::new();
        em.run_code("@33\nD=A\nA=1\nM=D\n", 50).unwrap();
        assert_eq!(em.ram[1], 33);
    }
    // need way more tests?
}
