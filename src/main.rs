pub mod arg_parsing;

use arg_parsing::parse_args;

fn main() {
    let args = parse_args();
    dbg!(args).unwrap();
}
