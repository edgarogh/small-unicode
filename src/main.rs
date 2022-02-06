use std::fmt::{Display, Formatter};
use std::process::exit;

const ERROR_OUT_OF_RANGE: &str = "out-of-range character";

enum UnicodeName {
    Named(unicode_names2::Name),
    Control,
}

impl Display for UnicodeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(name) => Display::fmt(name, f),
            Self::Control => write!(f, "<control>"),
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    assert_eq!(args.next().as_deref(), Some("-d"), "expected 2nd argument to be `-d`");
    let codepoint = args.next().expect("missing second argument");
    let codepoint = codepoint.parse::<u32>().expect("second argument should be decimal");
    let char = char::from_u32(codepoint).expect(ERROR_OUT_OF_RANGE);
    assert_eq!(args.next().as_deref(), Some("--brief"), "expected 2nd argument to be `--brief`");

    let name = (codepoint < 32)
        .then(|| UnicodeName::Control)
        .or_else(|| unicode_names2::name(char).map(UnicodeName::Named));

    if let Some(name) = name {
        println!("{char} U+{codepoint:0>4X} {name}");
    } else {
        eprintln!("{}", ERROR_OUT_OF_RANGE);
        exit(1);
    }
}
