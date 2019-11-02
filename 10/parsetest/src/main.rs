use std::io;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug,Clone)]
enum Token {
    Symbol(char),
    Keyword(String),
    Identifier(String),
    IntConst(u16),
    StringConst(String),
}

fn is_keyword(w: &str) -> bool {
    match w {
        "if" | "else" | "do" | "let" => true,
        _ => false,
    }
}

fn skip_whitespace(rdr: &mut Peekable<Chars>) {
    while let Some(p) = rdr.peek() {
        match *p {
            ' ' | '\n' | '\t' => { rdr.next();},
            _ => {break;},
        }
    }
}

fn is_symbol(c: char) -> bool {
    (!c.is_alphanumeric() || c == '_')
}

fn get_word(rdr: &mut Peekable<Chars>) -> String {
    let mut w = String::new();
    while let Some(p) = rdr.peek() {
        if is_symbol(*p) || (*p).is_whitespace() {
            // &&& not exactly right?
            break;
        }
        w.push(*p);
        rdr.next();
    }
    w
}

fn get_int(rdr: &mut Peekable<Chars>) -> u16 {
    let w = get_word(rdr);
    w.parse::<u16>().expect("Invalid u16")
}

fn get_string_const(rdr: &mut Peekable<Chars>) -> String {
    let mut s = String::new();
    while let Some(p) = rdr.peek() {
        let c = *p;
        rdr.next();
        if c == '"' {
            break;
        }
        s.push(c);
    }
    s
}

fn get_token(rdr: &mut Peekable<Chars>) -> Option<Token> {
    skip_whitespace(rdr);
    if let Some(p) = rdr.peek() {
        let c = *p;
        if c == '"' {
            rdr.next();
            Some(Token::StringConst(get_string_const(rdr)))
        } else if is_symbol(c) {
            rdr.next();
            Some(Token::Symbol(c))
        } else {
            if c.is_digit(10) {
                Some(Token::IntConst(get_int(rdr)))
            } else {
                let w = get_word(rdr);
                if is_keyword(&w) {
                    Some(Token::Keyword(w))
                } else {
                    Some(Token::Identifier(w))
                }
            }
        }
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let st = "if (a < 3) { let b = 3; } ".to_string();
    let mut rdr = st.chars().peekable();
    while let Some(t) = get_token(&mut rdr) {
        println!("{:?}", t);
    }
    println!("{:?}", rdr.peek());
    Ok(())
}
