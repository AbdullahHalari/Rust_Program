fn main() {
let names = ["Ali", "Zain" , "Naufil"];
for name in names.iter() {
match name {
&"Ali" => println!("There is a rockstar among us!"),
_ => println!("Hello {}", name),
}
}
}