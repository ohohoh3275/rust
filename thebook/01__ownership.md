### Ownership

---

- this is Rust's unique feature
- it enables neither programmer make memory free nor languages looks for garbage-collected
- so, Ownership does what other languages do for memory dellocation


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
  - so cleaning up unused data on the heap is main purpose of dealing ownership


---

- ownership rule
  - each value has an owner
 
