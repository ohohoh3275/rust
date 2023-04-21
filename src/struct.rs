#[de]

struct Classical {

}

impl std::fmt::Display for Classical<'_> {

}

struct Struct;
impl Struct {
    fn say_hi{
        println("hi!")
    }
}

impl Struct {
    fn say_bye{
        println("bye!")
    }
}

struct TupleLikeStruct(String, i32, bool)

impl TupleLikeStruct(String, i32, bool) {
    fn new(name:String, age:i32, male:bool) -> Self {
        Self(name, age, male);
    }
}

fn main() {
    dbg!()

    Struct::say_hi(&self);
    Struct::say_bye(&self)

}