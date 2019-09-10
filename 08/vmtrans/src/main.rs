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
    let parser = Parser::new(&infile_path);

    // Need to not just unwrap the errors.  Refactor.
    let cmds = rdr.lines()
        .map(|x| parser.parse_str(&x.unwrap()).unwrap());

    let mut tr = Translator::new(&base);
    for cmd in cmds {
        let asm = tr.trans_cmd(cmd.unwrap());
        write!(&mut outfile, "{}", asm).unwrap();
    }
    
    Ok(())
}
