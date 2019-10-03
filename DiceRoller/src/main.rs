use rand::Rng;
use std::io;
use std::string::String;

#[allow(non_snake_case)]
fn main() {

    let mut diceNotation = String::new();

    println!("Please enter correct dice notation. (Ex: 4d12)");
    io::stdin().read_line(&mut diceNotation).expect("Couldn't read line");

    if diceNotation.contains("d") {
        let diceNotation = diceNotation.split("d").collect::<Vec<&str>>();

        let dieAmount: i32 = diceNotation[0].trim().parse().expect("couldnt trim");
        let dieSides: i32 = diceNotation[1].trim().parse().expect("couldnt trim");
        let mut total: i32 = 0;
        let mut result = String::new();

        for _x in 0..dieAmount {
            let randomNum = rand::thread_rng().gen_range(1, dieSides + 1);
            result = result + randomNum.to_string().as_ref() + " ";
            total += randomNum;
        }

        println!("{}: {}", total, result);

    } else {
        panic!("Didn't get correct format; 4d12, 5d6 etc");
    }
}
