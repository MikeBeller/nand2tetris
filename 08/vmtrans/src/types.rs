

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum VMOp {
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
    pub fn from_str(s: &str) -> Option<VMOp> {
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

    pub fn as_str(&self) -> &'static str {
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
pub enum VMSeg {
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
    pub fn from_str(s: &str) -> Option<VMSeg> {
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

    pub fn as_str(&self) -> &'static str {
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

    pub fn base_var_str(&self) -> &'static str {
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
pub enum VMCommand {
    Arithmetic(VMOp),
    Push(VMSeg, i32),
    Pop(VMSeg, i32),
}

