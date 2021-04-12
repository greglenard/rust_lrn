use std::io;

fn main() {
    //println!("Hello, world!");

    // let x = 5;
    // let y = 6;
    // let z = x + y;
    // println!("z is {}", z);

    // println!("effin:  {}", effin(2));
    // effin_no_ret(3);
    // next_birthday("jake", 33);
    // next_birthday("Vivian", 0);
    // let b = square(4,4);
    // println!("b: {}", b);

    // if true {
    //     discount(10);
    // } else if false {
    //     discount(0)
    // } else {
    //     discount(11)
    // }

    // loop {
    //     println!("What's the secret word?");
    //     let mut word = String::new();
    //     io::stdin().read_line(&mut word).expect("Failed to read line");

    //     if word.trim() == "rust" {
    //         break;
    //     }

    // }

    // same as above but while

    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What's the secret word");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }

    println!("You know the secret word!");

    // for item in collection
    for i in 1..11 {
        // .. is range shorthand
        // left inclusive, right exclusive
        println!("now serving number {}", i);
    }

    /*
    Match expressions. Similar to if/else if/else, switch, case.  but better
    because of pattern matching and exhaustiveness checking.

    pattern matching
        specify list of patterns to test a value againt.  match express tests
        against list and runs code if matches
    */

    let x = 3;

    match x {
        1 => println!("one lone num"),
        2 => println!("two company"),
        3 => println!("three crowd"),
        _ => println!("something else"),
    }

    let die1 = 1;
    let die2 = 5;

    match (die1, die2){
        (1,1) => println!("Snake eyes!"),
        (5, _) | (_, 5) => {
            println!("hey a five!");
            println!("move and roll again");
        },
        _ => println!("Move your piece"),
    }
    
    /*
    exhaustiveness checking
        must cover every case. helps bugs not covering every scenario
    */

    let is_confirmed = true;
    let is_active = false;

    match (is_confirmed, is_active) {
        (true, true) => println!("good standard"),
        (true, false) => println!("need confirm acct"),
        (false, true) => println!("acct to be deactivated"),
        // false false not covered
        _ => println!("either have a catch all or cover false, false"),
    }

}

// fn discount(day_of_month: u8) {
//     let amount = if day_of_month % 2 == 0 {
//         50
//     }
//     else {
//         10
//     };

//     println!("yur discount is {}", amount);
// }

// fn effin(param1: i32) -> i32 {
//     param1
// }

// fn effin_no_ret(param1: i32) {
//     println!("{}",param1);
// }

// //name (arg1)

// fn next_birthday(name: &str, age: u8) {
//     println!("{} will be {} next birthday", name, age + 1);
// }

// fn square(p1: i32, p2: i32) -> i32{
//     // no semi colon implies returning result
//     p1 * p2
// }


