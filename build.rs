extern crate phf_codegen;
extern crate skeptic;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// Normally you would want to load an external data file instead of defining the map in Rust code
fn build_phf() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("phf.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, Keyword2> = \n{};\n",
        phf_codegen::Map::new()
            .entry("loop", "Keyword2::Loop")
            .entry("continue", "Keyword2::Continue")
            .entry("break", "Keyword2::Break")
            .entry("fn", "Keyword2::Fn")
            .entry("extern", "Keyword2::Extern")
            .build()
    ).unwrap();
}

fn main() {
    // generates doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);

    build_phf();
}
