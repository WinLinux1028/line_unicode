use rand::{rngs::OsRng, Rng};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut a = Vec::new();
    for _ in 0..10 {
        a.extend_from_slice("𰻞".as_bytes());
        for _ in 0..999 {
            a.push(205);
            a.push(OsRng.gen_range(128..=175));
        }
    }
    // 正当なUTF-8であることを確認
    let a = String::from_utf8(a).unwrap();

    let mut output = File::create("./result.txt").unwrap();
    output.write_all(a.as_bytes()).unwrap();
}
