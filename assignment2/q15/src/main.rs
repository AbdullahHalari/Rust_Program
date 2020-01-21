fn main() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum = 1;
    println!("the first 10 natural number is:");
    println!("1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
    for n in 1..10 {
        sum += array[n];        
    }
    println!("the sum of all array element is : {}",sum)
}
