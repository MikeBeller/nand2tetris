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
    pub fn new (fname: &str) -> Parser {
        Parser{file_name: fname.to_string(), line_num: 0}
    }

    fn parser_error(&self, desc: &str, code: &str) -> ParserError {
        ParserError{
            file_name: self.file_name.to_owned(),
            line_num: self.line_num,
            description: desc.to_string(),
            code: code.to_string()}
    }

    pub fn parse_str(&mut self, cmd_str: &str) -> Result<Option<VMCommand>, ParserError> {
        self.line_num += 1;
        let ws: Vec<&str> = cmd_str.split_whitespace().collect();
        if ws.len() < 1 || ws[0] == "//" {
            Ok(None)
        } else if let Some(vmc) = VMOp::from_str(ws[0]) {
            Ok(Some(VMCommand::Arithmetic(vmc)))
        } else if ws[0] == "push" || ws[0] == "pop" {
            if ws.len() == 3 {
                if let Some(seg) = VMSeg::from_str(ws[1]) {
                    if let Ok(n) = ws[2].parse::<i32>() {
                        if ws[0] == "push" {
                            Ok(Some(VMCommand::Push(seg, n)))
                        } else {
                            Ok(Some(VMCommand::Pop(seg, n)))
                        }
                    } else {
                        Err(self.parser_error("Invalid command format", cmd_str))
                    }
                } else {
                    Err(self.parser_error("Invalid segment name", cmd_str))
                }
            } else {
                Err(self.parser_error("Invalid command format", cmd_str))
            }
        } else if ws[0] == "label" || ws[0] == "goto" || ws[0] == "if-goto" {
            if ws.len() == 2 {
                if ws[0] == "label" {
                    Ok(Some(VMCommand::Label(ws[1].to_string())))
                } else if ws[0] == "goto" {
                    Ok(Some(VMCommand::Goto(ws[1].to_string())))
                } else {
                    Ok(Some(VMCommand::IfGoto(ws[1].to_string())))
                }
            } else {
                Err(self.parser_error("Invalid command format", cmd_str))
            }
        } else if ws[0] == "function" || ws[0] == "call" {
            if ws.len() == 3 {
                if let Ok(n) = ws[2].parse::<i32>() {
                    if ws[0] == "function" {
                        Ok(Some(VMCommand::Function(ws[1].to_string(), n)))
                    } else {
                        Ok(Some(VMCommand::Call(ws[1].to_string(), n)))
                    }
                } else {
                    Err(self.parser_error("Invalid command format", cmd_str))
                }
            } else {
                Err(self.parser_error("Invalid command format", cmd_str))
            }
        } else if ws[0] == "return" {
            Ok(Some(VMCommand::Return))
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
        let mut p = Parser::new("foo.vm");
        assert_eq!(p.parse_str(""), Ok(None));
        assert_eq!(p.parse_str("// foo bar"), Ok(None));
        assert_eq!(p.parse_str("add"), Ok(Some(VMCommand::Arithmetic(VMOp::ADD))));
        assert!(check_err(p.parse_str("pop foo 19"),"Invalid segment name"));
        assert!(check_err(p.parse_str("pop temp bar"),"Invalid command format"));
        assert!(check_err(p.parse_str("push splat 22"),"Invalid segment name"));
        assert!(check_err(p.parse_str("sblot temp 22"),"Invalid command name"));
        assert_eq!(p.parse_str("push constant 33"), Ok(Some(VMCommand::Push(VMSeg::CONSTANT, 33))));
        assert_eq!(p.parse_str("pop local 3"), Ok(Some(VMCommand::Pop(VMSeg::LOCAL, 3))));
        assert_eq!(p.parse_str("label foo"), Ok(Some(VMCommand::Label("foo".to_string()))));
        assert_eq!(p.parse_str("goto foo"), Ok(Some(VMCommand::Goto("foo".to_string()))));
        assert_eq!(p.parse_str("if-goto foo"), Ok(Some(VMCommand::IfGoto("foo".to_string()))));
        assert_eq!(p.parse_str("function foo 2"), Ok(Some(VMCommand::Function("foo".to_string(), 2))));
        assert_eq!(p.parse_str("call foo 3"), Ok(Some(VMCommand::Call("foo".to_string(), 3))));
        assert_eq!(p.parse_str("return"), Ok(Some(VMCommand::Return)));
    }
}
