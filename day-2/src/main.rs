use std::{fs::File, io::{self, BufRead}};

fn main() {
    println!("Hello, world!");

    println!("PART ONE");
    part_one();

    println!("PART TWO");
    part_two();
}

// 
fn part_one(){
    let lines = read_in("realInput.txt").unwrap();
    let rows: Vec<Vec<i32>> = lines.flatten().map(|s| parse_line(&s)).collect();
    let count:i32 = rows.into_iter().map(|row| { 
        inspect_row(row)
    }).fold(0, |acc, x| if x {acc + 1} else {acc});
    println!("count: {:?}", count);
}

fn inspect_row(row: Vec<i32>) -> bool {
    let row_diffs: Vec<i32> = row.windows(2).map(|xs| {
        xs[0] - xs[1]
    }).collect();

    apply_rules(row_diffs)
}


fn part_two(){
    let lines = read_in("realInput.txt").unwrap();
    let rows: Vec<Vec<i32>> = lines.flatten().map(|s| parse_line(&s)).collect();
    let count:i32 = rows.into_iter().map(|row| { 
        inspect_row_enhanced(row)
    }).fold(0, |acc, x| if x {acc + 1} else {acc});
    println!("count: {:?}", count);
}

fn subsequences_with_one_removed(vec: &[i32]) -> Vec<Vec<i32>> {
    let mut subsequences = Vec::new();

    for i in 0..vec.len() {
        let mut subseq = vec.to_vec();
        subseq.remove(i);
        subsequences.push(subseq);
    }

    subsequences
}

// we just going to brute force it - try each of the substrings
fn inspect_row_enhanced(row: Vec<i32>) -> bool {

    // do the following over all substrings

    let all_subs = subsequences_with_one_removed(&row); 

    let any_happy = all_subs.into_iter().map(|row| {
        let row_diffs: Vec<i32> = row.windows(2).map(|xs| {
            xs[0] - xs[1]
        }).collect();
    
        apply_rules(row_diffs)
    }).any(|b| b);

    any_happy
}

// 2 conditions to return false:
//  1. diff too big
//  2. positive and negative diffs
fn apply_rules(row: Vec<i32>) -> bool {
    let mut neg = false;
    let mut pos = false;
    for x in row {
        if (x == 0) || (x > 3) || (x < -3) {
            return false
        }
        if x < 0{
            neg = true;
        }
        if x > 0 {
            pos = true;
        }
    }
    return neg ^ pos;
}

fn parse_line(line: &str) -> Vec<i32> {
    let parts = line.split(" ");
    let all: Vec<&str> = parts.collect();
    let i32s: Vec<i32> = all.into_iter().map(|s| s.parse().unwrap()).collect();
    return i32s;
}


fn read_in(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file_res = File::open(filename);
    match file_res {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) =>{println!("fail to open file error: {:?}", e); return Err(e)}
    }
}