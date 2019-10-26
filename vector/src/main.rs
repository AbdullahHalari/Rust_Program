use std::{io};

fn main(){
    println!("enter size of vector then enter integer with space");
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    let vec: Vec<i32> = read_vector_elements();
    let mut num:i32 = int - 1;
    while num >= 0{
        let value: i32 = vec[num as usize];
        print!("{} ", value);
        num = num - 1;
    }
 }


fn read_vector_elements() -> Vec<i32>{
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    let v: Vec<i32> = s.trim()
        .split(' ')
        .map(|i| i.parse().unwrap())
        .collect();
    v
}