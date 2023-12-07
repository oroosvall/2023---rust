
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};
use std::collections::HashMap;
use std::cmp::Ordering;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn get_type(s :&str, p2 :bool) -> Type {

    let mut hm :HashMap<char, i32>= HashMap::new();
    for c in s.chars() {
        hm.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut t = match hm.keys().len() {
        1 => Type::Five,
        2 => {
            let Some((_, count)) = hm.iter().next() else {todo!()};
            if *count == 4 || *count == 1 {
                Type::Four
            } else {
                Type::FullHouse
            }
        },
        3 => {
            let mut max : i32 = 0;
            for (_, count) in hm.iter() {
                max = max.max(*count);
            }
            if max == 3 {
                Type::Three
            } else {
                Type::TwoPair
            }
        },
        4 => Type::OnePair,
        5 => Type::HighCard,
        _ => todo!(),
    };

    if p2 && hm.contains_key(&'J') {
        t = match t {
            Type::HighCard => Type::OnePair,
            Type::OnePair => Type::Three,
            Type::TwoPair => {
                // full house or four of a kind
                if hm[&'J'] == 1 {
                    Type::FullHouse
                } else {
                    Type::Four
                }
            }
            Type::Three => Type::Four,
            Type::FullHouse => Type::Five,
            Type::Four => Type::Five,
            Type::Five => Type::Five,
        }
    }

    // println!("{:?} -> {:?}", hm, t);
    t
}

fn get_strength(ch :&char, p2 :bool) -> i32 {
    match *ch {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {if p2 {1} else {11}},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 9999
    }
}

fn cmp_char(ch1 :&char, ch2 :&char, p2 :bool) -> Ordering {
    get_strength(ch1,p2).cmp(&get_strength(ch2, p2))
}

fn sort_hand(l :&(&str, i64), r :&(&str, i64), p2 :bool) -> Ordering {
    
    let lt = get_type(l.0, p2);
    let rt = get_type(r.0, p2);

    let mut eq = lt.cmp(&rt);

    if eq == Ordering::Equal {
        for (l, r) in l.0.chars().zip(r.0.chars()) {
            eq = cmp_char(&l, &r, p2);
            if eq != Ordering::Equal {
                break;
            }
        }
    }

    eq
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut hands :Vec<(&str, i64)> = lines.into_iter().map(|l| {
        let mut s = l.split(" ");
        let name = s.next().unwrap();
        let bid = s.next().unwrap().parse::<i64>().unwrap();
        (name, bid)
    }).collect();
    
    hands.sort_by(|l, r| sort_hand(l, r, false));

    let mut winnings : i64 = 0;
    for (i,h) in hands.iter().enumerate() 
    {
        // println!("{:?}", h.0);
        winnings += h.1 * (i as i64+1);
    }

    winnings
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut hands :Vec<(&str, i64)> = lines.into_iter().map(|l| {
        let mut s = l.split(" ");
        let name = s.next().unwrap();
        let bid = s.next().unwrap().parse::<i64>().unwrap();
        (name, bid)
    }).collect();
    
    hands.sort_by(|l, r| sort_hand(l, r, true));

    let mut winnings : i64 = 0;
    for (i,h) in hands.iter().enumerate() 
    {
        winnings += h.1 * (i as i64+1);
    }

    winnings
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