
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

fn get_diff(vals :Vec<i32>, p2 :bool) -> i32 {
    
    let mut zeros = 0;
    let mut nv = vec![0; vals.len() - 1];
    for i in 0..vals.len() - 1 {
        let diff = vals[i+1] - vals[i];
        nv[i] = diff; 
        if diff == 0 {
            zeros += 1;
        }
    }
    // println!("{:?}", nv);
    if zeros == nv.len() {
        return 0;
    }
    let v;
    if p2 {
        let a = nv[0].clone();
        let b = get_diff(nv, p2);
        v = a - b;
    } else {
        let a = nv.last().unwrap().clone();
        let b = get_diff(nv, p2);
        v = a + b;
    }
    // println!("{} + {} -> {}", a, b, v);
    v
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut sum = 0;
    for l in lines {
        let v : Vec<i32> = l.split(" ").map(|v| v.parse::<i32>().unwrap()).collect();
        let a = v.last().unwrap().clone();
        let d : i64 = get_diff(v, false) as i64;
        // println!("");
        sum += a as i64 + d;
    }
    sum
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut sum = 0;
    for l in lines {
        let v : Vec<i32> = l.split(" ").map(|v| v.parse::<i32>().unwrap()).collect();
        let a = v[0];
        let d : i64 = get_diff(v, true) as i64;
        // println!("");
        sum += a as i64 - d;
    }
    sum
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