use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let answer1 = day1_puz1("input_day1.txt")?;
    let answer2 = day1_puz2("input_day1.txt")?;
    let answer3 = day2_puz1("input_day2.txt")?;
    let answer4 = day2_puz2("input_day2.txt")?;
    let answer5 = day3_puz1("input_day3.txt")?;
    print!("{}\n", answer1);
    print!("{}\n", answer2);
    print!("{}\n", answer3);
    print!("{}\n", answer4);
    print!("{}\n", answer5);

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

fn day3_puz1(file_name: &str) -> io::Result<u32> {
    let lines = File::open(file_name)
        .map(BufReader::new)
        .map(BufReader::lines)?
        .into_iter()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .;

    for line in lines {
        for byte in line.chars() {
            print!("{}\n", byte)
        }
        //print!("{}\n", line);
    }
    Ok(0)
}