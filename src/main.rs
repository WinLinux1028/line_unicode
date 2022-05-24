use rand::prelude::SliceRandom;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut v: Vec<u32> = (0x0300..=0x036F).collect();
    let mut v1: Vec<u32> = (0x1DC0..=0x1DFF).collect();
    v.append(&mut v1);
    let mut v2: Vec<u32> = (0x1AB0..=0x1ACE).collect();
    v.append(&mut v2);
    let mut v3: Vec<u32> = (0x20D0..=0x20F0).collect();
    v.append(&mut v3);
    let mut v4: Vec<u32> = (0xFE20..=0xFE2F).collect();
    v.append(&mut v4);
    let mut v: Vec<char> = unsafe { std::mem::transmute(v) };

    let mut a = String::new();
    while 20400 >= a.len() {
        v.shuffle(&mut OsRng);
        a.push_str("🇷🇺");
        for i in &v {
            a.push(*i);
        }
    }
    let mut output = File::create("./result.txt").unwrap();
    output.write_all(a.as_bytes()).unwrap();
}
