# SB
A stack-based language, now you know what the name means, as I have said im uninspired

## syntax :: check design.md
```rust
// 0 is a list that's index-able
// 1 is a stack for general stuff like logic
// 2 is for function arguments

push(1, 1); // push 1 to stack 1
push(1, 1); // push 1 to stack 1

fun add2(int){
    push(2, 2);
    add(2);
    print(pop(2));
}

if pop(1) < pop(1) {  // compare the top 2 of stack 1 doesnt consume
    pop(1); // pops stack 1
    pops(1, 2); 
    // pop and store to index 2 of collection 0
    
    print(pop(1));
}

store(2, 1); // store 2 to collection 0 (the list)
push(get(1), 1); // gets item on list with index of 1 and pushes to stack 1
```
