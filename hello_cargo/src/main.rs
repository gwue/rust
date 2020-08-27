fn main() {

    let mut wurst = 1;

   let test = {
    wurst = wurst +1;
    wurst
   };

   add(5, 9);

   println!("Wurst: {}",wurst);
   println!("Test: {}",test);

   let number = 2;
   let test_even = if number % 2 == 0 {"even"} else {"odd"};

   println!("{}", test_even);

   let mut counter = 0;
   let result = loop {
       counter +=1;
       if counter == 10 {
           break counter*2;
       }
   };

   let a = [1,2,3,4,1];
   println!("A.len: {}",a.len());

   println!("Loop result {}", result);
   let mut sum = 0;
   for element in a.iter() {
        println!("For eleme {}",element);    
    sum += element;

   }
   println!("For sum: {}",sum);

for number in 1..4 {
    println!("{}!",number);
}
    
}

fn add(x: i32, y : i32) -> i32 {
    let r = x + y;
    println!("Val {} + {} = {}", x,y ,r);
    r
}