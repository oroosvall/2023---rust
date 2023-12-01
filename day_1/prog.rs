
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::time::{Instant};

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn part_1(vec :&Vec<String>) -> i32
{
    let mut num_vec : Vec<i32> = vec!();
    for l in vec {
        let numbers : Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
        let len = numbers.len();
        if len != 0 {
            let (first, second) = (numbers[0].to_string().parse::<i32>().unwrap() , numbers[len - 1].to_string().parse::<i32>().unwrap());
            // println!("first {}, second {}", first, second);
            num_vec.push((first * 10) + second);
        }
    }
    
    num_vec.iter().sum()
}

fn to_numbers(s :&String) -> Vec<char>
{
    let mut v = vec!();
    let len = s.len();

    let to_test = vec!["one", "two", "three", "four","five", "six", "seven", "eight", "nine",
                        "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for i in 0..len {
        for t in 0..to_test.len() {
            let m = &to_test[t];
            // println!("{} {}, {} - {}", m.len(), m, len, i);
            if m.len() > (len - i) {
                continue;
            }
            let val = &s[i..i+m.len()];
            if val == *m {
                // println!("{}", val);
                if t >= 9 { // if match index is over 8 it is a normal digit
                
                    v.push(val.chars().next().unwrap());
                }
                else {
                    v.push(to_test[t + 9].chars().next().unwrap());
                }
            }
        }
    }

    // println!("{:?}", v);
    v
}

fn part_2(vec :&Vec<String> ) -> i32
{
    let mut num_vec : Vec<i32> = vec!();

    for l in vec {
        let numbers = to_numbers(l);
        let len = numbers.len();
        let (first, second) = (numbers[0].to_string().parse::<i32>().unwrap() , numbers[len - 1].to_string().parse::<i32>().unwrap());
        // println!("first {}, second {}", first, second);
        num_vec.push((first * 10) + second);
    }

    num_vec.iter().sum()
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