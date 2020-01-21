fn main(){
let numbers = [20, 30, 25, 35, 16, 60, 100];
//calculate sum of all array elements
let mut sum = 0;
for a in 0..numbers.len() {
sum = sum + numbers[a];
}
//calculate average value
let _average = sum / numbers.len();
println!("average value of the array elements is {}",_average);
}