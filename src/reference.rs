fn main() {
    let mut mutable = "mutable";

    let mutable_reference = &mut mutable;

    mutable_reference = "mutable_reference"; // not working

    *mutable_reference = "mutable_reference"; // working
    
    
}