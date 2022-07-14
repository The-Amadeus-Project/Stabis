# SB
A stack-based language, now you know what the name means, as I have said im uninspired.
SbIs is a stack-based concatenative language

## syntax :: check design.md
```rust
//

1   // push 1 to main stack
create_stack // creates stack 1, pops the top element of the main stack

12  // push 1 to main stack
3   // push 1 to main stack
+   // pops the top two elements of the stack, adding, then pushes the result back
print // pops the top element then prints it


12 1 +  // pushes 12 and 1 then adds them -> 12 + 1
3 *     // pushes 3 to the main stack the multiplies the last result and three -> res * 3
3 %     // pushes 3 the mods the last result and three -> res % 3
// (12 + 1) * 3 % 3
print   // pops the top element of the main stack then prints it
```
