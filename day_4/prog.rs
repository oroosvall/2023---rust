
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

fn card_id(game :&str) -> i32
{
    let s : String = game.split(" ").last().unwrap().to_string();
    s.parse::<i32>().unwrap()
}

fn get_numbers(nums :&str) -> Vec<i32>
{
    let mut nrs = vec!();
    for n in nums.split(" ") {
        match n.to_string().parse::<i32>()
        {
            Ok(nr) => {nrs.push(nr)},
            Err(_) => {}
        }
    }
    nrs
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut sum = 0;
    for l in lines {
        let it = l.split(":");
        // let cardid = game_id(it.next().unwrap());
        let mut it2 = it.skip(1).next().unwrap().split("|");
        let winnings = get_numbers(it2.next().unwrap());
        let yours = get_numbers(it2.next().unwrap());
        let mut worht = 0;
        for nr in yours {
            if winnings.contains(&nr){
                if worht == 0{
                    worht = 1;
                } else {
                    worht *= 2;
                }
            }
        }
        sum += worht;
    }
    sum
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut card_stack = vec![1; lines.len()];
    
    for l in lines {
        let mut it = l.split(":");
        let cardid = card_id(it.next().unwrap()) - 1;
        let mut it2 = it.next().unwrap().split("|");
        let winnings = get_numbers(it2.next().unwrap());
        let yours = get_numbers(it2.next().unwrap());
        let mut idx = 0;
        for nr in yours {
            if winnings.contains(&nr) {
                idx += 1;
                if ((cardid + idx) as usize) < card_stack.len() {
                    card_stack[(cardid + idx) as usize] += card_stack[cardid as usize];
                }
            }
        }
    }
    // println!("{:?}", card_stack);

    card_stack.into_iter().sum::<i64>()
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