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


---


---
