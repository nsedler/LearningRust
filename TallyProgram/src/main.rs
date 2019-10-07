use std::collections::HashMap;
use std::borrow::Borrow;

#[allow(non_snake_case)]
fn main() {
    tallyScore("EbAAdbBEaBaaBBdAccbeebaec");
}

///
/// Gets the 'score' from a string of lower and uppercase letters and sorts the scores
/// uppercase = -1 lowercase = +1
///
/// # Arguments
/// * `score` the string of lower and uppercase letters
///
/// # An Example
/// tallyScore("abcdeEB") would print a sorted vec of [('c', 1), ('a', 1), ('d', 1), ('b', 0), ('e', 0)]
///

#[allow(non_snake_case)]
fn tallyScore(score: &str) {

    let mut mapScores: HashMap<char, i8> = HashMap::new();
    mapScores.insert('a', 0);
    mapScores.insert('b', 0);
    mapScores.insert('c', 0);
    mapScores.insert('d', 0);
    mapScores.insert('e', 0);

    for c in score.chars() {
        if mapScores.contains_key(c.to_ascii_lowercase().borrow()) {
            if c.is_ascii_lowercase() {
                let count = mapScores.entry(c).or_insert(0);
                *count += 1;
            } else {
                let count = mapScores.entry(c.to_ascii_lowercase()).or_insert(0);
                *count -= 1;
            }
        }
    }

    let mut tallyVec: Vec<(&char, &i8)> = mapScores.iter().collect();
    tallyVec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", tallyVec)

}