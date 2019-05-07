fn main() {
    let mut s = String::new();
    for i in 0..4 {
        s.push((i + b'0') as char)
    }
    println!("{}", s);
}
