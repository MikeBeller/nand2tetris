use crate::types::*;
use std::fmt;

pub struct Parser {
    file_name: String,
    line_num: i32,
}

#[derive(PartialEq)]
pub struct ParserError {
    file: String,
    line: i32,
    msg: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: File: {}, Line: {} -- {}\n", self.file, self.line, self.msg)
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {} {} {}", self.file, self.line, self.msg)
    }
}

impl Parser {
    pub fn new(fname: &str) -> Parser {
        Parser{file_name: fname.to_string(), line_num: 0}
    }

    fn parser_error(&self, msg: &str) -> ParserError {
        ParserError{file: self.file_name.to_owned(), line: self.line_num, msg: msg.to_string()}
    }

    pub fn parse_str(&self, cmd_str: &str) -> Result<Option<VMCommand>, ParserError> {
        let ws: Vec<&str> = cmd_str.split_whitespace().collect();
        if ws.len() < 1 || ws[0] == "//" {
            Ok(None)
        } else if let Some(vmc) = VMOp::from_str(ws[0]) {
            Ok(Some(VMCommand::Arithmetic(vmc)))
        } else if ws[0] == "push" {
            if ws.len() == 3 {
                if let Some(seg) = VMSeg::from_str(ws[1]) {
                    if let Ok(n) = ws[2].parse::<i32>() {
                        Ok(Some(VMCommand::Push(seg, n)))
                    } else {
                        Err(self.parser_error(&format!("Invalid number in push command: {}", cmd_str)))
                    }
                } else {
                    Err(self.parser_error(&format!("Invalid segment name in push command: {}", cmd_str)))
                }
            } else {
                Err(self.parser_error(&format!("Invalid push command: {}", cmd_str)))
            }
        } else if ws[0] == "pop" {
            if ws.len() == 3 {
                if let Some(seg) = VMSeg::from_str(ws[1]) {
                    if let Ok(n) = ws[2].parse::<i32>() {
                        Ok(Some(VMCommand::Pop(seg, n)))
                    } else {
                        Err(self.parser_error(&format!("Invalid number in pop command: {}", cmd_str)))
                    }
                } else {
                    Err(self.parser_error(&format!("Invalid segment name in pop command: {}", cmd_str)))
                }
            } else {
                Err(self.parser_error(&format!("Invalid pop command: {}", cmd_str)))
            }
        } else {
            Err(self.parser_error(&format!("Invalid command: {}", ws[0])))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse_str_test() {
        let p = Parser::new("foo.vm");
        assert_eq!(p.parse_str(""), Ok(None));
        assert_eq!(p.parse_str("// foo bar"), Ok(None));
        assert_eq!(p.parse_str("add"), Ok(Some(VMCommand::Arithmetic(VMOp::ADD))));
        assert!(p.parse_str("pop foo bar").is_err());
        assert!(p.parse_str("push splat bar").is_err());
        assert_eq!(p.parse_str("push constant 33"), Ok(Some(VMCommand::Push(VMSeg::CONSTANT, 33))));
        assert_eq!(p.parse_str("pop local 3"), Ok(Some(VMCommand::Pop(VMSeg::LOCAL, 3))));
    }
}
