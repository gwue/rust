pub fn square_of_sum(n: u32) -> u32 {
    let square_of_sum: u32 = (1..n + 1).sum::<u32>().pow(2);
    square_of_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    //(1..n + 1).map(|x| x.pow(2)).sum()
    (1..n + 1).fold(0, |acc, x| acc + x.pow(2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
