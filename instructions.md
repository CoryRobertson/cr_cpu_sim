### Registers:
* ACC Accumulator
* CR Counting register
* PC Program counter
* IR Instruction register
* OR Output register
* SP Stack pointer
* TR Temporary register

### Flags:
* zero flag
* less than flag
* greater than flag
* equal flag
* overflow flag


### Add:
Immediate mode:
```
add 17
```
Adds 17 dec to the acc register, the input number is 8 bits

Register mode:
```
add acc or
```
Adds the output register to the acc register, storing the result in the acc register

### Subtract:
Immediate mode:
```
sub 5
```
Subtracts 5 from the acc register

Register mode:
```
sub acc or
```
Subtracts the output register from the acc register, storing the outcome in acc

### Dump
```
dump
```
Dumps all relevant cpu information into the console

### Move
Immediate mode:
```
imovel acc 400
```
Moves 400 dec into the acc register

Register mode:
```
move acc or
```
Copies the output register into the acc register
 
### Jump instructions
Jump if overflow
Jump if zero
Jump if less than
Jump if greater than
Jump if equal
