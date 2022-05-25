use rand::prelude::SliceRandom;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut v: Vec<u32> = (0x0300..=0x0357).collect();
    v.append(&mut (0x0359..=0x036F).collect());
    v.append(&mut (0x1DC0..=0x1DF5).collect());
    v.append(&mut (0x1DFC..=0x1DFF).collect());
    v.append(&mut (0x1AB0..=0x1ABE).collect());
    v.append(&mut (0xFE20..=0xFE2D).collect());
    v.append(&mut (0x20D0..=0x20DE).collect());
    v.append(&mut (0x20E1..=0x20E2).collect());
    v.append(&mut (0x20E5..=0x20F0).collect());
    let mut v: Vec<char> = unsafe { std::mem::transmute(v) };

    let mut a = String::from("🇷🇺");
    for _ in 0..45 {
        v.shuffle(&mut OsRng);
        for i in &v {
            a.push(*i);
        }
    }
    let mut output = File::create("./result.txt").unwrap();
    output.write_all(a.as_bytes()).unwrap();
}
