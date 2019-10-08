use std::borrow::Borrow;

fn main() {
    let arr: [i32; 2] = [1, 2];
    println!("{}", arraySum(arr.borrow()));
}

#[allow(non_snake_case)]
fn arraySum(arr: &[i32]) -> i32 {
    let mut num = 0;
    for x in arr {
        num += *x;
    }

    num
}
