AMD64 Assembly Quick Reference
2025 Jan 21
·
Updated: 2025 Jan 28
·
6 mins
For Prof Tuck’s cs4310, Spring 2025.

This uses AT&T syntax (per norms for the GNU C compiler).

Registers#
In AMD64, we have 16 registers that can each hold 64-bits (8 bytes):

Group 1:

%rax - accumulator: arithmetic source & destination
%rcx - counter: loop index
%rdx - data: arithmetic source
%rbx - base: historically used for segmented addressing
Group 2:

%rsi - source index
%rdi - destination index
%rsp - stack pointer
%rbp - base pointer
Group 3:

%r8, %r9, …, %r15
Groups 1 and 2 are the extended form of the old 16 bit registers from the Intel 8086, while Group 3 was added specifically for AMD64.

Shorter parts of these registers are also accessible by the hisorical names:

{% striped() %}

<style type="text/css">
.tg  {border-collapse:collapse;border-spacing:0;}
.tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  overflow:hidden;padding:10px 5px;word-break:normal;}
.tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
.tg .tg-1wig{font-weight:bold;text-align:left;vertical-align:top}
.tg .tg-0lax{text-align:left;vertical-align:top}
</style>
<table class="tg"><thead>
  <tr>
    <th class="tg-1wig">64-bit register</th>
    <th class="tg-1wig">Low 32-bits</th>
    <th class="tg-1wig">Low 16 bits</th>
    <th class="tg-1wig">Low 8 bits</th>
    <th class="tg-1wig">Byte 7*</th>
  </tr></thead>
<tbody>
  <tr>
    <td class="tg-0lax">%rax</td>
    <td class="tg-0lax">%eax</td>
    <td class="tg-0lax">%ax</td>
    <td class="tg-0lax">%al</td>
    <td class="tg-0lax">%ah</td>
  </tr>
  <tr>
    <td class="tg-0lax">%rsi</td>
    <td class="tg-0lax">%esi</td>
    <td class="tg-0lax">%si</td>
    <td class="tg-0lax">%sil</td>
    <td class="tg-0lax">n/a</td>
  </tr>
  <tr>
    <td class="tg-0lax">%r8</td>
    <td class="tg-0lax">%r8d</td>
    <td class="tg-0lax">%r8w</td>
    <td class="tg-0lax">%r8b</td>
    <td class="tg-0lax">n/a</td>
  </tr>
</tbody>
</table>

{% end %}				
The names for the other registers follow the pattern within the same group.

For the Group 1 registers, the high and low bytes of the low 16-bits be accessed by dedicated names (%ah, %al, %ch, %cl, etc).

There are a couple of other registers worth remembering:

%rip - Instruction pointer. Points to next instruction to execute. Not readable.
%flags - (%eflags, %rflags) - Each bit means something specific. Not writable.
Instructions#
Arithmetic Instructions#
{% striped() %}

Instruction	Description
mov %rxx, %ryy	Copy data from register %rxx to %ryy
—————–	——————————————-
inc %rxx	Add one to %rxx
dec %rxx	Subtract one from rxx
neg %rxx	Negate %rxx
not %rxx	Bitwise NOT %rxx
—————–	——————————————-
add %rxx, %ryy	%ryy += %rxx
sub %rxx, %ryy	%ryy -= %rxx
or %rxx, %ryy	%ryy = %rxx OR %ryy (bitwise)
and %rxx, %ryy	%ryy = %rxx AND %ryy (bitwise)
—————–	——————————————-
imul %rxx	%rdx:%rax = %rax * %rxx
imul %rxx, %ryy	%ryy = %ryy * %rxx (truncated to 64-bits)
idiv %rxx	%rax = %rdx:%rax / %rxx; %rdx = remainder
{% end %}	
Flow Control and Logic#
Note: The argument order for “cmp” is backwards here.

  ## This is "%rbx > %rax"?
  cmp %rax, %rbx
  jg rbx_is_bigger
{% striped() %}

Instruction	Description
cmp %ryy, %rxx	Compares the two registers, updating the flags register (note: order seems backwards)
—————-	————————————————————–
je label	Jump if equal (if previous cmp set equal flag; %rxx == %ryy)
jne label	Jump if not equal (%rxx != %ryy)
jl label	Jump if less than (%rxx < %ryy)
jle label	<=
jg label	>
jge label	>=
—————-	————————————————————–
sete %rzz	Set %rzz if %rxx == %ryy in the previous cmp, else clear it.
setg %rzz	Set %rzz if %rxx > %ryy
setl %rzz	Set %rzz if %rxx < %ryy
{% end %}	
Function Call and Stack#
{% striped() %}

Instruction	Description
push %rxx	Copy %rxx to stack @ %rsp, move down %rsp
pop %rxx	Copy from stack @ %rsp to %rxx, move up %rsp
enter $NN, $0	Allocate a stack frame with NN bytes of space
leave	Deallocate a stack frame
call label	Push $rip and jump to the address of the “label” function
{% end %}	
Memory and Immediate Arguments#
In addition to operating on registers, many instructions can accept alternate modes that operate on data in memory or on constant values.

{% striped() %}

Example Instruction	Description
add (%rcx), %rdx	%rdx = %rdx + (value at address in %rcx)
add $10, %rdx	%rdx = %rdx + 10
addq $10, 2(%e10, %e11, 2)	(the value at %e10+2*%e11) += 10
add -16(%rsp), %rax	%rax += the value 16 bytes below where %rsp points
{% end %}	
There’s a special instruction, lea, that calcuates an address as if it were going to access an argument in memory but gives you the address as its output.

{% striped() %}

Example Instruction	Description
lea -16(%rsp), %rax	%rax = %rsp - 16
{% end %}	
Instruction suffixes: Instructions can have a single letter suffix added to indicate hte size of the value operated on: b, w, l, q for 1, 2, 4, 8 bytes.

{% striped() %}

Example Instruction	Description
movw $10, (%rdx)	Move a 16-bit (2 byte, short) int to the address in %rdx
movq %10, (%rdx)	Move a 64-bit (8 byte, long) int to the address in %rdx
{% end %}	
Calling a Function#
To call a function with the “call” instruction, you must first:

Put arguments in the appropriate registers / stack.
%rdi, %rsi, %rdx, %rcx, %r8, %r9, then the stack
Think about “caller save”/“temporary” registers.
That’s all the data registers except %rbx and %r12-%r15.
Assume calling any function corrupts them.
You can push these before the call and pop after, but a different allocation with stack / callee-save might work better.
Make sure %rsp points to an address divisible by 16 bytes.
This is not true when you enter a function, because call pushes a return address.
Pushing %ebp (e.g. with “enter $0,…”) corrects this.
Make sure you do an even number of pushes / reserve stack space in multiples of 16 bytes.
Once the function returns, your result will be in %rax. An optional second result is returned in %rdx.

Writing a Function#
These registers are callee-saved. If you want to use them, save them to the stack in your prologue and restore them in your epilogue:

%rbx, %r12-%r15
The stack registers (%rsp, %rbp) are technically callee save, but this is handled by the standard use of “enter” and “leave” calls.

Register Allocation#
Function arguments: %rdi, %rsi, %rdx, %rcx, %r8, %r9
Temporary registers: %r10, %r11, (argument registers)
Safe registers: %r12, %r13, %r14, %r15, %rbx
How to map variables / values to locations:

Local variables that get their address taken should go on the stack.
Local variables that get used before and after a function call should go in a safe (callee-saved) register.
Local variables that don’t need to survive a function call can go in temporary registers.
Temporary values should go in temporary registers.
The caller-save strategy, where temporary registers are pushed/popped around a function call to preserve them can be used but tends to be more annoying than using safe registers.

Compiling ASM with GCC#
$ gcc -no-pie -o foo foo.s
By default, program starts at _start
If you declare “.global main”, program will start at main.
The “-no-pie” flag is needed for Ubuntu 18.04. This disables ASLR for the binary, which hurts security.
Compiling ASM without libc#
$ gcc -nostdlib -no-pie -o foo foo.s
OR
$ as -o foo.o foo.s
$ ld -o foo foo.o
Using GDB with ASM#
Compile with -g
Use break to set breakpoints on labels or line numbers.
Print registers with p $rax or p/x $rax.
Print all registers with info reg
External Resources#
Brown CS0330: Great x86-64 assembly cheat sheet.
