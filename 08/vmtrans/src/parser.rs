use crate::types::*;
use std::fmt;

pub struct Parser {
    file_name: String,
    line_num: i32,
}

#[derive(PartialEq)]
pub struct ParserError {
    file_name: String,
    line_num: i32,
    description: String,
    code: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: File: {}, Line: {},  Error: {}, Code: {}\n",
               self.file_name, self.line_num, self.description, self.code)
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: File: {}, Line: {},  Error: {}, Code: {}\n",
               self.file_name, self.line_num, self.description, self.code)
    }
}

impl Parser {
    pub fn new(fname: &str) -> Parser {
        Parser{file_name: fname.to_string(), line_num: 0}
    }

    fn parser_error(&self, desc: &str, code: &str) -> ParserError {
        ParserError{
            file_name: self.file_name.to_owned(),
            line_num: self.line_num,
            description: desc.to_string(),
            code: code.to_string()}
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
                        Err(self.parser_error("Invalid command format", cmd_str))
                    }
                } else {
                    Err(self.parser_error("Invalid segment name", cmd_str))
                }
            } else {
                Err(self.parser_error("Invalid command format", cmd_str))
            }
        } else if ws[0] == "pop" {
            if ws.len() == 3 {
                if let Some(seg) = VMSeg::from_str(ws[1]) {
                    if let Ok(n) = ws[2].parse::<i32>() {
                        Ok(Some(VMCommand::Pop(seg, n)))
                    } else {
                        Err(self.parser_error("Invalid command format", cmd_str))
                    }
                } else {
                    Err(self.parser_error("Invalid segment name", cmd_str))
                }
            } else {
                Err(self.parser_error("Invalid command format", cmd_str))
            }
        } else {
            Err(self.parser_error("Invalid command name", cmd_str))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_err(r: Result<Option<VMCommand>, ParserError>, desc: &str) -> bool {
        if let Err(pe) = r {
            pe.description == desc
        } else {
            false
        }
    }

    #[test]
    fn simple_parse_str_test() {
        let p = Parser::new("foo.vm");
        assert_eq!(p.parse_str(""), Ok(None));
        assert_eq!(p.parse_str("// foo bar"), Ok(None));
        assert_eq!(p.parse_str("add"), Ok(Some(VMCommand::Arithmetic(VMOp::ADD))));
        assert!(check_err(p.parse_str("pop foo 19"),"Invalid segment name"));
        assert!(check_err(p.parse_str("pop temp bar"),"Invalid command format"));
        assert!(check_err(p.parse_str("push splat 22"),"Invalid segment name"));
        assert!(check_err(p.parse_str("sblot temp 22"),"Invalid command name"));
        assert_eq!(p.parse_str("push constant 33"), Ok(Some(VMCommand::Push(VMSeg::CONSTANT, 33))));
        assert_eq!(p.parse_str("pop local 3"), Ok(Some(VMCommand::Pop(VMSeg::LOCAL, 3))));
    }
}
