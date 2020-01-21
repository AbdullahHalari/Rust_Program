use std::io;
fn main(){
	
  	let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    // Enter your code below this line.
    is_even_odd(int);
}

fn is_even_odd(x:i32){
    
    let mut num:i32 = x % 2;
    
    if num == 0{
        println!("even")
    }
    else
    {
        println!("odd")
    }
}   