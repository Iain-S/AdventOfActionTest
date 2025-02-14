use std::fs::read_to_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_to_string("input.txt").unwrap();
    if args[args.len()-1] == "one" {
        println!("{}",part_one(input));
        return
    }
    println!("{}",part_two(input));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn split_line(s: String) -> (u32, u32) {
    let vector: Vec<&str> = s.split("   ").collect();
    if vector.len() != 2 {
        panic!("Expected two numbers, got {}", vector.len());
    }
    (
        vector[0].parse::<u32>().unwrap(),
        vector[1].parse::<u32>().unwrap(),
    )
}

fn part_one(s: String) -> u32 {
    // Pair up the smallest number in the left list with the smallest number in the
    // right list, then the second-smallest left number with the second-smallest
    // right number, and so on.

    // Within each pair, figure out how far apart the two numbers are; you'll need
    // to add up all of those distances.
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Split the input into two vectors
    for line in s.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (l, r) = split_line(line.to_string());
        left.push(l);
        right.push(r);
    }

    // Sort the vectors
    left.sort();
    right.sort();

    // Calculate the distance between each pair of numbers
    let zip = left.iter().zip(right.iter());
    let result = zip.map(|(l, r)| if r > l { r - l } else { l - r });

    // Sum the result
    result.sum()
}

fn part_two(s: String) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Split the input into two vectors
    for line in s.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (l, r) = split_line(line.to_string());
        left.push(l);
        right.push(r);
    }

    let mut result = 0;
    for l in left {
        let mut x = 0;
        for r in right.clone() {
            if r == l {
                x += 1;
            }
        }
        result += l * x;
    }
    result
}
