use std::{collections::HashMap, fs::File, io::{self, BufRead}, iter::{self, Map}};


fn main() {
    let mut history = History {
        left: Vec::new(),
        right: Vec::new(),
    };

    println!("loading files");
    if let Ok(lines) = read_in("realInput.txt") {
        for line in lines.flatten() {
            if let Ok((left,right)) = parse_line(&line) {
                history.left.push(left);
                history.right.push(right);
            } else{
                panic!("Failed to parse line")
            }
        }
    } else {
        panic!("failed to read file")
    }

    println!("sorting");
    // sort them
    history.left.sort_unstable();
    history.right.sort_unstable();  

    
    // part 1
    part_one(history.clone());

    part_two(history.clone());

}

fn part_one(history: History)-> u32 {
// sum and collect them

    let pairs = iter::zip(history.left, history.right);
    // println!("pairs {:?}", pairs.clone().collect::<Vec<(i32,i32)>>());

    let diffs = pairs.map(|p| p.0.abs_diff(p.1));
    // println!("diffs {:?}", diffs.clone().collect::<Vec<u32>>());

    let total = diffs.reduce(|acc, el| acc+el).unwrap(); 
    println!("total: {:?}",total);
    return total
}

fn part_two(history: History) -> i32 {
    // find the similarity
    // we need to count the occurences in the right hand side
    let mut counts: HashMap<i32, i32> = HashMap::new();
    history.right.into_iter().for_each(|el|  match counts.get(&el) {
        Some(v) => {counts.insert(el, v+1);},
        None => {counts.insert(el, 1);},
    });

    let similarity = history.left.into_iter().fold(0, |acc,el| {
        let appears:i32 = match counts.get(&el) {
            Some(v) => *v,
            None => 0,
        };
        let to_add = el * appears;
        return acc + to_add;
    });
    println!("similarity {:?}", similarity);

    return similarity
}

fn parse_line(line: &str) -> Result<(i32,i32), ()> {
    let parts = line.split("   ");
    let left_right: Vec<&str> = parts.collect();
    if left_right.len() != 2 {
        panic!("a")
    }
    let left:i32 = match left_right[0].parse(){
        Ok(num) => num,
        Err(_) => panic!("b")
    };
    let right:i32 = match left_right[1].parse(){
        Ok(num) => num,
        Err(_) => panic!("c") 
    };
    return Ok((left,right))
}

#[derive(Clone)]
struct History {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_in(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file_res = File::open(filename);
    match file_res {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) =>{println!("fail to open file error: {:?}", e); return Err(e)}
    }
}