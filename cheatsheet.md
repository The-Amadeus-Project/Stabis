| Instructions                  | it takes from the main stack | it gives to the main stack | description                                                                      |
|-------------------------------|------------------------------|----------------------------|----------------------------------------------------------------------------------|
| print                         | 1                            | 0                          | pops the top of the main stack then prints it without a new line                 |
| println                       | 1                            | 0                          | pops the top of the main stack then prints it with a new line                    |
| pop                           | 1                            | 1                          | ex. 31 pop -> pop the top element of the stack 31  then pushes to the main stack |
| push                          | 2                            | 0                          | ex. 12 31 push -> pushes 12 to stack 31                                          |
| create_stack                  | 1                            | 0                          |                                                                                  |
| dup                           | 1                            | 2                          |                                                                                  |
| dup2                          | 2                            | 4                          |                                                                                  |
| swap                          | 2                            | 2                          |                                                                                  |
| rotate                        | 3                            | 3                          |                                                                                  |
| drop                          | 1                            | 0                          |                                                                                  |
| main_stack_length             | 0                            | 1                          |                                                                                  |
| stack_length                  | 1                            | 1                          |                                                                                  |
| print_stack                   | 1                            | 0                          |                                                                                  |
| print_main_stack              | 0                            | 0                          |                                                                                  |
| input                         | 1                            | 1                          |                                                                                  |
| --not necessarily functions-- |                              |                            |                                                                                  |
| 1 2 3                         | N/A                          | N/A                        | this is how you push to main stack  this pushes 1 2 3 to the main stack          |
| loop                          | 1                            | 0                          |                                                                                  |
| if                            | 1                            | 0                          |                                                                                  |
| else                          | 0                            | 0                          |                                                                                  |
| lastly                        | 0                            | 0                          |                                                                                  |
| end                           | 0                            | 0                          |                                                                                  |
| inst                          | 0                            | 0                          |                                                                                  |
| +                             | 2                            | 1                          |                                                                                  |
| -                             | 2                            | 1                          |                                                                                  |
| /                             | 2                            | 1                          |                                                                                  |
| *                             | 2                            | 1                          |                                                                                  |
| %                             | 2                            | 1                          |                                                                                  |
| ==                            | 2                            | 1                          |                                                                                  |
| !=                            | 2                            | 1                          |                                                                                  |
| '>'                           | 2                            | 1                          |                                                                                  |
| '>='                          | 2                            | 1                          |                                                                                  |
| <                             | 2                            | 1                          |                                                                                  |
| <=                            | 2                            | 1                          |                                                                                  |