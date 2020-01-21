//  use std::io; 
//  fn main(){
    
//     println!("please enter number to calculate factorial:");
//     let mut int = String::new();
//     io::stdin().read_line(&mut int)
//         .expect("Failed to read your input");

//     let int: i32 = match int.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 0
//     };
//     let fact:i32 = factorial(int);
//     println!("factorial is {}", fact);
// }

// fn factorial(number:i32)->i32{
    // let mut num: i32 = number;
    // let mut fact: i32 = 1;
    // while num > 0{
    //     fact = fact * num;
    //     num = num - 1;
    // }
    
//     fact
// }


// // // fn factorial(number:i32)->i32{
// // //     // Enter your code here.
// // //     let mut num:i32 = number;
// // //     let mut fact:i32 = 1;
// // //     while num > 0{
// // //         fact = fact * number;
// // //         num = num - 1;
// // //     }
// // //     fact
// // // }
// // fn main (){
// //     let a = 2;
// //     let b = 3;
// //     let c = a+b;
// //     let d = a-b;
// //     println!("the sum is{}",c);
// //     println!("the diff is {}",d);
// // }

// fn main (){
//     let mut x = 2;
//     loop{
//             println!("{}",x);
//             if x==10{
//                 break;
//             }
//             x = x+2;
//     }
// }
