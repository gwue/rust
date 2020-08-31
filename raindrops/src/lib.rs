pub fn raindrops(n: u32) -> String {
    let s3 = "Pling";
    let s5 = "Plang";
    let s7 = "Plong";
    let div3 = n %3 == 0;
    let div5 = n % 5==0;
    let div7 = n % 7==0;

    let mut result = String::from("");

    if div3 {
        result.push_str(s3);
    }
    if div5 {
        result.push_str(s5);
    }
    if div7 {
        result.push_str(s7);
    }
    if ! (div3 || div5 || div7) {
        result = n.to_string();
    }

    result

}
