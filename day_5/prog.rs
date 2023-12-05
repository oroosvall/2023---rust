
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

#[derive(Debug)]
struct Range {
    dst_start: i64,
    src_start: i64,
    count: i64
}

impl Range {
    fn from_str(data :&str) -> Range{

        let mut s = data.split(" ");
        let a = s.next().unwrap().parse::<i64>().unwrap();
        let b = s.next().unwrap().parse::<i64>().unwrap();
        let c = s.next().unwrap().parse::<i64>().unwrap();

        Range {dst_start:a, src_start:b, count:c}
    }
}

fn get_data(lines :&Vec<String>, part_2 :bool) -> (Vec<(i64, i64)>, HashMap<String, Vec<Range>>)
{
    let mut seeds : Vec<(i64, i64)> = vec!();
    let mut mappings : HashMap<String, Vec<Range>> = HashMap::new();
    
    let mut process_seeds = true;
    let mut name = "";
    for l in lines {
        if process_seeds {
            let mut s = l.split(":");
            name = s.next().unwrap();
            let nrs = s.next().unwrap();
            let mut tmp = -1;
            for nr in nrs.split(" ") {
                match nr.parse::<i64>() {
                    Ok(n) => {
                        if part_2 {
                            if tmp == -1 {
                                tmp = n;
                            } else {
                                seeds.push((tmp, n));
                                tmp = -1;
                            }     
                        } else {
                            seeds.push((n,1))
                        }
                    },
                    Err(_) => {},
                }
            }
            process_seeds = false;
        } else {
            if l.is_empty() {
                name = "";
            } else if name.is_empty() {
                name = l.split(":").next().unwrap();
                mappings.insert(name.to_string(), vec!());
            } else {
                let r = Range::from_str(l);
                mappings.entry(name.to_string()).and_modify(|range| range.push(r));
                mappings.entry(name.to_string()).and_modify(|r| r.sort_by_key(|k| k.src_start));
            }
        }
    }

    (seeds, mappings)
}

fn remap_part1(val :i64, ranges :&Vec<Range>) -> i64 {
    for r in ranges {
        if r.src_start <= val && val < r.src_start + r.count {
            let offset = val - r.src_start;
            return r.dst_start + offset;
        }
    }
    val
}

// our ranges are sorted, by src_start key
fn remap_part2(val :(i64, i64), ranges :&Vec<Range>) -> Vec<(i64, i64)> {
    let mut new_vals = vec!();

    let mut vstart = val.0;
    let vend = val.0 + val.1;

    for r in ranges {

        // this checks if seedrange is entierly outside of the range
        if vstart < r.src_start && vend < r.src_start {
            new_vals.push((vstart, vend - vstart));
            break; // should be safe to break here
        }
        // this cheks if our seed range starts outside mapp range
        // and then enters it
        if vstart < r.src_start && r.src_start <= vend {
            let outside_count = r.src_start - vstart;
            new_vals.push((vstart, outside_count));
            vstart = r.src_start;
        }
        // this will remap everything inside the range
        // but we might extend outside to the right, so we cannot copy everything
        if r.src_start <= vstart && vstart < r.src_start + r.count {
            let offset = vstart - r.src_start; // could be 0 or above
            let count; // this could extend outside range
            if r.src_start + r.count < vend {
                count = (r.src_start + r.count) - vstart;
            } else {
                count = vend - vstart;
            }

            new_vals.push((r.dst_start + offset, count));
            vstart = vstart + count;

            if vstart == vend {
                break;
            }
        }
    }

    if vstart != vend {
        new_vals.push((vstart, vend - vstart));
    }

    new_vals
}

fn part_1(lines :&Vec<String>) -> i64
{
    let (mut seeds, mappings) = get_data(lines, false);

    let order = vec!["seed-to-soil map", "soil-to-fertilizer map", "fertilizer-to-water map", "water-to-light map", "light-to-temperature map", "temperature-to-humidity map", "humidity-to-location map"];

    for o in order {
        for (v1, _v2) in seeds.iter_mut() {
            *v1 = remap_part1(*v1, &mappings[o]);
        }
    }

    let mut min = i64::MAX;
    for v in seeds {
        min = min.min(v.0);
    }
    min
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let (mut seeds, mappings) = get_data(lines, true);

    let order = vec!["seed-to-soil map", "soil-to-fertilizer map", "fertilizer-to-water map", "water-to-light map", "light-to-temperature map", "temperature-to-humidity map", "humidity-to-location map"];

    for o in order {
        let mut new_seeds = vec!();
        // println!("Iter");
        for val in seeds.iter_mut() {
            let mut remapped = remap_part2(*val, &mappings[o]);
            new_seeds.append(&mut remapped);
        }
        seeds = new_seeds;
    }

    let mut min = i64::MAX;
    for v in seeds {
        min = min.min(v.0);
    }
    min
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