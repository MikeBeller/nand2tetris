use crate::asm::{Command};

pub struct Emul {
    a: i16,
    d: i16,
    pc: i16,
    ram: [i16; 32768],
    prog: Vec<Command>,
}

impl Emul {
    pub fn new() -> Emul {
        Emul{a: 0, d: 0,pc: 0, ram: [0; 32768], prog: vec![]}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let em = Emul::new();
        assert_eq!(em.a, 0);
        assert_eq!(em.d, 0);
        assert_eq!(em.pc, 0);
        assert_eq!(em.ram[17], 0);
        assert!(em.prog.len() == 0);
    }
}
