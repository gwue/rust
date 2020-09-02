use clockk::Clock;

fn main() {
    let min = 40;
    let c1 = Clock::new(0, 50);
    println!("Clock1: {}", c1);
    let c2 = &c1 + min;
    //println!("Clock1: {}",c);
    println!("Clock2: {}", c2);
    println!("min {}", min);


    let c3 = &c1 + &c2;

    println!("Clock1: {}", c1);
    println!("Clock2: {}", c2);
    println!("Clock3: {}", c3);
}
