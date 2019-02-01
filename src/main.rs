struct Animal(String);
impl Animal {
    fn howl(&self) {
        println!("Fuck!")
    }
    fn kill(self) {}
}
impl Drop for Animal {
    fn drop(&mut self) {
        println!("{} was killed!", self.0)
    }
}

fn main() {
    let mut s = String::new();
    for i in 0..4 {
        s.push((i + b'0') as char)
    }

    let tiger = Animal(s.to_string());
    tiger.howl();
    tiger.kill();
}
