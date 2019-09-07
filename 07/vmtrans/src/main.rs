// vmtranslator.rs
use std::fmt::Write;
use std::fs::File;
use std::io::Write as OtherWrite;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug,PartialEq,Copy,Clone)]
enum VMOp {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

impl VMOp {
    fn from_str(s: &str) -> Option<VMOp> {
        match s {
            "add" => Some(VMOp::ADD),
            "sub" => Some(VMOp::SUB),
            "neg" => Some(VMOp::NEG),
            "eq" => Some(VMOp::EQ),
            "gt" => Some(VMOp::GT),
            "lt" => Some(VMOp::LT),
            "and" => Some(VMOp::AND),
            "or" => Some(VMOp::OR),
            "not" => Some(VMOp::NOT),
            _ => None
        }        
    }

    fn as_str(&self) -> &'static str {
        match self {
            VMOp::ADD => "add",
            VMOp::SUB => "sub",
            VMOp::NEG => "neg",
            VMOp::EQ => "eq",
            VMOp::GT => "gt",
            VMOp::LT => "lt",
            VMOp::AND => "and",
            VMOp::OR => "or",
            VMOp::NOT => "not",
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
enum VMSeg {
    LOCAL,
    ARGUMENT,
    THIS,
    THAT,
    CONSTANT,
    STATIC,
    TEMP,
    POINTER,
}

impl VMSeg {
    fn from_str(s: &str) -> Option<VMSeg> {
        match s {
            "local" => Some(VMSeg::LOCAL),
            "argument" => Some(VMSeg::ARGUMENT),
            "this" => Some(VMSeg::THIS),
            "that" => Some(VMSeg::THAT),
            "constant" => Some(VMSeg::CONSTANT),
            "static" => Some(VMSeg::STATIC),
            "temp" => Some(VMSeg::TEMP),
            "pointer" => Some(VMSeg::POINTER),
            _ => None
        }        
    }

    fn as_str(&self) -> &'static str {
        match self {
            VMSeg::LOCAL => "local",
            VMSeg::ARGUMENT => "argument",
            VMSeg::THIS => "this",
            VMSeg::THAT => "that",
            VMSeg::CONSTANT => "constant",
            VMSeg::STATIC => "static",
            VMSeg::TEMP => "temp",
            VMSeg::POINTER => "pointer",
        }
    }

    fn base_var_str(&self) -> &'static str {
        match self {
            VMSeg::LOCAL => "LCL",
            VMSeg::ARGUMENT => "ARG",
            VMSeg::THIS => "THIS",
            VMSeg::THAT => "THAT",
            VMSeg::TEMP => "5",
            _ => panic!("no base var for segment?")
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
enum VMCommand {
    Arithmetic(VMOp),
    Push(VMSeg, i32),
    Pop(VMSeg, i32),
}

fn parse_str(cmd_str: &str) -> Option<VMCommand> {
    let ws: Vec<&str> = cmd_str.split_whitespace().collect();
    if ws.len() < 1 || ws[0] == "//" {
        return None;
    }

    if let Some(vmc) = VMOp::from_str(ws[0]) {
        Some(VMCommand::Arithmetic(vmc))
    } else if ws[0] == "push" {
        if ws.len() == 3 {
            if let Some(seg) = VMSeg::from_str(ws[1]) {
                if let Ok(n) = ws[2].parse::<i32>() {
                    Some(VMCommand::Push(seg, n))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else if ws[0] == "pop" {
        if ws.len() == 3 {
            if let Some(seg) = VMSeg::from_str(ws[1]) {
                if let Ok(n) = ws[2].parse::<i32>() {
                    Some(VMCommand::Pop(seg, n))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

struct Translator {
    file_name: String,
    label_num: i32,
}

impl Translator {
    fn new(fname: &str) -> Translator {
        Translator{file_name: fname.to_string(), label_num: 0}
    }

    fn trans_cmd(&mut self, cmd: VMCommand) -> String {
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
            //_ => {}
        }
        r
    }
}

fn main() -> Result<(), std::io::Error> {
    let base = std::env::args().nth(1).expect("usage: $0 <fname.vm>");
    let infile_path = format!("{}.vm", base);
    let outfile_path = format!("{}.asm", base);
    let infile = File::open(infile_path)?;
    let rdr = BufReader::new(&infile);
    let mut outfile = File::create(outfile_path)?;

    let cmds = rdr.lines()
        .filter_map(|x| parse_str(&x.unwrap()));

    let mut tr = Translator::new(&base);
    for cmd in cmds {
        let asm = tr.trans_cmd(cmd);
        write!(&mut outfile, "{}", asm).unwrap();
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse_str_test() {
        assert_eq!(parse_str(""), None);
        assert_eq!(parse_str("// foo bar"), None);
        assert_eq!(parse_str("add"), Some(VMCommand::Arithmetic(VMOp::ADD)));
        assert_eq!(parse_str("pop foo bar"), None);
        assert_eq!(parse_str("push splat bar"), None);
        assert_eq!(parse_str("push constant 33"), Some(VMCommand::Push(VMSeg::CONSTANT, 33)));
        assert_eq!(parse_str("pop local 3"), Some(VMCommand::Pop(VMSeg::LOCAL, 3)));
    }

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

}
