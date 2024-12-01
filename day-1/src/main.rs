use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut list_a: Vec<u32> = Vec::new();
    let mut list_b: Vec<u32> = Vec::new();
    for l in lines {
        let line = l?;
        let mut values = line.split_whitespace();
        let a = values.next().unwrap().parse::<u32>().unwrap();
        let b = values.next().unwrap().parse::<u32>().unwrap();
        list_a.push(a);
        list_b.push(b);
    }
    list_a.sort();
    list_b.sort();
    let final_iter = list_a.iter().zip(list_b.iter());
    let sum: u32 = final_iter.map(|(a, b)| (*a).abs_diff(*b)).sum();
    println!("{sum}");
    Ok(())
}
