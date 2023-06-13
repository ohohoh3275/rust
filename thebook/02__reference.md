## reference

- AS-IS
  - function `calculate_length` made s2 take s1's ownership.
  - so it is not printing when s1 called after the function(calculate_length) executed
```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
    
    println!(s1); // !not working!
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

- TO-BE
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```


  - instead of using `String`, we use `&String`
  - `&` represent `references`
    - this help us to refer to some value without taking ownership.
  - &String value points String == s: &String -> s1: String
  - reference points original variable, not value itself.
  - we call <u>reference `borrows` not `owns`</u>
 

> borrowed, reference value is <b>immutable</b> by default.

---

- mutable reference


```
let mut s = String::from("hello"); // change declaring String firstly

let r1 = &mut s; // it just add & on original shape
let r2 = &mut s;

println!("{r1}, {r2}");”

```
- but it is not working codes above.
  - errors out `cannot borrow s as mutable more than once at a time`

... think about this!
---
