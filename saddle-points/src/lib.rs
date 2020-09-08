use std::cmp::min;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output = Vec::new();

    for row in 0..input.len() {
        let row_max = row_max(input, row);
        println!("Max in row {} = {}", row, row_max);

        for col in 0..input[row].len() {
            let col_min = col_min(input, col);
            println!("Min in col {} = {}", col, col_min);
            let val = input[row][col];
            if val >= row_max && val <= col_min {
                output.push((row, col));
            }
        }
    }

    output
}

fn row_max(input: &[Vec<u64>], row: usize) -> u64 {
    *input[row].iter().max().unwrap_or(&u64::MIN)
}

fn col_min(input: &[Vec<u64>], col: usize) -> u64 {
    let mut c_min = u64::MAX;
    for row in input {
        c_min = min(row[col], c_min);
    }
    c_min
}
