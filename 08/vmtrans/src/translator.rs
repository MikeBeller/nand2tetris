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
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M+D\n"),
                    VMOp::SUB =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M-D\n"),
                    VMOp::AND =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M&D\n"),
                    VMOp::OR =>
                        r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M|D\n"),
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
                        writeln!(&mut r, "@{}\nD=A\n@{}\nA=M+D\nD=M", num, seg.base_var_str()).unwrap();
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

    #[test]
    fn trans_bin_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::ADD)), 
            "// add\n@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M+D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::SUB)), 
            "// sub\n@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M-D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::AND)), 
            "// and\n@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M&D\n");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::OR)), 
            "// or\n@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M|D\n");
    }

    #[test]
    fn trans_unary_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::NEG)), "// neg\n@SP\nA=M-1\nM=-M\n");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::NOT)), "// not\n@SP\nA=M-1\nM=!M\n");
    }

    #[test]
    fn trans_cmp_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::EQ)), 
                   "// eq\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\nM=-1\n".to_owned() +
                   "@TST.0\nD;JEQ\n@SP\nA=M\nM=0\n" +
                   "(TST.0)\n@SP\nM=M+1\n"
                   );
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::LT)), 
                   "// lt\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\nM=-1\n".to_owned() +
                   "@TST.1\nD;JLT\n@SP\nA=M\nM=0\n" +
                   "(TST.1)\n@SP\nM=M+1\n"
                   );
        assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::GT)), 
                   "// gt\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\nM=-1\n".to_owned() +
                   "@TST.2\nD;JGT\n@SP\nA=M\nM=0\n" +
                   "(TST.2)\n@SP\nM=M+1\n"
                   );
    }

    #[test]
    fn trans_push_test() {
        let mut tr = Translator::new("Splat");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::CONSTANT, 33)), 
            "// push constant 33\n@33\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::LOCAL, 3)), 
            "// push local 3\n@3\nD=A\n@LCL\nA=M+D\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::TEMP, 3)), 
            "// push temp 3\n@3\nD=A\n@5\nA=A+D\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::POINTER, 0)), 
            "// push pointer 0\n@THIS\nD=M\n".to_owned() +
                    "@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::POINTER, 1)), 
            "// push pointer 1\n@THAT\nD=M\n".to_owned() +
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
