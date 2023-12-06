
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn part_1(lines :&Vec<String>) -> i32
{
    let mut time = vec!();
    let mut dist = vec!();
    for l in lines {
        let s = l.split(":");
        if time.is_empty() {
            for nrs in s.skip(1).next().unwrap().split(" ") {
                match nrs.parse::<i32>() {
                    Ok(n) => {time.push(n);},
                    Err(_) => {},
                }
            }
        } else {
            for nrs in s.skip(1).next().unwrap().split(" ") {
                match nrs.parse::<i32>() {
                    Ok(n) => {dist.push(n);},
                    Err(_) => {},
                }
            }
        }
    }

    let mut margin = vec!();
    for i in 0..time.len() {
        let mut oks = 0;
        for v in 0..time[i] {
            let d = v * (time[i]-v);
            if d > dist[i] {
                oks += 1;
            }
        }
        margin.push(oks);
    }

    margin.iter().product()
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut time = vec!();
    let mut dist = vec!();
    for l in lines {
        let s = l.split(":");
        if time.is_empty() {
            let nr = s.skip(1).next().unwrap().replace(" ", "");
            time.push(nr.parse::<i64>().unwrap());
        } else {
            let nr = s.skip(1).next().unwrap().replace(" ", "");
            dist.push(nr.parse::<i64>().unwrap());
        }
    }

    let mut margin = vec!();
    for i in 0..time.len() {
        let mut oks = 0;
        for v in 0..time[i] {
            let d = v * (time[i]-v);
            if d > dist[i] {
                oks += 1;
            }
        }
        margin.push(oks);
    }

    margin.iter().product()
}

fn main() -> Result<(), Error>
{
    let mut now = Instant::now();
    let input : Vec<String> = read_to_vec(File::open("input.txt")?)?;
    println!("Read input: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r1 = part_1(&input);
    println!("Part 1: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r2 = part_2(&input);
    println!("Part 2: {} µs", now.elapsed().as_micros());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}