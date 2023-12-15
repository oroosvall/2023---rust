
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

fn get_new(mut s:Vec<char>, start: usize, nums: &Vec<i64>, mut co: i64, mut idx: usize, cache: &mut HashMap<String, i64>) -> i64 {
    let mut n = 0;

    for i in start..s.len() {
        if s[i] == '#' {
            co += 1;
            if idx == nums.len() || co > nums[idx] {
                return n;
            }
        } else if s[i] == '.' && co != 0 {
            if idx == nums.len() || co != nums[idx] {
                return n;
            }
            idx += 1;
            co = 0;
        } else if s[i] == '?' {
            if co != 0 && idx != nums.len() && co < nums[idx] { // optimal path here is to keep adding #, no branch
                co += 1;
                s[i] = '#';
            } else if co == 0 { // need to branch
                s[i] = '.';
                let mut s1 = s.clone();
                s1[i] = '#';
                let ss = format!("{},{}",i,idx);
                // let ss: String = s1[i..].into_iter().collect();
                // println!("{:?}", ss);
                if ss.len() != 0 {
                    match cache.get(&ss) {
                        Some(nn) => {
                            // println!("{:?}, {}", ss, nn);
                            n += nn},
                        None => {
                            let nn = get_new(s1, i + 1, &nums, co + 1, idx, cache);
                            cache.insert(ss, nn);
                            n += nn;
                        }
                    }
                } else {
                    n += get_new(s1, i + 1, &nums, co + 1, idx, cache);
                }
            } else if idx != nums.len() && co == nums[idx] { // we have the max number of #, start over, no need to branch
                s[i] = '.';
                idx += 1;
                co = 0;
            } else { // we have fulfilled the list can only add .
                co = 0;
                s[i] = '.';
            }

        } else {
            // Do nothing
        }
    }

    co = 0;
    let mut v = vec!();

    for i in 0..s.len() {
        if s[i] == '#' {
            co += 1;
        } else if s[i] == '.' && co != 0 {
            v.push(co);
            co = 0;
        } else {
            // do nothing
        }
    }

    if co != 0 {
        v.push(co);
    }

    if v.len() == nums.len() {
        let mut ok = true;
        for (i, j) in v.iter().zip(nums.iter()) {
            if i != j {
                ok = false;
                break;
            }
        }
        if ok {
            n += 1;
        }
    }

    // if n != 0 {
    //     println!("{:?} - {}", s, n);
    // }

    n
}

fn get_line(l:&str) -> (Vec<char>, Vec<i64>) {
    let mut s = l.split(" ");
    let springs :Vec<char> = s.next().unwrap().chars().collect();
    let nums :Vec<i64> = s.next().unwrap().split(",").map(|i| i.parse::<i64>().unwrap()).collect();

    (springs, nums)
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut sum = 0;
    for l in lines {
        // println!("{:?}", l);
        let (s, n) = get_line(l);
        sum += get_new(s, 0, &n, 0,0, &mut HashMap::new());
    }
    sum
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut sum = 0;
    for l in lines {
        let (s, n) = get_line(l);
        let mut ss = vec!();
        let mut nn = vec!();
        for i in 0..5 {
            ss.extend(s.clone());
            nn.extend(n.clone());
            if i != 4 {
                ss.push('?');
            }
        }

        sum += get_new(ss, 0, &nn, 0, 0, &mut HashMap::new());
    }
    sum
}

fn main() -> Result<(), Error>
{
    let inputs = vec![
        //"input_ex.txt", //"input_ex2.txt", "input_ex3.txt", "input_ex4.txt", "input_ex5.txt", "input_ex6.txt",
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