use std::fmt;
use std::fmt::Write;

#[derive(Debug,PartialEq)]
pub enum Command {
    A(i32),
    ALabel(String),
    C(Dest, Comp, Jump),
}

impl Command {
    pub fn as_str(&self) -> String {
        match self {
            Command::A(n) => format!("@{}", n),
            Command::ALabel(label) => format!("@{}", label),
            Command::C(dest, comp, jump) => {
                let mut r = String::new();
                if dest != &Dest::Null {
                    write!(&mut r, "{}=", dest.as_str()).unwrap();
                }
                write!(&mut r, "{}", comp.as_str()).unwrap();
                if jump != &Jump::Null {
                    write!(&mut r, ";{}", jump.as_str()).unwrap();
                }
                r
            },
        }
    }
}


#[derive(Debug,PartialEq)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

impl Comp {
    pub fn from_str(s: &str) -> Option<Comp> {
        match s {
            "0" => Some(Comp::Zero),
            "1" => Some(Comp::One),
            "-1" => Some(Comp::MinusOne),
            "D" => Some(Comp::D),
            "A" => Some(Comp::A),
            "!D" => Some(Comp::NotD),
            "!A" => Some(Comp::NotA),
            "-D" => Some(Comp::MinusD),
            "-A" => Some(Comp::MinusA),
            "D+1" => Some(Comp::DPlusOne),
            "A+1" => Some(Comp::APlusOne),
            "D-1" => Some(Comp::DMinusOne),
            "A-1" => Some(Comp::AMinusOne),
            "D+A" => Some(Comp::DPlusA),
            "D-A" => Some(Comp::DMinusA),
            "A-D" => Some(Comp::AMinusD),
            "D&A" => Some(Comp::DAndA),
            "D|A" => Some(Comp::DOrA),
            "M" => Some(Comp::M),
            "!M" => Some(Comp::NotM),
            "-M" => Some(Comp::MinusM),
            "M+1" => Some(Comp::MPlusOne),
            "M-1" => Some(Comp::MMinusOne),
            "D+M" => Some(Comp::DPlusM),
            "D-M" => Some(Comp::DMinusM),
            "M-D" => Some(Comp::MMinusD),
            "D&M" => Some(Comp::DAndM),
            "D|M" => Some(Comp::DOrM),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Comp::Zero => "0",
            Comp::One => "1",
            Comp::MinusOne => "-1",
            Comp::D => "D",
            Comp::A => "A",
            Comp::NotD => "notD",
            Comp::NotA => "notA",
            Comp::MinusD => "-D",
            Comp::MinusA => "-A",
            Comp::DPlusOne => "D+1",
            Comp::APlusOne => "A+1",
            Comp::DMinusOne => "D-1",
            Comp::AMinusOne => "A-1",
            Comp::DPlusA => "D+A",
            Comp::DMinusA => "D-A",
            Comp::AMinusD => "A-D",
            Comp::DAndA => "DandA",
            Comp::DOrA => "DorA",
            Comp::M => "M",
            Comp::NotM => "notM",
            Comp::MinusM => "-M",
            Comp::MPlusOne => "M+1",
            Comp::MMinusOne => "M-1",
            Comp::DPlusM => "D+M",
            Comp::DMinusM => "D-M",
            Comp::MMinusD => "M-D",
            Comp::DAndM => "DandM",
            Comp::DOrM => "DorM",
        }

    }
}


#[derive(Debug,PartialEq)]
pub enum Dest {
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

impl Dest {
    pub fn from_str(s: &str) -> Option<Dest> {
        match s {
            "" => Some(Dest::Null),
            "M" => Some(Dest::M),
            "D" => Some(Dest::D),
            "MD" => Some(Dest::MD),
            "A" => Some(Dest::A),
            "AM" => Some(Dest::AM),
            "AD" => Some(Dest::AD),
            "AMD" => Some(Dest::AMD),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Dest::Null => "",
            Dest::M => "M",
            Dest::D => "D",
            Dest::MD => "MD",
            Dest::A => "A",
            Dest::AM => "AM",
            Dest::AD => "AD",
            Dest::AMD => "AMD",
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Jump {
    pub fn from_str(s: &str) -> Option<Jump> {
        match s {
            "" => Some(Jump::Null),
            "JGT" => Some(Jump::JGT),
            "JEQ" => Some(Jump::JEQ),
            "JGE" => Some(Jump::JGE),
            "JLT" => Some(Jump::JLT),
            "JNE" => Some(Jump::JNE),
            "JLE" => Some(Jump::JLE),
            "JMP" => Some(Jump::JMP),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Jump::Null => "",
            Jump::JGT => "JGT",
            Jump::JEQ => "JEQ",
            Jump::JGE => "JGE",
            Jump::JLT => "JLT",
            Jump::JNE => "JNE",
            Jump::JLE => "JLE",
            Jump::JMP => "JMP",
        }
    }
}

#[derive(PartialEq)]
pub struct ParserError {
    code: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {}", self.code)
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {}", self.code)
    }
}


pub fn parse_cmd(st: &str) -> Result<Option<Command>,ParserError> {
    let mut s: &str = &st.replace(" ","");
    let f = s.split("//").collect::<Vec<_>>();
    s = &f[0];
    if s.len() == 0 {
        return Ok(None)
    }
    if s.starts_with("@") {
        let rest = &s[1..];
        if rest.len() == 0 {
            Ok(None)
        } else if let Ok(n) = rest.parse::<i32>() {
            Ok(Some(Command::A(n)))
        } else {
            Ok(Some(Command::ALabel(rest.to_string())))
        }
    } else {
        let mut jump = Jump::Null;
        let f = s.split(";").collect::<Vec<_>>();
        if f.len() == 2 {
            if let Some(j) = Jump::from_str(f[1]) {
                jump = j;
            } else {
                return Err(ParserError{code: st.to_string()});
            }
        }
        s = &f[0];
        let mut dest = Dest::Null;
        let f = s.split("=").collect::<Vec<_>>();
        if f.len() == 2 {
            if let Some(d) = Dest::from_str(f[0]) {
                dest = d;
            } else {
                return Err(ParserError{code: st.to_string()});
            }

            s = &f[1];
        }
        if let Some(c) = Comp::from_str(s) {
            Ok(Some(Command::C(dest, c, jump)))
        } else {
            Err(ParserError{code: s.to_string()})
        }
    }
}

pub fn parse_code_str(code: &str) -> Result<Vec<Command>, ParserError> {
    let mut r: Vec<Command> = vec![];
    for line in code.lines() {
        let op_c = parse_cmd(line)?;
        if let Some(c) = op_c {
            r.push(c);
        }
    }
    Ok(r)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(parse_cmd(""), Ok(None));
        assert_eq!(parse_cmd("// foo"), Ok(None));
        assert_eq!(parse_cmd("@23"), Ok(Some(Command::A(23))));
        assert_eq!(parse_cmd("@F34"), Ok(Some(Command::ALabel("F34".to_string()))));
        assert_eq!(parse_cmd("M+1"), Ok(Some(Command::C(Dest::Null, Comp::MPlusOne, Jump::Null))));
        assert!(parse_cmd("M1").is_err());
        assert_eq!(parse_cmd("0;JMP"), Ok(Some(Command::C(Dest::Null, Comp::Zero, Jump::JMP))));
        assert_eq!(parse_cmd("AM=M+1;JGE"), Ok(Some(Command::C(Dest::AM, Comp::MPlusOne, Jump::JGE))));
    }

    #[test]
    fn test_parse_code_str() {
        assert_eq!(parse_code_str("@32\n// foo\n0;JMP\n"), Ok(vec![Command::A(32), Command::C(Dest::Null, Comp::Zero, Jump::JMP)]));
        assert!(parse_code_str("@32\nxyx\nM=1\n").is_err());
    }
}



