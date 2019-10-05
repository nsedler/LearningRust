fn main() { }

fn balanced(chars: &str) -> bool {

    let mut x = 0;
    let mut y = 0;

    for c in chars.chars() {
        if c == 'x' {
            x += 1;
        } else {
            y += 1;
        }
    }

    if x == y {
        return true
    }

    return false
}

#[cfg(test)]
mod tests {
    use crate::balanced;

    #[test]
    fn test1() {
        assert_eq!(true, balanced("xxxyyy"))
    }
    #[test]
    fn test2() {
        assert_eq!(true, balanced("yyyxxx"))
    }
    #[test]
    fn test3() {
        assert_eq!(false, balanced("xxxyyyy"))
    }
    #[test]
    fn test4() {
        assert_eq!(true, balanced(""))
    }
    #[test]
    fn test5() {
        assert_eq!(true, balanced("yyxyxxyxxyyyyxxxyxyx"))
    }
    #[test]
    fn test6() {
        assert_eq!(false, balanced("xyxxxxyyyxyxxyxxyy"))
    }
    #[test]
    fn test7() {
        assert_eq!(false, balanced("x"))
    }
}