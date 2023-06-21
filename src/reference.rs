// the rust book excutable reference example
// run with `cargo run --bin reference`

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The value of s2 is: {}", s2);

    let mut s3 = String::from("hello");
    let r1 = &mut s3;
    // let r2 = &mut s3; // error[E0499]: cannot borrow `s3` as mutable more than once at a time
    println!("{}", r1);
    // println!("{}", r2);

    let mut s4 = String::from("hello");
    {
        let r1 = &mut s4;
        println!("{}", r1);
    }
    let r2 = &mut s4;
    println!("{}", r2);

    let mut s5 = String::from("hello");
    let r1 = &s5;
    let r2 = &s5;
    println!("{} and {}", r1, r2);
    let r3 = &mut s5;
    println!("{}", r3);

    let mut s6 = String::from("hello");
    let r1 = &s6;
    let r2 = &s6;
    println!("{} and {}", r1, r2);
    // let r3 = &mut s6; // error[E0502]: cannot borrow `s6` as mutable because it is also borrowed as immutable
    // println!("{}", r3);

    let mut s7 = String::from("hello");
    let r1 = &s7;
    let r2 = &s7;
    println!("{} and {}", r1, r2);
    println!("{}", r1);
    println!("{}", r2);
    let r3 = &mut s7;
    println!("{}", r3);

    let mut s8 = String::from("hello");
    let r1 = &s8;
    let r2 = &s8;
    println!("{} and {}", r1, r2);
    println!("{}", r1);
    println!("{}", r2);
    // let r3 = &mut s8; // error[E0502]: cannot borrow `s8` as mutable because it is also borrowed as immutable
    // println!("{}", r1);
    // println!("{}", r2);
    // println!("{}", r3);

    let mut s9 = String::from("hello");
    let r1 = &mut s9;
    println!("{}", r1);
    // let r2 = &mut s9; // error[E0499]: cannot borrow `s9` as mutable more than once at a time
    // println!("{}", r2);

    let mut s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} and {}", r1, r2);
    let r3 = &mut s10;
    println!("{}", r3);
    // println!("{}", r1); // error[E0502]: cannot borrow `s10` as mutable because it is also borrowed as immutable
    // println!("{}", r2); // error[E0502]: cannot borrow `s10` as mutable because it is also borrowed as immutable
    // println!("{}", r3);
    
}

// calculate_length function takes a reference to a String
// and returns the length of the String
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but because it does not have ownership of what
  // it refers to, nothing happens

// change function takes a mutable reference to a String
// and changes the String
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope and is dropped, its memory goes away
  // danger!

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// } // s goes out of scope and is dropped, its memory goes away
  // no danger!
