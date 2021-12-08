use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let answer1 = day2_puz1("input_day2.txt")?;
    let answer2 = day2_puz2("input_day2.txt")?;
    print!("{}\n", answer1);
    print!("{}\n", answer2);

    Ok(())
}

fn day2_puz1(file_name: &str) -> io::Result<i32> {
    let lines = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap);
    
    let mut hor = 0;
    let mut depth = 0;
    for line in lines {
        let x = line.split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        if line.contains("forward") {
            hor += x;
        } else if line.contains("up") {
            depth -= x;
        } else if line.contains("down") {
            depth += x;
        }
    }

    Ok(hor*depth)
}

fn day2_puz2(file_name: &str) -> io::Result<i32> {
    let lines = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap);
    
    let mut aim = 0;
    let mut hor = 0;
    let mut depth = 0;
    for line in lines {
        let x = line.split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        if line.contains("forward") {
            hor += x;
            depth += aim*x;
        } else if line.contains("up") {
            aim -= x;
        } else if line.contains("down") {
            aim += x;
        }
    }

    Ok(hor*depth)
}