enum MyEnum {
    A,
    B,
    C
}

enum Status {
    Tired,
    Happy,
    Angry
}

fn returning_option(val:i32) -> Option<bool> {
    if val == 0 {
        Some(true)
    } else {
        None
    }
}

impl Status {
    fn express_your_feeling(&self) {
        match *self {
            Self::Happy => println!("I am so happy!"),
            Self::Tired => println!("I am so Tired"),
            Self::Angry => println!("I am so Angry"),

        }
    }
}

struct Person {
    name: String
}

// enum Happening<'a> {
//     Friend(Person, Person),
//     Teaching(Person, Vec<Person>)
// }


fn main() {

    let my_enum = MyEnum::A;
    match my_enum{
        MyEnum::A => {println!("Hey I am A!")}
        MyEnum::B => {println!("Hey I am B!")}
        MyEnum::C => {println!("Hey I am C!")}
    }

    let status:Status = Status::Happy;
    dbg!(status.express_your_feeling())
        

    // let res = returning_option(0);

    // let person1 = Person(name);

}

