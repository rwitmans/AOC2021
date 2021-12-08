use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let answer1 = day1_puz1("input_day1.txt")?;
    let answer2 = day1_puz2("input_day1.txt")?;
    print!("{}\n", answer1);
    print!("{}\n", answer2);

    Ok(())
}

fn day1_puz1(file_name: &str) -> io::Result<u32> {
    let sum: u32 = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap)
        .map(|e| e.parse::<u32>())
        .map(Result::unwrap)
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|curr| (curr[0] < curr[1]) as u32)
        .sum();

    Ok(sum)
}

fn day1_puz2(file_name: &str) -> io::Result<u32> {
    let sum: u32 = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap)
        .map(|e| e.parse::<u32>())
        .map(Result::unwrap)
        .collect::<Vec<u32>>()
        .windows(4)
        .map(|curr| (curr[0] + curr[1] + curr[2] < curr[1] + curr[2] + curr[3]) as u32)
        .sum();

    Ok(sum)
}