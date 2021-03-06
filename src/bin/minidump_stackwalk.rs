// Copyright 2015 Ted Mielczarek. See the COPYRIGHT
// file at the top-level directory of this distribution.

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;

extern crate breakpad_symbols;
extern crate minidump;

use breakpad_symbols::{SimpleSymbolSupplier,Symbolizer};
use minidump::*;

const USAGE : &'static str =
    "usage: minidump_stackwalk <minidump-file> [symbol-path ...]";

fn print_minidump_process(path : &Path, symbol_paths : Vec<PathBuf>) {
    let mut stderr = std::io::stderr();
    if let Ok(mut dump) = Minidump::read_path(path) {
        match process_minidump(&mut dump,
                               &Symbolizer::new(
                                   SimpleSymbolSupplier::new(symbol_paths))) {
            Ok(state) => {
                let mut stdout = std::io::stdout();
                state.print(&mut stdout).unwrap();
            },
            Err(err) => {
                writeln!(&mut stderr, "Error processing dump: {:?}", err).unwrap();
            }
        }
    } else {
        writeln!(&mut stderr, "Error reading dump").unwrap();
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    if let Some(arg) = env::args().nth(1) {
        let path = Path::new(&arg);
        let symbol_paths = env::args().skip(2).map(|a| PathBuf::from(&a)).collect();
        print_minidump_process(&path, symbol_paths);
    } else {
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "{}", USAGE).unwrap();
    }
}
