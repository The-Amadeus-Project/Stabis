
# Syntax
```rust
// 0 is the main stack

1 // 1 -> stack 0, 1 is pushed to the main stack
1 // 1 -> stack 0, 1 is pushed to the main stack
+ // pops two element from the main stack adds them then pushes them back to the main stack
print // pops the top element then prints it

```

```rust
1 create_stack  // creates stack 1
12 1 push       // pushes 12 to stack 1
// 12   -> push 12 to main stack
// 1    -> push 1  to main stack
// push -> pop the top to element of main stack,
//         the top element is the stack to push to
//         the second top is the value to push
1 pop           // pop the top element of stack 1 then push to main stack
print           // pop the top element of main stack then print
```