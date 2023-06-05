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
