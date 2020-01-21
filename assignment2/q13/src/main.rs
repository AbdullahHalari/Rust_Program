fn main() {
    // loop
    let mut num = 0;
    loop {
        if num < 3{
        println!("I LOVE MY MOTHER");
        }
        else {
            println!("i love my father");
        }
        if num == 3{
            break;
        }
        num = num + 1 
    }
    // while loop
    let mut num = 0;
    while num < 4 {
        if num < 3{
            println!("I LOVE MY MOTHER");
        }
        else{
            println!("i LOVE MY FATHER");
        }
        num = num + 1;
    }
    // for loop
    for num in (0..4){
        if num < 3{
            println!("I LOVE MY MOTHER");
        }
        else{
            println!("i LOVE MY FATHER");
        }
    }

}

        
    


