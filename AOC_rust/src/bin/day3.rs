use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    //let answer1 = day3_puz1("input_day3.txt")?;
    //let answer2 = day3_puz2("input_day3.txt")?;
    print!("{}\n", answer1);
    //print!("{}\n", answer2);

    Ok(())
}

fn day3_puz1(file_name: &str) -> io::Result<u32> {
    let lines = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .into_iter();

    let mut results: [i32;12] = [0;12];
    for line in lines {
        let bytes = line.as_bytes();
        for i in 0..line.len() {
            results[i] += if bytes[i] as char == '1' { 1 } else {-1};
        }
    }

    let epsilon = results.iter()
        .rev()
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .into_iter().filter(|(_, x)| x > &&0)
        .fold(0, |acc, (i, _)| acc + (u32::pow(2, i as u32) as i32)) as u32;

    let gamma = !epsilon & 4095;
    
    Ok((gamma * epsilon) as u32)
}