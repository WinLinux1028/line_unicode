use rand::prelude::SliceRandom;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut v: Vec<u8> = (128..=175).collect();

    let mut a = String::new();
    while 20400 >= a.len() {
        v.shuffle(&mut OsRng);
        a.push_str("🇷🇺");
        for i in &v {
            a.push_str(&String::from_utf8(vec![205, *i]).unwrap());
        }
    }
    let mut output = File::create("./result.txt").unwrap();
    output.write_all(a.as_bytes()).unwrap();
}
