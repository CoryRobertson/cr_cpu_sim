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

### Shift Left & Right:
If a shift caused a bit overflow of any kind, the register is instead set to 0
Not to be confused with an integer overflow, which would act normally.

Register mode:
```
shl acc 2
```
Shift the acc register left by two bits

```
shr acc 2
```
Shift the acc register right by two bits

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
The number moved can not be 0

Register mode:
```
move acc or
```
Copies the output register into the **acc** register

Address mode:
```
movea 96 acc
```

Variable mode:
```
myvariable = 15
movea myvariable acc
```
Copies the value of acc into myvariable

Copies the value of acc into address 96

### Load Effective Address:
Immediate mode:
```
lea 96
```
Loads the dram value stored at address 96 and stores it in the output register

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
