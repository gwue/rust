fn main() {
  let r = Rectangle::square(50);
  let r2 = RStruct(30,50);
  let a = r.area();
  
  let c = area2(&r2);
  println!("Area is {}",a);
  
  println!("Other area is {}",c);

  println!("Rect: {:?}", r);
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

struct RStruct(u32,u32);

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }


  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn area2 (rect: &RStruct) -> u32 {
  return rect.0 * rect.1;
}