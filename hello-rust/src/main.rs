use std::io::{stdout, BufWriter};
use ferris_says::say;

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Restaceans!";
    let width = 20;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
