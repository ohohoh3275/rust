use std::rc::Rc;

fn take_string(s:String) {

}


fn main() {
    println!("Hello, world!");

    let mut st = String::from("");

    // s == owner
    // one-owner-at-a-time
    let s: String = String::from("Oh");

    //let s2 = s;
    //println!("{}", s);

    let n1 = Rc::new(String::from("i am rc string which is different from just String"));
    let n2= Rc::clone(&n1);
    println!("{}", n1);
    println!("{}", n2);

    let x="kkj";

}
