use std::collections::HashMap;

pub struct Emul {
    a: i16,
    d: i16,
    pc: i16,
    ram: [i16; 32768],
    prog: Vec<i16>,
    syms: HashMap<String,i16>,
}

impl Emul {
    pub fn new() -> Emul {
        let mut h = HashMap::new();
        h.insert("SP".to_string(), 0);
        h.insert("LCL".to_string(), 1);
        h.insert("ARG".to_string(), 2);
        h.insert("THIS".to_string(), 3);
        h.insert("THAT".to_string(), 4);
        Emul{a: 0, d: 0,pc: 0, ram: [0; 32768], prog: vec![], syms: h}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut em = Emul::new();
        assert_eq!(em.a, 0);
        assert_eq!(em.d, 0);
        assert_eq!(em.pc, 0);
        assert_eq!(em.ram[17], 0);
        assert!(em.prog.len() == 0);
        assert_eq!(em.syms["THIS"], 3);
    }
}
