use std::io::{self, BufRead};


fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");

    let stops: i32 = n.trim()
                      .parse()
                      .expect("invalid input");

    let mut tram_stop = String::new();
    let max : i32 = 0;
    let mut current : i32 = 0;

    for x in 0..stops {
        io::stdin()
            .read_line(&mut tram_stop)
            .expect("failed to read input.");
            
            let spilted: Vec<&str> = tram_stop.split_whitespace().collect();
            let people_in : i32 = spilted[1].parse().expect("error");
            let people_out : i32 = spilted[0].parse().expect("error");

            current = (current - people_out) + people_in;
    }
    

    println!("{:?}", current);
}
