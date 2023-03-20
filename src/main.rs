fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    println!("The key is {}", key);
}
