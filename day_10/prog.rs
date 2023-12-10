
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

fn get_next(lines :&Vec<String>, pos: (i32, i32), last: (i32, i32)) -> (i32, i32)
{
    match lines[pos.1 as usize].chars().nth(pos.0 as usize).unwrap() {
        'S' => {
            if last.0 == -1 && last.1 == -1 {
                let mut ch;
                if pos.1 != 0 {
                    ch = lines[(pos.1 - 1) as usize].chars().nth(pos.0 as usize).unwrap();
                    if ch == '|' || ch == 'F' || ch == '7' {
                        return (pos.0, pos.1 - 1);
                    }
                }
                if pos.1 as usize != lines.len() {
                    ch = lines[(pos.1 + 1) as usize].chars().nth(pos.0 as usize).unwrap();
                    if ch == '|' || ch == 'J' || ch == 'L' {
                        return (pos.0, pos.1 + 1);
                    }
                }
                if pos.0 != 0 {
                    ch = lines[pos.1 as usize].chars().nth((pos.0 - 1) as usize).unwrap();
                    if ch == '-' || ch == 'F' || ch == 'L' {
                        return (pos.0 - 1, pos.1);
                    }
                }
                // figure this case out...
                {
                    ch = lines[pos.1 as usize].chars().nth((pos.0 + 1) as usize).unwrap();
                    if ch == '-' || ch == 'J' || ch == '7' {
                        return (pos.0 + 1, pos.1);
                    }
                }
            }
        },
        '-' => {
            if is_same(last, (pos.0 - 1, pos.1)) {
                return (pos.0 + 1, pos.1)
            } else {
                return (pos.0 - 1, pos.1)
            }
        },
        '|' => {
            if is_same(last, (pos.0, pos.1 - 1)) {
                return (pos.0, pos.1 + 1)
            } else {
                return (pos.0, pos.1 - 1)
            }
        },
        'J' => {
            if is_same(last, (pos.0 - 1, pos.1)) {
                return (pos.0, pos.1 - 1)
            } else {
                return (pos.0 - 1, pos.1)
            }
        },
        'F' => {
            if is_same(last, (pos.0 + 1, pos.1)) {
                return (pos.0, pos.1 + 1)
            } else {
                return (pos.0 + 1, pos.1)
            }
        },
        'L' => {
            if is_same(last, (pos.0 + 1, pos.1)) {
                return (pos.0, pos.1 - 1)
            } else {
                return (pos.0 + 1, pos.1)
            }
        },
        '7' => {
            if is_same(last, (pos.0 - 1, pos.1)) {
                return (pos.0, pos.1 + 1)
            } else {
                return (pos.0 - 1, pos.1)
            }
        },
        _ => {},
    }
    (0,0)
}

fn is_same(pos1: (i32, i32), pos2: (i32, i32)) -> bool {
    pos1.0 == pos2.0 && pos1.1 == pos2.1
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut start_pos : (i32, i32) = (0,0);
    let mut y = 0;
    for l in lines {
        let mut x = 0;
        for c in l.chars() {
            if c == 'S' {
                start_pos = (x, y);
            }
            x += 1;
        }
        y += 1;
    }

    let mut steps = 1;
    let mut p = get_next(lines, start_pos, (-1, -1));
    let mut last_pos = start_pos;
    while !is_same(p, start_pos) {
        let np = get_next(lines, p, last_pos);
        last_pos = p;
        p = np;
        steps += 1;
    }

    steps / 2
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut start_pos : (i32, i32) = (0,0);
    let mut y = 0;
    for l in lines {
        let mut x = 0;
        for c in l.chars() {
            if c == 'S' {
                start_pos = (x, y);
            }
            x += 1;
        }
        y += 1;
    }

    let org :Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
    let mut sg :Vec<Vec<char>> = lines.iter().map(|l|l.chars().map(|_c| ' ').collect()).collect();
    let mut p = get_next(lines, start_pos, (-1, -1));
    sg[p.1 as usize][p.0 as usize] = org[p.1 as usize][p.0 as usize];
    let mut last_pos = start_pos;
    while !is_same(p, start_pos) {
        let np = get_next(lines, p, last_pos);
        last_pos = p;
        p = np;
        sg[p.1 as usize][p.0 as usize] = org[p.1 as usize][p.0 as usize];
    }

    let mut inside = 0;

    for l in 0..sg.len() {
        let ln = &mut sg[l];
        let mut hx = 0;
        let mut last = ' ';
        for c in 0..ln.len() {
            let v = ln[c];
            if v == '|' {
                hx += 1;
            } else if v == 'L' || v == '7' {
                if last == 'L' && v == '7' {
                    hx += 1;
                }
                last = v;
            } else if v == 'F' || v == 'J' {
                if last == 'F' && v == 'J' {
                    hx += 1;
                }
                last = v;
            } else if v == 'S' || v == '-' {
            } else {
                if hx % 2 != 0 {
                    inside += 1;
                    // ln[c] = 'I';
                }
            }
        }
    }

    inside
}

fn main() -> Result<(), Error>
{
    let inputs = vec![
        // "input_ex.txt", "input_ex2.txt", "input_ex3.txt", "input_ex4.txt", "input_ex5.txt", "input_ex6.txt",
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