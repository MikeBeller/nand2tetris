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
enum VMCommand {
    Arithmetic(VMOp),
    Push(String, i32),
    //Pop(String, i32),
}

fn parse_str(cmd_str: &str) -> Option<VMCommand> {
    let ws: Vec<&str> = cmd_str.split_whitespace().collect();
    if ws.len() < 1 || ws[0] == "//" {
        return None;
    }

    if let Some(vmc) = VMOp::from_str(ws[0]) {
        Some(VMCommand::Arithmetic(vmc))
    } else if ws[0] == "push" {
        if ws.len() == 3 && ws[1] == "constant" && ws[2].parse::<i32>().is_ok() {
            Some(VMCommand::Push("constant".to_string(), ws[2].parse::<i32>().unwrap()))
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
    assert_eq!(parse_str("push constant 33"), Some(VMCommand::Push("constant".to_string(), 33)));
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
