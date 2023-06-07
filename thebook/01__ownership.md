### Ownership

---

- this is Rust's unique feature
- it enables neither programmer make memory free nor language looks for garbage-collected value.
- so, Ownership can be used for what other languages do when memory dellocation


---

- stack and heap
- stack 
  - stores values in a way of LIFO
  - push & pop

- heap
  - less organized 
  - memory allocator find random place to store `pointer`
    -> allocating
    
- data in the heap is more slower to get 
  - because memory allocator first should find empty space for storing datas
  - so cleaning up unused data on the heap is main purpose of dealing with ownership


---

- ownership rule
  - each value has an owner
  - one owner at a time


---

- Variable Scope
  ```
  {
  
  }
  ```
  
---

- String
  - String type is good for understanding ownership
  - 1. string-literal
  - 2. String::from

Q. is it like Wrapper Class and Primitive type in Java ? <br/>
A. i think it's different ...


  - string literal 
    - is set when app is on compile time.
    - it also means that string literal in the text is determined when app is on final execution.
    - is immutable
   


  - String type
    - is allocated on the heap
    - initialize :  `let s = String::from("hi there");`
      - `from` function is `Method Syntax` (on page 97)
      - also discussed in page 125 "Path for Reffering to an Item in the Module Tree"
    - is mutable

> The thing that string litaral is mutable and String type is not is about Memory Allocation


- string litaral is faster because it has immutablilty.
  - thing is we cannot change size of the value
- on the other hand, String type is mutable
  - which means we need to allocate memory on the heap. by coding, one by one, directly
  - > it must have memory allocator at runtime
    - 1. call `String::from`
    - 2. using (1) in only one scope, Rust language drops automatically at the closing curly bracket.

- this can work our `String` needs the allocator

---

- Variable and Data

```
let x = 5;
let y = x;
```

- integer are simple value with fixed size that we know
- 5, 5 value are pushed into the stack

but String

```
let s1 = String::from("hello");
let s2 = s1;
```

- s1 has
  1. pointer : points 0 index of the value
  2. length : how much bytes are String current using
  3. capacity : total amount of memory that String has received from the allocator.


- when we assign s1 to s2
  1. s1 pointer points where exactly s1 pointer points (0 index of the value)

  2. not coping the heap data but points same value

  3. when s2 and s1 go out of the scope, this is known as a double free error.

  - Rust's move : without copying the data(known as <u>swallow copy</u> with other language), Rust invalidates the first variable, s1 was moved into s2.


---


- Rust's deep copy : use `clone`

```
let s1 = String::from("i will be deep copied");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

-> heap data DOES get copied

---

❓ in a way of not String, but integer

 ```
 let x = 5;
 let y = x;
 
 println!("x = {x}, y = {y}");
 ```
- this works.
  - not with `clone`
  - x did not `moved` to y

because it's Stack-Only Data's copy
  - no difference between deep and swallow copying
 
+) there are `traits` in Rust language : Copy, Drop
  - these can implement Copy
    - integer (u32)
    - Boolean (bool, true, false)
    - floating-point type (f64)
    - characer (char)
    - Tuples (i32, i32 o) / (i32, String x)

---
with Function

- function that takes ownership

```
fn this_will_take_ownership(str: String) {
  println("{str}");
}

fn main() {
  let str = String::from("spoiler : this will be taken by function");
  
  this_will_take_ownership(str);
 
}
```

- with integer

```
fn what_about_integer(int: i32) {
  println("{int}");
}

fn main() {
  let i = 5;
  what_about_integer(i);
  
  println("{i} is Copy. so it is still valid");
}

```

- return value
```
fn just_return_any_string() -> String {
  
  let str = String::from("any");
  
  str // this returns str
}

fn returns_argument_string_itself(str: String) -> String {
  str
}

fn main() {

  let s1 = just_return_any_string();
  let s2 = String::from("i am newly declared");
  let s3 = returns_argument_string_itself(s2);
  
  // valid when printing out : s1, s3
}
```

- return multiple value using tuple

```
fn return_length_of_string -> (String, usize) {

  let length = s.len();
  
  (s, length)
}

fn main() {
  let s1 = String::from("hi there");
  
  let (s2, length) = return_length_of_string(s1);
  
  println!("the length '{s2}' is {len}");
}
```

