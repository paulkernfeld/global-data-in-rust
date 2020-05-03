extern crate phf;

#[derive(Clone, Debug, PartialEq)]
enum Keyword2 {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/phf.rs"));

fn main() {
    assert_eq!(KEYWORDS.get("loop"), Some(&crate::Keyword2::Loop))
}
