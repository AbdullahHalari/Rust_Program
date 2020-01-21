use std::io;

fn main() {   
    let mut data = String::new();
    println!("enter maths marks");
    io::stdin().read_line(&mut data);
    let marks_maths :u32 = data.trim().parse().unwrap();

    data = String::new();   
    println!("enter physics marks");
    io::stdin().read_line(&mut data);
    let marks_physics :u32 = data.trim().parse().unwrap();

    data = String::new();
    println!("enter marks_chemistry");
    io::stdin().read_line(&mut data);
    let marks_chemistry :u32 = data.trim().parse().unwrap();

    let total_threesubject :u32 = marks_maths + marks_physics + marks_chemistry;
    let subjects = marks_physics + marks_chemistry;

    if (marks_maths >= 65) & (marks_chemistry >= 50) & (marks_physics >= 55) & (total_threesubject >= 180){ 
        println!("student is eligible");
    }
    else if (marks_maths >= 65)  & (subjects >= 140){
        println!("student is eligible");
    }
    else {
        println!("student is not eligible");
    }


}
    