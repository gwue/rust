fn main() {

    let mut wurst = 1;

   let test = {
wurst = wurst +1;
wurst
   };

   add(5, 9);

   println!("Wurst: {}",wurst);
   println!("Test: {}",test);
    
}

fn add(x: i32, y : i32) -> i32 {
    let r = x + y;
    println!("Val {} + {} = {}", x,y ,r);
    r
}
