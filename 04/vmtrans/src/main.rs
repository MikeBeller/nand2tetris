// vmtranslator.rs

#[derive(Debug,PartialEq)]
enum VMCommand {
    Arithmetic(String),
    Push(String, i32),
    //Pop(String, i32),
}

fn parse(cmd_str: &str) -> Option<VMCommand> {
    let ws: Vec<&str> = cmd_str.split_whitespace().collect();
    if ws.len() < 1 || ws[0] == "//" {
        return None;
    }

    match ws[0] {
        "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" =>
            Some(VMCommand::Arithmetic(ws[0].to_string())),
        "push" =>
            if ws.len() == 3 && ws[2].parse::<i32>().is_ok() {
                Some(VMCommand::Push(ws[1].to_string(), ws[2].parse::<i32>().unwrap()))
            } else {
                None
            },
        _ => None
    }
}

#[test]
fn simple_parse() {
    assert_eq!(parse(""), None);
    assert_eq!(parse("// foo bar"), None);
    assert_eq!(parse("add"), Some(VMCommand::Arithmetic("add".to_string())));
    assert_eq!(parse("pop foo bar"), None);
    assert_eq!(parse("push splat bar"), None);
    assert_eq!(parse("push constant 33"), Some(VMCommand::Push("constant".to_string(), 33)));
}
