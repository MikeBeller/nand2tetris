use std::fmt::Write;

use crate::types::*;

pub struct Translator {
    file_name: String,
    label_num: i32,
    return_num: i32,
}

impl Translator {
    pub fn new(fname: &str) -> Translator {
        Translator{file_name: fname.to_string(), label_num: 0, return_num: 0}
    }

    pub fn gen_bootstrap() -> String {
        //"@256\nD=A\n@SP\nM=D\n@Sys.init\n0;JMP\n".to_string() 
        let mut tr = Translator::new("bootstrap");
        "@256\nD=A\n@SP\nM=D\n".to_string()  + 
            &tr.trans_cmd(&VMCommand::Call("Sys.init".to_string(), 0))
    }

    fn get_return_address(&mut self) -> String {
        self.return_num += 1;
        format!("RETURN.{}", self.return_num-1)
    }

    pub fn trans_cmd(&mut self, cmd: &VMCommand) -> String {
        let mut r = String::new();
        match cmd {
            VMCommand::Arithmetic(op) => {
                writeln!(&mut r, "// {}", op.as_str()).unwrap();
                match *op {
                    VMOp::ADD =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D+M\n"),
                    VMOp::SUB =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M-D\n"),
                    VMOp::AND =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D&M\n"),
                    VMOp::OR =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D|M\n"),
                    VMOp::NEG =>
                        r.push_str("@SP\nA=M-1\nM=-M\n"),
                    VMOp::NOT =>
                        r.push_str("@SP\nA=M-1\nM=!M\n"),
                    VMOp::EQ | VMOp::LT | VMOp::GT => {
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\nM=-1\n");
                        write!(&mut r, "@TST.{}\n", self.label_num).unwrap();
                        if *op == VMOp::EQ {
                            r.push_str("D;JEQ\n");
                        } else if *op == VMOp::LT {
                            r.push_str("D;JLT\n");
                        } else {
                            r.push_str("D;JGT\n");
                        }
                        write!(&mut r, "@SP\nA=M\nM=0\n(TST.{})\n@SP\nM=M+1\n", self.label_num).unwrap();
                        self.label_num += 1;
                    }
                }
            },
            VMCommand::Push(seg, num) => {
                writeln!(&mut r, "// push {} {}", seg.as_str(), num).unwrap();
                match *seg {
                    VMSeg::CONSTANT => {
                        writeln!(&mut r, "@{}\nD=A", *num).unwrap();
                    },
                    VMSeg::LOCAL | VMSeg::ARGUMENT | VMSeg::THIS | VMSeg::THAT => {
                        writeln!(&mut r, "@{}\nD=A\n@{}\nA=D+M\nD=M", *num, seg.base_var_str()).unwrap();
                    },
                    VMSeg::TEMP => {
                        if *num < 0 || *num > 7 {
                            panic!("Invalid offset for temp segment: {}", *num);
                        }
                        writeln!(&mut r, "@{}\nD=M", *num+5).unwrap();
                    },
                    VMSeg::POINTER => {
                        if *num == 0 {
                            r.push_str("@THIS\nD=M\n");
                        } else if *num == 1 {
                            r.push_str("@THAT\nD=M\n");
                        } else {
                            panic!("Invalid offset for pointer segment: {}", *num);
                        }
                    },
                    VMSeg::STATIC => {
                        writeln!(&mut r, "@{}.{}\nD=M", self.file_name, *num).unwrap();
                    },
                }
                r.push_str("@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            },
            VMCommand::Pop(seg, num) => {
                writeln!(&mut r, "// pop {} {}", seg.as_str(), *num).unwrap();
                match *seg {
                    VMSeg::CONSTANT => {
                        panic!("WTF?  Can't pop constant");
                    },
                    VMSeg::LOCAL | VMSeg::ARGUMENT | VMSeg::THIS | VMSeg::THAT => {
                        // R15 = <segment> + <num>
                        writeln!(&mut r, "@{}\nD=M\n@{}\nD=D+A\n@R15\nM=D", seg.base_var_str(), *num).unwrap();
                    },
                    VMSeg::TEMP => {
                        if *num < 0 || *num > 7 {
                            panic!("Invalid offset for temp segment: {}", *num);
                        }
                        writeln!(&mut r, "@{}\nD=A\n@{}\nD=D+A\n@R15\nM=D", seg.base_var_str(), *num).unwrap();
                    },
                    VMSeg::POINTER => {
                        // could be optimized to avoid use of R15
                        if *num == 0 {
                            r.push_str("@THIS\nD=A\n@R15\nM=D\n");
                        } else if *num == 1 {
                            r.push_str("@THAT\nD=A\n@R15\nM=D\n");
                        } else {
                            panic!("Invalid offset for temp segment: {}", *num);
                        }
                    },
                    VMSeg::STATIC => {
                        // could be optimized to avoid use of R15
                        writeln!(&mut r, "@{}.{}\nD=A\n@R15\nM=D", self.file_name, *num).unwrap();
                    },
                }
                // D = *SP++
                r.push_str("@SP\nAM=M-1\nD=M\n");
                // *R15 = D
                r.push_str("@R15\nA=M\nM=D\n");
            },
            VMCommand::Label(label_str) => {
                writeln!(&mut r, "// label {}", label_str).unwrap();
                writeln!(&mut r, "({})", label_str).unwrap();
            },
            VMCommand::Goto(label_str) => {
                writeln!(&mut r, "// goto {}", label_str).unwrap();
                writeln!(&mut r, "@{}\n0;JMP", label_str).unwrap();
            },
            VMCommand::IfGoto(label_str) => {
                writeln!(&mut r, "// if-goto {}", label_str).unwrap();
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@{}\nD;JNE", label_str).unwrap();
            },
            VMCommand::Call(label_str, n_args) => {
                writeln!(&mut r, "// call {} {}", label_str, n_args).unwrap();
                // push return address
                let return_label = self.get_return_address();
                writeln!(&mut r, "@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", return_label).unwrap();
                // push LCL, ARG, THIS, THAT
                writeln!(&mut r, "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1").unwrap();
                writeln!(&mut r, "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1").unwrap();
                writeln!(&mut r, "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1").unwrap();
                writeln!(&mut r, "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1").unwrap();
                // compute and store new ARG
                writeln!(&mut r, "@SP\nD=M\n@{}\nD=D-A\n@ARG\nM=D", n_args+5).unwrap();
                // LCL = current SP
                writeln!(&mut r, "@SP\nD=M\n@LCL\nM=D").unwrap();
                // jump to the function, and write the return label
                writeln!(&mut r, "@{}\n0; JMP", label_str).unwrap();
                writeln!(&mut r, "({})", return_label).unwrap();
            },
            VMCommand::Function(label_str, n_locals) => {
                writeln!(&mut r, "// function {} {}", label_str, n_locals).unwrap();
                // Put the label
                writeln!(&mut r, "({})", label_str).unwrap();
                // Zero out the locals
                writeln!(&mut r, "@SP\nA=M").unwrap();
                for _i in 0..*n_locals {
                    writeln!(&mut r, "M=0\nA=A+1").unwrap();
                }
                writeln!(&mut r, "D=A\n@SP\nM=D").unwrap();
            },
            VMCommand::Return => {
                writeln!(&mut r, "// return").unwrap();
                // Copy return value onto argument 0
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@ARG\nA=M\nM=D").unwrap();
                // Save ARG in R15
                writeln!(&mut r, "@ARG\nD=M\n@R15\nM=D").unwrap();
                // Restore THAT, THIS, ARG, LCL
                //   set SP = LCL, then pop in reverse order
                writeln!(&mut r, "@LCL\nD=M\n@SP\nM=D").unwrap();
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@THAT\nM=D").unwrap();
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@THIS\nM=D").unwrap();
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@ARG\nM=D").unwrap();
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@LCL\nM=D").unwrap();
                // Save return address in R14
                writeln!(&mut r, "@SP\nAM=M-1\nD=M\n@R14\nM=D").unwrap();
                // SP = R15 + 1
                writeln!(&mut r, "@R15\nD=M\n@SP\nM=D+1").unwrap();
                // Jump to return address
                writeln!(&mut r, "@R14\nA=M\n0;JMP").unwrap();
            },
        }
        r
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::emul::Emul;
    use crate::asm::Asm;

    #[test]
    fn trans_bin_asm_test() {
        let table = vec![
            ((3,7,VMOp::ADD), 10),
            ((3,7,VMOp::SUB), -4),
            ((3,7,VMOp::AND), 3),
            ((3,7,VMOp::OR), 7),
            ((3,7,VMOp::EQ), 0),
            ((7,7,VMOp::EQ), -1),
            ((3,7,VMOp::LT), -1),
            ((3,7,VMOp::GT), 0),
            ((7,1,VMOp::GT), -1),
        ];

        for ((a,b,op), expected) in table {
            let mut tr = Translator::new("foo");
            let code = tr.trans_cmd(&VMCommand::Arithmetic(op));
            let mut em = Emul::new();
            em.set_ram(&[(0,258), (256, a), (257, b)]);
            em.run_code(&code, 50).unwrap();
            assert_eq!(em.ram[0], 257, "SP wrong");
            assert_eq!(em.ram[256], expected, "Wrong result from operation");
        }
    }

    #[test]
    fn trans_unary_asm_test() {
        let table = vec![
            ((3,VMOp::NEG), -3),
            ((-1,VMOp::NOT), 0),
        ];

        for ((a,op), expected) in table {
            let mut tr = Translator::new("foo");
            let code = tr.trans_cmd(&VMCommand::Arithmetic(op));
            let mut em = Emul::new();
            em.set_ram(&[(0,257), (256, a)]);
            em.run_code(&code, 50).unwrap();
            assert_eq!(em.ram[0], 257, "SP wrong");
            assert_eq!(em.ram[256], expected, "Wrong result from operation");
        }
    }

    #[test]
    fn trans_push_asm_test() {
        /* Scenario:
         * Assume a 2-argument, 2 local function has been called, starting from SP=256.
         * This means ARG=256, then there are 2 spots for arguments (256,257) and 5 spots
         * for saved RA,ARG,LCL,THIS,THAT (258,259,260,261,262).  So LCL should point to 263.
         * With two local variables, SP should then point to 265.  During the test, we push 9
         * items on the stack and check that they are where they should be, and that the SP
         * is correct.
         */
        let table = &[
            (VMSeg::CONSTANT, 33),
            (VMSeg::CONSTANT, 77),
            (VMSeg::LOCAL, 0),
            (VMSeg::LOCAL, 1),
            (VMSeg::ARGUMENT, 0),
            (VMSeg::ARGUMENT, 1),
            (VMSeg::POINTER, 0),
            (VMSeg::POINTER, 1),
            (VMSeg::TEMP, 1),
            //(VMSeg::STATIC, 9),
        ];

        let mut tr = Translator::new("Foo");
        let mut code = String::new();
        for (seg,n) in table {
            code += &tr.trans_cmd(&VMCommand::Push(*seg, *n));
        }
        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 265),
                   (1, 263),
                   (2, 256),
                   (3, -3),
                   (4, -4),
                   (6, -5),
                   //(16, -6),
                   (256, 17),
                   (257, 18),
                   (263, 97),
                   (264, 98),
        ]);

        em.run_code(&code, 100).unwrap();
        assert_eq!(em.ram[0], 265+table.len() as i16, "SP wrong");
        assert_eq!(em.ram[265], 33, "Wrong result from push constant 33");
        assert_eq!(em.ram[266], 77, "Wrong result from push constant 77");
        assert_eq!(em.ram[267], 97, "Wrong result from push local 0");
        assert_eq!(em.ram[268], 98, "Wrong result from push local 1");
        assert_eq!(em.ram[269], 17, "Wrong result from push argument 0");
        assert_eq!(em.ram[270], 18, "Wrong result from push argument 1");
        assert_eq!(em.ram[271], -3, "Wrong result from push pointer 0");
        assert_eq!(em.ram[272], -4, "Wrong result from push pointer 1");
        assert_eq!(em.ram[273], -5, "Wrong result from push temp 0");
        //assert_eq!(em.ram[270], -6, "Wrong result from static 9");
    }

    #[test]
    fn trans_push_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(&VMCommand::Push(VMSeg::STATIC, 9)), 
            "// push static 9\n@Splat.9\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    }

    #[test]
    fn trans_pop_asm_test() {
        /* Scenario:
         * Assume a 2-argument, 2 local function has been called, starting from SP=256.
         * This means ARG=256, then there are 2 spots for arguments (256,257) and 5 spots
         * for saved RA,ARG,LCL,THIS,THAT (258,259,260,261,262).  So LCL should point to 263.
         * With two local variables, SP should then point to 265.  We load the stack with
         * 9 values, and pop them into various places, and check that SP and popped values
         * are correct.
         */
        let table = &[
            (VMSeg::ARGUMENT, 0),
            (VMSeg::ARGUMENT, 1),
            (VMSeg::LOCAL, 0),
            (VMSeg::LOCAL, 1),
            (VMSeg::POINTER, 0),
            (VMSeg::POINTER, 1),
            (VMSeg::TEMP, 1),
            //(VMSeg::STATIC, 9),
        ];

        let mut tr = Translator::new("Foo");
        let mut code = String::new();
        for (seg,n) in table {
            code += &tr.trans_cmd(&VMCommand::Pop(*seg, *n));
        }
        let mut em = Emul::new();
        // everything in RAM is preset to zero
        em.set_ram(&[
                   (0, 272),
                   (1, 262),
                   (2, 256),
                   (265, -7),
                   (266, -6),
                   (267, -5),
                   (268, -4),
                   (269, -3),
                   (270, -2),
                   (271, -1),

        ]);

        em.run_code(&code, 100).unwrap();
        assert_eq!(em.ram[0], 265, "SP wrong");
        assert_eq!(em.ram[256], -1, "Wrong result from pop argument 0");
        assert_eq!(em.ram[257], -2, "Wrong result from pop argument 1");
        assert_eq!(em.ram[262], -3, "Wrong result from pop argument 0");
        assert_eq!(em.ram[263], -4, "Wrong result from pop argument 1");
        assert_eq!(em.ram[3], -5, "Wrong result from pop pointer 0");
        assert_eq!(em.ram[4], -6, "Wrong result from pop pointer 1");
        assert_eq!(em.ram[6], -7, "Wrong result from pop temp 1");
        //assert_eq!(em.ram[270], -6, "Wrong result from static 9");
    }

    #[test]
    fn trans_pop_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(&VMCommand::Pop(VMSeg::STATIC, 9)),
            "// pop static 9\n@Splat.9\nD=A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
    }

    #[test]
    fn trans_goto_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(&VMCommand::Label("foo".to_string())), "// label foo\n(foo)\n");
        assert_eq!(tr.trans_cmd(&VMCommand::Goto("foo".to_string())), "// goto foo\n@foo\n0;JMP\n");
        assert_eq!(tr.trans_cmd(&VMCommand::IfGoto("foo".to_string())), "// if-goto foo\n@SP\nAM=M-1\nD=M\n@foo\nD;JNE\n");
    }

    #[test]
    fn trans_call_test() {
        /* Test a basic call with 2 arguments.  LCL, ARG, THIS, THAT are set
         * to nonsense values just to check that they are stored properly.
         * */
        let mut tr = Translator::new("Foo");
        let code = tr.trans_cmd(&VMCommand::Call("BAR".to_string(), 2))
            + &tr.trans_cmd(&VMCommand::Label("BAR".to_string()));
        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 258),
                   (1, -1),
                   (2, -2),
                   (3, -3),
                   (4, -4),
                   (256, 11),
                   (257, 22),
        ]);

        em.run_code(&code, 100).unwrap();
        assert_eq!(em.ram[0], 263, "SP wrong");
        assert_eq!(em.ram[1], 263, "LCL wrong");
        assert_eq!(em.ram[2], 256, "ARG wrong");
        assert_eq!(em.ram[256], 11, "Argument 0 wrong");
        assert_eq!(em.ram[257], 22, "Argument 1 wrong");
        assert_eq!(em.ram[258], 47, "RA incorrect");
        assert_eq!(em.ram[259], -1, "SavedLCL incorrect");
        assert_eq!(em.ram[260], -2, "SavedARG incorrect");
        assert_eq!(em.ram[261], -3, "SavedThis incorrect");
        assert_eq!(em.ram[262], -4, "SavedThat incorrect");
        //assert_eq!(em.ram[270], -6, "Wrong result from static 9");
    }
    
    #[test]
    fn trans_function_test() {
        /* Test a basic "function" with 2 arguments.  Assumes CALL FOO 2 has
         * already been executed (the scenario from previous test).
         * */
        let mut tr = Translator::new("Foo");
        let code = tr.trans_cmd(&VMCommand::Function("BAR".to_string(), 2));
        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 263),
                   (1, 263),
                   (2, 256),
                   (256, 11),
                   (257, 22),
                   (258, 47), // not right
                   (259, -1),
                   (260, -2),
                   (261, -3),
                   (262, -4),
        ]);

        em.run_code(&code, 100).unwrap();
        assert_eq!(em.ram[0], 265, "SP wrong");
        assert_eq!(em.ram[1], 263, "LCL wrong");
        assert_eq!(em.ram[2], 256, "ARG wrong");
        assert_eq!(em.ram[256], 11, "Argument 0 wrong");
        assert_eq!(em.ram[257], 22, "Argument 1 wrong");
        assert_eq!(em.ram[258], 47, "RA incorrect");
        assert_eq!(em.ram[259], -1, "SavedLCL incorrect");
        assert_eq!(em.ram[260], -2, "SavedARG incorrect");
        assert_eq!(em.ram[261], -3, "SavedThis incorrect");
        assert_eq!(em.ram[262], -4, "SavedThat incorrect");
        assert_eq!(em.ram[263], 0, "Arg0 incorrect");
        assert_eq!(em.ram[264], 0, "Arg1 incorrect");
    }

    #[test]
    fn trans_return_test() {
        /* Test a basic "function" with 2 arguments.  Assumes CALL FOO 2 has
         * already been executed and then function FOO 2 has been executed.
         * */
        let mut tr = Translator::new("Foo");
        let code = tr.trans_cmd(&VMCommand::Return)
            + &tr.trans_cmd(&VMCommand::Label("RET1".to_string()));

        let mut asm = Asm::new();
        let cmds = asm.parse_code_str(&code).unwrap();

        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 266),
                   (1, 263),
                   (2, 256),
                   (256, 11),
                   (257, 22),
                   (258, asm.get_sym("RET1")),
                   (259, -1),
                   (260, -2),
                   (261, -3),
                   (262, -4),
                   (263, 0),
                   (264, 0),
                   (265, 99),
        ]);


        em.run(cmds, 100);
        assert_eq!(em.ram[0], 257, "SP wrong");
        assert_eq!(em.ram[1], -1, "LCL wrong");
        assert_eq!(em.ram[2], -2, "ARG wrong");
        assert_eq!(em.ram[3], -3, "THIS wrong");
        assert_eq!(em.ram[4], -4, "THAT wrong");
        assert_eq!(em.ram[256], 99, "Return val wrong");
    }

    #[test]
    fn trans_callret_test() {
        /* Test full round trip call/function/return with 2 arguments.
         * LCL, ARG, THIS, THAT are set to nonsense values just to check
         * that they are stored and restored properly.
         * */
        let table = vec![
            VMCommand::Push(VMSeg::CONSTANT, 3),
            VMCommand::Push(VMSeg::CONSTANT, 7),
            VMCommand::Call("ADD".to_string(), 2),
            VMCommand::Goto("END".to_string()),
            VMCommand::Function("ADD".to_string(), 1),
            VMCommand::Push(VMSeg::ARGUMENT, 0),
            VMCommand::Push(VMSeg::ARGUMENT, 1),
            VMCommand::Arithmetic(VMOp::ADD),
            VMCommand::Pop(VMSeg::LOCAL, 0),
            VMCommand::Push(VMSeg::LOCAL, 0),
            VMCommand::Return,
            VMCommand::Label("END".to_string()),
        ];

        let mut tr = Translator::new("Foo");
        let mut code = String::new();
        for cmd in &table {
            code += &tr.trans_cmd(cmd);
        }
        println!("{:?}", table);

        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 256),
                   (1, -1),
                   (2, -2),
                   (3, -3),
                   (4, -4),
        ]);

        em.run_code(&code, 1000).unwrap();
        assert_eq!(em.ram[0], 257, "SP wrong");
        assert_eq!(em.ram[1], -1, "LCL wrong");
        assert_eq!(em.ram[2], -2, "LCL wrong");
        assert_eq!(em.ram[3], -3, "LCL wrong");
        assert_eq!(em.ram[4], -4, "LCL wrong");
        assert_eq!(em.ram[256], 10, "Result wrong");
    }
    
    #[test]
    fn trans_callcallretret_test() {
        /* Test full round trip call/function/return with 2 arguments.
         * LCL, ARG, THIS, THAT are set to nonsense values just to check
         * that they are stored and restored properly.
         * */
        let table = vec![
            VMCommand::Push(VMSeg::CONSTANT, 3),
            VMCommand::Push(VMSeg::CONSTANT, 7),
            VMCommand::Call("ADD".to_string(), 2),
            VMCommand::Goto("END".to_string()),
            VMCommand::Function("ADD".to_string(), 1),
            VMCommand::Push(VMSeg::ARGUMENT, 0),
            VMCommand::Push(VMSeg::ARGUMENT, 1),
            VMCommand::Arithmetic(VMOp::ADD),
            VMCommand::Pop(VMSeg::LOCAL, 0),
            VMCommand::Push(VMSeg::LOCAL, 0),
            VMCommand::Push(VMSeg::CONSTANT, 9),
            VMCommand::Call("SUB".to_string(), 2),
            VMCommand::Return,
            VMCommand::Function("SUB".to_string(), 1),
            VMCommand::Push(VMSeg::ARGUMENT, 0),
            VMCommand::Push(VMSeg::ARGUMENT, 1),
            VMCommand::Arithmetic(VMOp::SUB),
            VMCommand::Pop(VMSeg::LOCAL, 0),
            VMCommand::Push(VMSeg::LOCAL, 0),
            VMCommand::Return,
            VMCommand::Label("END".to_string()),
        ];

        let mut tr = Translator::new("Foo");
        let mut code = String::new();
        for cmd in &table {
            code += &tr.trans_cmd(cmd);
        }

        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 256),
                   (1, -1),
                   (2, -2),
                   (3, -3),
                   (4, -4),
        ]);

        em.run_code(&code, 1000).unwrap();
        assert_eq!(em.ram[0], 257, "SP wrong");
        assert_eq!(em.ram[1], -1, "LCL wrong");
        assert_eq!(em.ram[2], -2, "LCL wrong");
        assert_eq!(em.ram[3], -3, "LCL wrong");
        assert_eq!(em.ram[4], -4, "LCL wrong");
        assert_eq!(em.ram[256], 1, "Result wrong");
    }
    
}

