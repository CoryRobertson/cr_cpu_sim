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

### Labels:
Labels represent a compiletime marker of an instruction number. 
When a jump command uses a label, the label text is replaced with the 
line number equivalent at compiletime, which means that it is not
required to change every jump instruction when a previous line is changed.

TLDR: Labels are a dynamic line number reference

### Add:
Immediate mode:
```
add 17
```
Adds 17 dec to the **acc** register, the input number is 8 bits

Register mode:
```
add acc or
```
Adds the output register to the **acc** register, storing the result in the **acc** register

### Subtract:
Immediate mode:
```
sub 5
```
Subtracts 5 from the **acc** register

Register mode:
```
sub acc or
```
Subtracts the output register from the **acc** register, storing the outcome in **acc**

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
Moves 400 dec into the **acc** register

Register mode:
```
move acc or
```
Copies the output register into the **acc** register

### Compare:
All compare instructions store outputs in the form of flags (see flags section)

Immediate mode (compare with u16):
```
icmp acc 5
```
Compares register **acc** with the literal number 5, the literal is a 16 bit number

Immediate mode long (compare with u32):
```
icmpl acc 1000
```
Compares register **acc** with literal number 1000. The literal is a 32-bit number

Register mode: 
```
cmp acc cr
```
Compares register **acc** with register **cr**
 
### Jump instructions
Jump always
```
:supercoollabel:
; -- snip --
jmp supercoollabel
```
Jump if overflow
```
:supercoollabel:
; -- snip --
jov supercoollabel
```
Jump if zero
```
:supercoollabel:
; -- snip --
jz supercoollabel
```
Jump if less than
```
:supercoollabel:
; -- snip --
jlt supercoollabel
```
Jump if greater than
```
:supercoollabel:
; -- snip --
jgt supercoollabel
```
Jump if equal
```
:supercoollabel:
; -- snip --
je supercoollabel
```
