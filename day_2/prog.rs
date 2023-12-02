
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

fn game_id(game :&str) -> i32
{
    let s : String = game.split(" ").skip(1).next().unwrap().to_string();
    s.parse::<i32>().unwrap()
}

fn get_cube_count(cubes :&str) -> (i32, &str)
{
    let mut m = cubes.split(" ");
    (m.next().unwrap().to_string().parse::<i32>().unwrap(), m.next().unwrap())
}

fn is_impossible(cubes :&str, r :i32, g :i32, b :i32) -> bool
{
    for s in cubes.split("; ")
    {
        let mut exit = false;

        for p in s.split(", ")
        {
            let (count, t) = get_cube_count(p);
            // println!("{}, {}", count, t);
            match t {
                "red" => {
                    exit = exit || count > r;
                },
                "green" => {
                    exit = exit || count > g;
                },
                "blue" => {
                    exit = exit || count > b;
                },
                _ => {}
            }
        }

        if exit {
            return true
        }
    }
    false
}

fn part_1(lines :&Vec<String>) -> i32
{
    let mut sum = 0;
    for l in lines {
        let mut it = l.split(": ");
        let gameid = game_id(it.next().unwrap());
        if is_impossible(it.next().unwrap(), 12, 13, 14) == false
        {
            sum += gameid;
        }
    }
    sum
}

fn get_game_cube_count_value(cubes :&str) -> i32
{
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for s in cubes.split("; ")
    {
        for p in s.split(", ")
        {
            let (count, t) = get_cube_count(p);
            match t {
                "red" => {
                    r = r.max(count);
                },
                "green" => {
                    g = g.max(count);
                },
                "blue" => {
                    b = b.max(count);
                },
                _ => {}
            }
        }
    }

    return r * g * b;
}

fn part_2(lines :&Vec<String> ) -> i32
{
    let mut sum = 0;
    for l in lines {
        sum += get_game_cube_count_value(l.split(": ").skip(1).next().unwrap());
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