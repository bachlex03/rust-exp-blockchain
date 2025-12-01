# Why?

## 1. The critical problem: Trade off of Speed and Safety

- Rust is designed to archive Seed and Safety
- This lead to number **2. Memory Safety Without Garbage Collection**

## 2. Memory Safety Without Garbage Collection

**70% problems of software is from unsafe memory**:

### Errors regarding to unsafe memory

- Buffer Overflows (stack & heap)
- Use-after-free
- Dangling pointers
- Type confusion
- Data races

* link: `https://www.youtube.com/watch?v=Z7qIYrG24I0&list=LL`

## Ownership

## 2 Accessibility

- Allow to any read (multiple read)
- Just 1 write

=> resolve Data races
