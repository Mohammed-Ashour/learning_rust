fn main() {
    println!("Hello, world!");
	print_hello();
    let z: i32 = add(4, 5);
    println!("{z}");

}
fn print_hello() {
    println!("Hello")
}
//
fn add(x:i32, y:i32) -> i32 {
    let result = x+ y;
    return result
}
