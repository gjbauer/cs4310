# Design Recipe for an AMD64 ASM Program

### A Program is a Bunch of Functions
 * Execution starts at “main”.
 * We build functions to be independent.
 * To design a program, design its functions.

### Design Recipe for a Function
 * Signature
 * Pseudocode
 * Variable mappings
 * Skeleton
 * Write the body

### Signature and Pseudocode
 * Thinking in assembly is a bit fiddly. Better to think at a higher level of abstraction.
 * Translating pseudocode to assembly is resonably straightforward.
 * The design recipie, in any language, suggests signature first.
   - What are our arguments?
   - What will we return?
 * Pseudocode is useful to determine how we will compute our function.
   - Something like C makes good pseudocode here.

### Variable Mappings
 * In assembly we don’t get to use named variables.
 * Instead, we need to figure out where to put each value used in our function.
 * Values either go in registers or on the stack.
 * It’s worth actually figuring this out and writing it down.

The normal strategy:

 * Try to allocate values to the registers they already need to be in (arguments in argument registers, result in result register).
 * If that won’t work, move values to safe registers.
 * For variables where you need to take their address, put them on the stack in space allocated by the enter instruction.

Another way to write functions is the way C pretends to work: all local variables live on the stack. In that plan:

 * Using the ENTER instruction, we allocate a slot for each local variable and function argument.
 * The first stack slot is -8(%rbp), the next -16(%rbp), etc.
 * Function arguments come in in registers - note which ones.

We also should decide which temporary values we produce and where those will be stored. These can be allocated to temporary registers: %r11, %r10, %r9, %r8.

#### Thinking
Strategies:

 * Everything in registers, save to registers, pre-save to stack.
 * Everything on the stack.
 
### Function Skeleton

```
label:
    # Prologue:
    #   Set up stack frame.
    # Body:
    #   Just say "TODO"
    # Epilogue:
    #   Clean up stack frame.
```

### Function Prologue

 * Save (push) any callee-save registers we use.
 * ENTER: Allocate stack space for local variables.
 * Save any arguments that we want to use after making function calls.
   - Plan A: Push to stack
   - Plan B: Copy to callee-save registers
 * Make sure stack pointer aligned to 16-byte boundary before doing any further function calls.

### Function Epilogue

 * Make sure the return value is in %rax
 * LEAVE: Deallocate the entire stack frame.
 * Restore (pop) any callee-save registers we used.
 * RET: Return to caller.

### Writing the Function Body

 * Translate the pseudocode to assembly, line by line.
 * Use the variable mapping you already figured out.

# Translating Pseudocode to ASM#

 * C can translate to ASM nearly 1:1.
 * Every C statement can be used to fill in a corresponding ASM “template”.
 * The resulting ASM will perform the same computation.

### Variables, Temporaries, and Assignment

 * Each C (int, pointer) variable maps to either a register or a stack location.
 * Temporary values map to a temporary register.
 * Registers can be shared / reused if you run out.

Example:

Pseudocode:

```
  int a = 5;
  int b = 3 * a + 1; 
```

Mapping variables:

 * a is -8(%rbp)
 * b is -16(%rpb)
 * Our temporary for (3*a) and (3*a+1) is %r11

Assembly:

```
# int a = 5;
  mov $5, -8(%rbp)

# int b = 3 * a + 1;
  mov -8(%rbp), %r11
  imulq $3, %r11
  inc %r11
  mov %r11, -16(%rbp)
```

### Which Registers

 * There are two pure temporary registers: %r10 and %r11.
   - Temporary registers go bad when you call a function.
 * There are five available callee-saved registers: %rbx, %r12-%r15
   - These are safe across function calls, but if you use them in your function you need to save them in your prologue and restore them in your epilogue before returning.
 * The first six function arguments go in: %rdi, %rsi, %rdx, %rcx, %r8, %r9
   - These are temporary registers and can be re-used as such.
 * The value returned by a function goes in %rax.
   - This is also a temporary, but some instructions (e.g. idiv) write to it.

### if statements

```
// Case 1
if (x < y) {
  y = 7;
}

// Case 2
if (x < y) {
  y = 7;
}
else {
  y = 9;
}
```

Variable Mapping:

 * x is -8(%rbp)
 * y is -16(%rbp) or, temporarily, %r10

Case 1:

```
  # if (x < y)
  mov -16(%rbp), %r10  # cmp can only take one indirect arg
  cmp %r10, -8(%rbp)   # cmp order backwards from C
  jge else1:           # condition reversed, skip block unless cond

  # y = 7
  movq $7, -16(%rbp)    # need suffix to set size of "7"

else1:
  ...
```

Case 2:

```
  # if (x < y)
  mov -16(%rbp), %r10   # cmp can only take one indirect arg
  cmp %r10, -8(%rbp)    # cmp order backwards
  jge else1:            # condition reversed, skip block unless cond

  # then {
  # y = 7
  movq $7, -16(%rbp)    # need suffix to set size of "7"
  
  j done1               # skip else

  # } else {
else1:
  # y = 9
  movq $9, -16(%rbp)

  # }
done1:
  ...
```

### do-while loops

```
do {
  x = x + 1;
} while (x < 10);
```

Variable Mapping:

 * x is -8(%rbp)

```
loop:
  add $1, -8(%rbp)

  cmp $10, -8(%rbp)   # reversed for cmp arg order
  jge loop            # sense reversed

  # ...
```

### while loops

```
while (x < 10) {
  x = x + 1;
}
```

Variable mappings:

 * x is -8(%rbp)

```
loop_test:
  cmp $10, -8(%rbp) # reversed for cmp
  jl loop_done      # reversed twice

  add $1, -8(%rbp)
  j loop_test
  
loop_done:
```

### Complex for loop

```
for (int i = 0; i < 10 && x != 7; ++i) {
  x = x + 3;
}
```

Variable mappings:

 * x = -8(%rbp)
 * i = %rcx
 * (i < 10) = %r8
 * (x != 7) = %r9
 * (i < 10 && x != 7) = %r10
 
```
  # for var init
  # i = 0
  mov $0, %rcx
  
for_test:
  # %r8 = i < 10
  cmp $10, %rcx 
  setle %r8
  
  # %r9 = x != 7 
  cmp $7, -8(%rbp)
  setne %r9
 
  # %r10 = full cond
  mov %r10, %r9
  and %r8, %r10   # bitwise and (&) of single bits is logical and (&&)
  
  # for condition
  cmp $0, %r10
  je for_done

  # loop body
  # {
  
  add $3, -8(%rbp)

  # }

  # for increment
  inc %rcx
  jmp for_test
  
for_done:
  ...
```

# Example Program

```
int main(...) 
{
  long x = read_int();
  long y = read_int();
  long z = foo(x, y);
  print_int(z);
  exit(0);
}

void print_int(long k) 
{
   printf("Your number is: %ld\n", k);
}

long read_int()
{
  long y;
  printf("Type in a number:\n");
  scanf("%ld", &y);
  return y;
}

long foo(long a, long b)
{
  b = bar(b + 1); 
  b = bar(b + 1);
  return 2 * a + b + 3;
}

long bar(int x) {
  if (x < 10) {
     return x;
  }
  else {
     return x % 20;
  }
}
```

To write this in assembly, we go one function at a time.

### function: bar

Signature and pseudocode:

```
long bar(int x) {
  if (x < 10) {
     return x;
  }
  else {
     return x % 20;
  }
}
```

Variable mappings:

 * x is %rdi
 * temporary 20 is %r10

Skeleton:

```
bar:
  enter $0, $0
  
  ...
 
  leave
  ret
```

Write the body:

```
bar:
  enter $0, $0
 
  cmp $10, %rdi
  bge bar_then
  jmp bar_else

bar_then:
  mov %rdi, %rax
  jmp bar_done
  
bar_else:
  mov %rdi, %rax
  mov $0, %edx
  mov $20, %r10 
  idiv %r10
  mov %rdx, %rax

bar_done:
  leave
  ret
```

### function: foo

Signature and pseudocode:

```
long foo(long a, long b)
{
    b = bar(b + 1); 
    b = bar(b + 1); 
    return 2 * a + b + 3;
}
```

Variable mappings:

The arguments a and b are needed after a function call, so we copy them to a callee-save register. These could also be put on the stack. Since we need the argument values after two function calls, a caller-save pattern would be less effective.

 * a is %r14
 * b is %r15
 * temporary is %r10, if needed

Skeleton:

```
foo:
  push %r14
  push %r15
  enter $0, $0
  
  ...
  
  leave
  pop %r15
  pop %r14
  ret
```

Write the body:

```
foo:
  push %r14
  push %r15
  enter $0, $0
 
  # move the arguments to callee-save registers
  mov %rdi, %r14
  mov %rsi, %r15

  # b = bar(b + 1)
  mov %r15, %rdi
  inc %rdi
  call bar
  mov %rax, %r15
  
  # b = bar(b + 1)
  mov %r15, %rdi
  inc %rdi
  call bar
  mov %rax, %r15
 
  # 2 * a + b + 3
  mov %r14, %rax
  add %rax, %rax
  add %r15, %rax
  add $3, %rax 
  
  leave
  pop %r15
  pop %r14
  ret
```

### read_int

Signature and pseudocode:

```
long read_int()
{
  long y;
  printf("Type in a number:\n");
  scanf("%ld", &y);
  return y;
}
```

Variable mappings:

Note that y must be on the stack, since we’re taking its address for scanf.

 * y is -8(%rbp)

Skeleton:

```
read_int:
  enter $16, $0  # Align stack & allocate 1 local

  ...

  leave
  ret
```

Body:

```
read_int:
  enter $16, $0   # Align stack & allocates an 16-byte (2-slot) stack frame

  mov $read_int_prompt, %rdi
  mov $0, %al    # required for vararg functions like printf
  call printf

  mov $read_int_format, %rdi
  lea -8($rbp), %rsi
  mov $0, %al
  call scanf

  mov -8($rbp), $rax

  leave
  ret
  
...

.data
read_int_prompt: 
  .string "Type in a number:\n"
read_int_format:
  .string "%d"
```

### print_int

```
void print_int(long k) 
{
   printf("Your number is: %ld\n", k);
}
```

Variable mappings:

 * k moves from %rdi to %rsi

Skeleton:

```
print_int:
  enter $0, $0

  ...

  leave
  ret
```

Body:

```
print_int:
  enter $0, $0

  mov %rdi, %rsi
  mov $print_int_format, %rdi
  mov $0, %al    # required for vararg functions like printf
  call printf
 
  leave
  ret
 
...
 
.data
print_int_format:
  .string "Your number is: %d\n"
```

### main

Signature & Pseudocode:

```
int main(...) 
{
  long x = read_int();
  long y = read_int();
  long z = foo(x, y);
  print_int(z);
  exit(0);
}
```

Variable mappings:

 * x is -8(%rbp)
 * y is -16(%rbp)
 * z is -24(%rbp)

Skeleton

```
main:
  enter $32, $0     # Allocate stack frame with 3 slots + 1 for alignment

  ...

  leave
  ret
```

Body:

```
main:
  enter $32, $0    # Allocate stack frame with 3 slots + 1 for alignment

  call read_int
  mov %rax, -8(%rbp)
  
  call read_int
  mov %rax, -16(%rbp)

  mov -8(%rbp), %rdi
  mov -16(%rbp), %rsi
  call foo
  mov %rax, -24(%rbp)

  mov -24(%rbp), %rdi
  call print_int

  mov $60, %rax
  syscall
  
  leave
  ret
```
