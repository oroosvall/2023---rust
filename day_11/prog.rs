
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};
use std::collections::VecDeque;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn get_points(lines :&Vec<String>, c: usize) -> VecDeque<(i64, i64)> {
    let mut poses :VecDeque<(i64,i64)> = VecDeque::new();

    let sg: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    
    let mut yh = vec![0; sg[0].len()];

    for l in 0..sg.len() {
        let ln = &sg[l];
        for c in 0..ln.len() {
            if ln[c] == '#' {
                yh[c] += 1;
            }
        }
    }

    let mut py = 0;

    for y in 0..sg.len() {
        let ln = &sg[y];
        let mut px = 0;
        let mut xh = 0;

        for x in 0..ln.len() {
            let ch = ln[x];
            if yh[x] == 0 {
                px += c;
            }
            if ch == '#' {
                xh += 1;
                poses.push_back(((px + x) as i64, (py + y) as i64));
            }
        }
        if xh == 0 {
            py += c;
        }
    }

    poses
}

fn get_dist(p1: (i64, i64), p2: (i64,i64)) -> i64 {
    let a = (p1.0 - p2.0).abs();
    let b = (p1.1 - p2.1).abs();
    a + b
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut poses :VecDeque<(i64,i64)> = get_points(lines, 1);

    let mut sum_dist : i64 = 0;
    while !poses.is_empty() {
        let p1 = poses.pop_front().unwrap();
        for p2 in &poses {
            let d = get_dist(p1, *p2);
            // println!("{:?} - {:?} > {}", p1, *p2, d);
            sum_dist += d;
        }
    }

    sum_dist
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let one_m = 1000000;
    let mut poses :VecDeque<(i64,i64)> = get_points(lines, one_m - 1);

    let mut sum_dist : i64 = 0;
    while !poses.is_empty() {
        let p1 = poses.pop_front().unwrap();
        for p2 in &poses {
            let d = get_dist(p1, *p2);
            // println!("{:?} - {:?} > {}", p1, *p2, d);
            sum_dist += d;
        }
    }

    sum_dist
}

fn main() -> Result<(), Error>
{
    let inputs = vec![
        "input_ex.txt", //"input_ex2.txt", "input_ex3.txt", "input_ex4.txt", "input_ex5.txt", "input_ex6.txt",
        "input.txt"
    ];

    for i in inputs {
        let mut now = Instant::now();
        let input : Vec<String> = read_to_vec(File::open(i)?)?;
        println!("Read input: {} µs", now.elapsed().as_micros());

        now = Instant::now();
        let r1 = part_1(&input);
        println!("Part 1: {} µs", now.elapsed().as_micros());

        now = Instant::now();
        let r2 = part_2(&input);
        println!("Part 2: {} µs", now.elapsed().as_micros());

        println!("Result 1: {}\nResult 2: {}", r1, r2);
    }

    Ok(())
}