use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub mod rope;

const FIXNUM_MASK: u64 = 0b11;
const FIXNUM_TAG: u64 = 0b00;
const CHAR_TAG: u64 = 0b00001111;
const BOOL_TAG: u64 = 0b0011111;
const EMPTY_LIST: u64 = 0b00101111;

const TRUE_VAL: u64 = (1 << 7) | BOOL_TAG;
const FALSE_VAL: u64 = BOOL_TAG;

#[derive(Clone, Copy, Debug)]
enum SchemeVal {
    Fixnum(i32),
    Char(char),
    Bool(bool),
    EmptyList,
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: iacc <file.scm>");
        std::process::exit(64);
    }

    let name = args[1].split('.').next().unwrap();

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let val = s.trim_end().parse::<i32>().unwrap();

    let mut output = String::new();

    output.push_str(".globl _main\n");
    output.push_str("_main:\n");
    output.push_str(&format!("\tmov w0, #{}\n", val));
    output.push_str("\tret\n");

    let output_path = format!("{}.s", name);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(output.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn create_new_rating() {
        let actual = 2 + 2;
        let expected = expect!["4"]; // or expect![["5"]]
        expected.assert_eq(&actual.to_string())
    }
}
