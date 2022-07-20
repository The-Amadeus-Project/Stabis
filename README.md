# Stabis
A stack-based language, now you know what the name means, as I have said im uninspired.
SbIs is a stack-based concatenative language

# Purpose
Stabis is a project for me learning language 
development, learning the workflows and maybe a project to flex people on, totally not because I hate naming variables


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

## RoadMap
- [ ] Documentation :: check cheatsheet.md
### Compilation
- [x] pushing data types to main stack 
- [x] calling built-in functions
- [x] numbered loops 
- [x] if statements
- [x] else statements 
- [x] lastly statements
- [x] new function using inst 

### Interpretation
- [x] pushing data types to main stack 
- [x] calling built-in functions
- [x] numbered loops 
- [x] if statements
- [ ] else statements
- [ ] lastly statements
- [ ] new function using inst

## Requirements
- Rust 1.6
- Python 3
- Git (Optional, you can download the repo from github website)

## How To Use
First clone the repo
```commandline
git clone https://github.com/The-Amadeus-Project/SB-instruction-set.git
```

Change directory to the cloned repo
```
cd SB-instruction-set
```

now run Cargo build release
```commandline
cargo build --release
```

Now create a file test.sbis(must be in the same directory) that contains
```rust
"Hello World!" println
```

to compile and run use c_r.py
```commandline
python c_r.py test.sbis
```

You should get
```commandline
Hello World!
```
