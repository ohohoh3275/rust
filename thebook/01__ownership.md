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

Q. is it like Wrapper Class and Primitive type in Java ?

---
