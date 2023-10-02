use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    io::stdin().read_line(&mut input3).unwrap();
    let num: i32 = input1.trim().parse().unwrap();
    let vec: Vec<i32> = input2
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{} {}", num + vec[0] + vec[1], input3);
}
