use crate::types::*;

pub fn parse_str(cmd_str: &str) -> Option<VMCommand> {
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
}
