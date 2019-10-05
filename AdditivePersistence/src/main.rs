#[allow(non_snake_case)]
fn main() {
    println!("{}", addPers(199))
}

/// Returns the additive persistence of a number
///
/// # Arguments
/// * `num` - The i32 you want the additive persistence of
///
/// # An Example
/// addPers(13) would return 1
#[allow(non_snake_case)]
fn addPers(mut num: i32) -> i32 {
    let mut count = 0;
    while num > 9 {
        num = addSum(num);
        count += 1;
    }
    count
}

#[allow(non_snake_case)]
fn addSum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

// all of my tests to seem fancy
#[cfg(test)]
mod tests {
    use super::addPers;

    #[test]
    fn test1() {
        assert_eq!(1, addPers(13))
    }
    #[test]
    fn test2() {
        assert_eq!(2, addPers(1234))
    }
    #[test]
    fn test3() {
        assert_eq!(2, addPers(9876))
    }
    #[test]
    fn test4() {
        assert_eq!(3, addPers(199))
    }
}
