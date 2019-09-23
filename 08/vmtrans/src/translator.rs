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
            &tr.trans_cmd(VMCommand::Call("Sys.init".to_string(), 0))
    }

    fn get_return_address(&mut self) -> String {
        self.return_num += 1;
        format!("RETURN.{}", self.return_num-1)
    }

    pub fn trans_cmd(&mut self, cmd: VMCommand) -> String {
        let mut r = String::new();
        match cmd {
            VMCommand::Arithmetic(op) => {
                writeln!(&mut r, "// {}", op.as_str()).unwrap();
                match op {
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
                        if op == VMOp::EQ {
                            r.push_str("D;JEQ\n");
                        } else if op == VMOp::LT {
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
                match seg {
                    VMSeg::CONSTANT => {
                        writeln!(&mut r, "@{}\nD=A", num).unwrap();
                    },
                    VMSeg::LOCAL | VMSeg::ARGUMENT | VMSeg::THIS | VMSeg::THAT => {
                        writeln!(&mut r, "@{}\nD=A\n@{}\nA=D+M\nD=M", num, seg.base_var_str()).unwrap();
                    },
                    VMSeg::TEMP => {
                        if num < 0 || num > 7 {
                            panic!("Invalid offset for temp segment: {}", num);
                        }
                        writeln!(&mut r, "@{}\nD=A\n@5\nA=A+D\nD=M", num).unwrap();
                    },
                    VMSeg::POINTER => {
                        if num == 0 {
                            r.push_str("@THIS\nD=M\n");
                        } else if num == 1 {
                            r.push_str("@THAT\nD=M\n");
                        } else {
                            panic!("Invalid offset for pointer segment: {}", num);
                        }
                    },
                    VMSeg::STATIC => {
                        writeln!(&mut r, "@{}.{}\nD=M", self.file_name, num).unwrap();
                    },
                }
                r.push_str("@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            },
            VMCommand::Pop(seg, num) => {
                writeln!(&mut r, "// pop {} {}", seg.as_str(), num).unwrap();
                match seg {
                    VMSeg::CONSTANT => {
                        panic!("WTF?  Can't pop constant");
                    },
                    VMSeg::LOCAL | VMSeg::ARGUMENT | VMSeg::THIS | VMSeg::THAT => {
                        // R15 = <segment> + <num>
                        writeln!(&mut r, "@{}\nD=M\n@{}\nD=D+A\n@R15\nM=D", seg.base_var_str(), num).unwrap();
                    },
                    VMSeg::TEMP => {
                        if num < 0 || num > 7 {
                            panic!("Invalid offset for temp segment: {}", num);
                        }
                        writeln!(&mut r, "@{}\nD=A\n@{}\nD=D+A\n@R15\nM=D", seg.base_var_str(), num).unwrap();
                    },
                    VMSeg::POINTER => {
                        // could be optimized to avoid use of R15
                        if num == 0 {
                            r.push_str("@THIS\nD=A\n@R15\nM=D\n");
                        } else if num == 1 {
                            r.push_str("@THAT\nD=A\n@R15\nM=D\n");
                        } else {
                            panic!("Invalid offset for temp segment: {}", num);
                        }
                    },
                    VMSeg::STATIC => {
                        // could be optimized to avoid use of R15
                        writeln!(&mut r, "@{}.{}\nD=A\n@R15\nM=D", self.file_name, num).unwrap();
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
                for _i in 0..n_locals {
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
            let code = tr.trans_cmd(VMCommand::Arithmetic(op));
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
            let code = tr.trans_cmd(VMCommand::Arithmetic(op));
            let mut em = Emul::new();
            em.set_ram(&[(0,257), (256, a)]);
            em.run_code(&code, 50).unwrap();
            assert_eq!(em.ram[0], 257, "SP wrong");
            assert_eq!(em.ram[256], expected, "Wrong result from operation");
        }
    }

    #[test]
    fn trans_push_asm_constant_test() {
        let table = &[
            (VMSeg::CONSTANT, 33),
            (VMSeg::CONSTANT, 77),
            (VMSeg::LOCAL, 0),
            (VMSeg::ARGUMENT, 0),
            (VMSeg::POINTER, 0),
            (VMSeg::POINTER, 1),
        ];

        let mut tr = Translator::new("foo");
        let mut code = String::new();
        for (seg,n) in table {
            code += &tr.trans_cmd(VMCommand::Push(*seg, *n));
        }
        let mut em = Emul::new();
        em.set_ram(&[
                   (0, 263),
                   (1, 262),
                   (2, 256),
                   (3, -3),
                   (4, -4),
                   (256, 17),
                   (262, 99),
        ]);

        em.run_code(&code, 50).unwrap();
        assert_eq!(em.ram[0], 263+table.len() as i16, "SP wrong");
        assert_eq!(em.ram[263], 33, "Wrong result from push constant 33");
        assert_eq!(em.ram[264], 77, "Wrong result from push constant 77");
        assert_eq!(em.ram[265], 99, "Wrong result from push local 0");
        assert_eq!(em.ram[266], 17, "Wrong result from push argument 0");
        assert_eq!(em.ram[267], -3, "Wrong result from push pointer 0");
        assert_eq!(em.ram[268], -4, "Wrong result from push pointer 1");
    }

    #[test]
    fn trans_push_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::TEMP, 3)), 
            "// push temp 3\n@3\nD=A\n@5\nA=A+D\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::STATIC, 9)), 
            "// push static 9\n@Splat.9\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    }

    #[test]
    fn trans_pop_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Pop(VMSeg::LOCAL, 3)),
            "// pop local 3\n@LCL\nD=M\n@3\nD=D+A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Pop(VMSeg::TEMP, 7)),
            "// pop temp 7\n@5\nD=A\n@7\nD=D+A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Pop(VMSeg::POINTER, 0)),
            "// pop pointer 0\n@THIS\nD=A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Pop(VMSeg::POINTER, 1)),
            "// pop pointer 1\n@THAT\nD=A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Pop(VMSeg::STATIC, 9)),
            "// pop static 9\n@Splat.9\nD=A\n@R15\nM=D\n".to_owned() + 
            "@SP\nAM=M-1\nD=M\n" + 
            "@R15\nA=M\nM=D\n");
    }

    #[test]
    fn trans_goto_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Label("foo".to_string())), "// label foo\n(foo)\n");
        assert_eq!(tr.trans_cmd(VMCommand::Goto("foo".to_string())), "// goto foo\n@foo\n0;JMP\n");
        assert_eq!(tr.trans_cmd(VMCommand::IfGoto("foo".to_string())), "// if-goto foo\n@SP\nAM=M-1\nD=M\n@foo\nD;JNE\n");
    }

    #[test]
    fn trans_function_test() {
        let mut tr = Translator::new("Splat");
        println!("{}", tr.trans_cmd(VMCommand::Call("FOO".to_string(), 2)).replace("\n","\\n"));
        println!("{}", tr.trans_cmd(VMCommand::Function("FOO".to_string(), 2)).replace("\n","\\n"));
        println!("{}", tr.trans_cmd(VMCommand::Return).replace("\n","\\n"));
    }
}
