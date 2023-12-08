
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};
use std::collections::HashMap;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut hm : HashMap<&str,(String,String)> = HashMap::new();
    let moves : Vec<char> = lines.iter().next().unwrap().chars().collect();

    for l in lines.iter().skip(2)
    {
        let mut s = l.split("=");
        let key = s.next().unwrap().trim();

        let mut p = s.next().unwrap().split(",");
        let p1 = p.next().unwrap().replace("(", " ");
        let p2 = p.next().unwrap().replace(")", " ");

        let np1 = p1.trim().to_string();
        let np2 = p2.trim().to_string();

        // println!("{:?} = ({:?}, {:?})", key, np1, np2);
        hm.insert(key, (np1, np2));
    }

    let mut v = "AAA";
    let mut steps = 0;

    while v != "ZZZ" {
        for ch in &moves {
            if ch == &'L' {
                v = &hm[v].0;
            } else {
                v = &hm[v].1;
            }
            steps += 1;
            if v == "ZZZ" {
                break;
            }
        }
    }

    steps
}

fn gcd(a: i64, b: i64) -> i64 {
    // println!("gcd {}, {}", a, b);
    let mut r = a.min(b);
    while r > 0 {
        if (a % r == 0) && (b % r == 0) {
            break;
        }
        r -= 1;
    }
    r
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut hm : HashMap<&str,(String,String)> = HashMap::new();
    let moves : Vec<char> = lines.iter().next().unwrap().chars().collect();

    for l in lines.iter().skip(2)
    {
        let mut s = l.split("=");
        let key = s.next().unwrap().trim();

        let mut p = s.next().unwrap().split(",");
        let p1 = p.next().unwrap().replace("(", " ");
        let p2 = p.next().unwrap().replace(")", " ");

        let np1 = p1.trim().to_string();
        let np2 = p2.trim().to_string();

        // println!("{:?} = ({:?}, {:?})", key, np1, np2);
        hm.insert(key, (np1, np2));
    }

    let mut values : Vec<(String, bool, i32)> = vec!();
    let mut ends_in_z = 0;

    for k in hm.keys() {
        if k.ends_with("A") {
            values.push((k.to_string(), false, 0));
        }
    }

    // println!("{}", values.len());

    while ends_in_z != values.len() {
        for ch in &moves {
            for i in 0..values.len() {
                let v = &mut values[i];
                if v.1 == false {
                    if ch == &'L' {
                        v.0 = hm[&v.0 as &str].0.clone();
                    } else {
                        v.0 = hm[&v.0 as &str].1.clone();
                    }
                    v.2 += 1;
                    if v.0.ends_with("Z") {
                        v.1 = true;
                    }
                } else {
                    ends_in_z += 1
                }
            }
            if ends_in_z == values.len() {
                break;
            }
            ends_in_z = 0;
        }
    }

    // Shit ton of assumptions below this line
    let g = gcd(values[0].2 as i64, values[1].2 as i64);

    let mut v = values[0].2 as i64;
    for i in 1..values.len() {
        let q = values[i].2 as i64 / g;
        v *= q;
    }

    v
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