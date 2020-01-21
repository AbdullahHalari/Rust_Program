use std::io;

fn main() {
    
    let mut data = String::new();
    println!("enter integer");
    io::stdin().read_line(&mut data);
    let data :u32 = data.trim().parse().unwrap();
    let mut modeCount : u32 = 0;
    for num in 1..data{
       // println!("num % data ");
       // println!("the num is: {}",num);
       if data % num == 0{
           modeCount = modeCount + 1
       }
    }

    //println!("modeCount {}",modeCount);
    if modeCount>1{
        println!("the num is not prime");
    }
        else {
            println!("the num is prime");
        }
    }



