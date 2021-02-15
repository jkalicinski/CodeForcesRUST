use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let stops: i32 = n.trim().parse().expect("invalid input");
   
    println!("{:?}", stops);
}
