// I put this in lib.rs because the path from skeptic was weird
pub const SAMPLE_STR: &'static str = include_str!("../sample.txt");
pub const SAMPLE_BYTES: &'static [u8] = include_bytes!("../sample.txt");

pub const SIX: i8 = 6;
pub const ALSO_SIX: i8 = include!("six.rs-snippet");
