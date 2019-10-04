fn main() { /* ... */ }

/// Return a bool of if the string follows the 'I before E except after C'
///
/// # Arguments
/// * `word` - The string you want tested
///
/// # An example
/// iEECChecker("zombie") will return true
#[allow(non_snake_case)]
fn iECChecker(word: &str) -> bool {

    if word.contains("ei") && word.find("cei") == None {
        return false;
    } else if word.contains("ie") && word.find("cie") != None {
        return false;
    }

    return true;
}

// ↓ tests all below ↓
#[test]
fn test1() {
    assert_eq!(iECChecker("a"), true);
}

#[test]
fn test2() {
    assert_eq!(iECChecker("zombie"), true);
}

#[test]
fn test3() {
    assert_eq!(iECChecker("transceiver"), true);
}

#[test]
fn test4() {
    assert_eq!(iECChecker("veil"), false);
}

#[test]
fn test6() {
    assert_eq!(iECChecker("icier"), false);
}