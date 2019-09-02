// vmtranslator.rs

#[derive(Debug,PartialEq)]
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

#[derive(Debug,PartialEq)]
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

#[derive(Debug,PartialEq)]
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

fn trans_cmd(c: VMCommand) -> String {
    let mut r = String::new();
    match c {
        VMCommand::Arithmetic(op) => {
            r.push_str("// ");
            r.push_str(op.as_str());
            r.push_str("\n");
            match op {
                VMOp::ADD =>
                    r.push_str("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M+D\n"),
                _ => {}
            }
        },
        _ => {}
    }
    r
}

#[test]
fn trans_add_test() {
    assert_eq!(trans_cmd(VMCommand::Arithmetic(VMOp::ADD)), 
        "// add\n@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M+D\n")
}

