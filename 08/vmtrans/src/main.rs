// vmtrans.rs
//
use std::fs::{File,read_dir};
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

use vmtrans::translator::Translator;
use vmtrans::parser::Parser;

fn process_file(base: &str, inpath: &Path, mut outfile: &File) -> Result<(), std::io::Error> {
    let infile = File::open(inpath)?;
    let rdr = BufReader::new(&infile);

    let mut parser = Parser::new(base);

    let mut tr = Translator::new(base);
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

fn main() -> Result<(), std::io::Error> {
    // If called with a single file, generate ASM for that file (with no bootstrap).
    // If called with a directory name -- generate ASM for *.vm in that directory,
    // and generate bootstrap code which sets SP and calls Sys.init
    let arg = std::env::args().nth(1).expect("usage: $0 <fname.vm|dirname>");
    let inpath = Path::new(&arg);
    let base = inpath.file_stem().unwrap().to_string_lossy().into_owned();
    let outfile_name = format!("{}.asm", &base);
    let mut outfile = File::create(outfile_name)?;

    if inpath.is_file() {
        if inpath.extension().unwrap() == "vm" {
            process_file(&base, inpath, &mut outfile)?;
        } else {
            panic!("Input is a file and does not have a .vm extension");
        }
    } else if inpath.is_dir() {
        write!(&mut outfile, "{}", Translator::gen_bootstrap()).unwrap();
        for entry in read_dir(inpath)? {
            let entry = entry?;
            let ep = entry.path();
            if ep.is_file() {
                if ep.extension().unwrap() == "vm" {
                    let base = ep.file_stem().unwrap().to_string_lossy().into_owned();
                    process_file(&base, &ep, &mut outfile)?;
                }
            }
        }
    } else {
        panic!("Input is neither file nor directory");
    }
    
    Ok(())
}
