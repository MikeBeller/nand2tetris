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
}

#[derive(Debug,PartialEq,Copy,Clone)]
enum VMCommand {
    Arithmetic(VMOp),
    Push(VMSeg, i32),
    //Pop(VMSeg, i32),
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
    } else {
        None
    }
}

#[test]
fn simple_parse_str_test() {
    assert_eq!(parse_str(""), None);
    assert_eq!(parse_str("// foo bar"), None);
    assert_eq!(parse_str("add"), Some(VMCommand::Arithmetic(VMOp::ADD)));
    assert_eq!(parse_str("pop foo bar"), None);
    assert_eq!(parse_str("push splat bar"), None);
    assert_eq!(parse_str("push constant 33"), Some(VMCommand::Push(VMSeg::CONSTANT, 33)));
}

struct Translator {
    label_num: i32
}

impl Translator {
    fn new() -> Translator {
        Translator{label_num: 0}
    }

    fn trans_cmd(&mut self, c: VMCommand) -> String {
        let mut r = String::new();
        match c {
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
            VMCommand::Push(seg, num) if seg == VMSeg::CONSTANT => {
                writeln!(&mut r, "// push {} {}", seg.as_str(), num).unwrap();
                writeln!(&mut r, "@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", num).unwrap();
            },
            _ => {}
        }
        r
    }
}

#[test]
fn trans_bin_test() {
    let mut tr = Translator::new();
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
    let mut tr = Translator::new();
    assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::NEG)), "// neg\n@SP\nA=M-1\nM=-M\n");
    assert_eq!(tr.trans_cmd(VMCommand::Arithmetic(VMOp::NOT)), "// not\n@SP\nA=M-1\nM=!M\n");
}

#[test]
fn trans_cmp_test() {
    let mut tr = Translator::new();
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
fn trans_pushconst_test() {
    let mut tr = Translator::new();
    assert_eq!(tr.trans_cmd(VMCommand::Push(VMSeg::CONSTANT, 33)), 
        "// push constant 33\n@33\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
}


fn main() -> Result<(), std::io::Error> {
    let root = std::env::args().nth(1).expect("usage: $0 <fname.vm>");
    let infile_path = format!("{}.vm", root);
    let outfile_path = format!("{}.asm", root);
    let infile = File::open(infile_path)?;
    let rdr = BufReader::new(&infile);
    let mut outfile = File::create(outfile_path)?;

    let cmds = rdr.lines()
        .filter_map(|x| parse_str(&x.unwrap()));

    let mut tr = Translator::new();
    for cmd in cmds {
        let asm = tr.trans_cmd(cmd);
        write!(&mut outfile, "{}", asm).unwrap();
    }
    
    Ok(())
}

