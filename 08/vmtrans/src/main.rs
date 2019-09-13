// vmtrans.rs
//
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

use vmtrans::translator::Translator;
use vmtrans::parser::Parser;

fn main() -> Result<(), std::io::Error> {
    let base = std::env::args().nth(1).expect("usage: $0 <fname.vm>");
    let infile_path = format!("{}.vm", base);
    let outfile_path = format!("{}.asm", base);
    let infile = File::open(infile_path.to_string())?;
    let rdr = BufReader::new(&infile);

    let mut outfile = File::create(outfile_path)?;
    let mut parser = Parser::new(&infile_path);

    let mut tr = Translator::new(&base);
    for some_line in rdr.lines() {
        let line = some_line.unwrap();
        match parser.parse_str(&line) {
            Ok(Some(cmd)) => {
                let asm = tr.trans_cmd(cmd);
                write!(&mut outfile, "{}", asm).unwrap();
            },
            Ok(None) => {},  // comment or whitespace
            Err(e) => {
                println!("ERROR: {:?}", e);
            },
        }
    }
    
    Ok(())
}
