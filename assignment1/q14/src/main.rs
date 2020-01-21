fn main() {

    let a = 125;
    let b = 12345;
    let ax: u64 = 1234567890;
    let s: u16  = 4043;
    let _x: f32  = 2.13459;
    let dx: f64  = 1.1415927;
    let c: char = 'w';
    let c = c as i32;
    let x: u64 = 2541567890;

println!("a + c is{}", a + c as i32);
println!("x + c is{}", x + c as u64);
println!("dx + x is{}", dx + x as f64);
println!("a + x is{}", a + x as i32);
println!("s + b is{}", s + b);
println!("ax + b is{}", ax + b as u64);
println!("s + c is {}", s + c as u16);
println!("ax + c is{}", ax + c as u64);
println!("ax + dx is{}", dx + ax as f64);
}