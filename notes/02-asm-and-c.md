Lecture Notes: 02 ASM and C · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 02 ASM and C
===========================

2025 Jan 22·Updated: 2025 Jan 23·6 mins

C -> ASM [#](#c---asm)
----------------------

* “Programming” means “writing C code”.
* On Linux-like (UNIX, \*nix, POSIX) systems, the operating system
  API is primarily exposed to C programs through the system C library.
* The hardware doesn’t run C though - it runs amd64 machine code (on your
  laptop) or ARM machine code (on your phone) or maybe some other machine
  code.
* Machine code is for machines, not humans, so it’s hard to read.
* Machine code is a series of instructions. If you write the instructions
  down as text, you get assembly language.
* To run a C program, you need to translate to machine code (or “binary”).
* Conceptually, and historically, you first translate C -> ASM, then
  ASM -> binary.
* You can still do this if you explicitly ask for it.

Note: In some of our homeworks you will be writing ASM code. You
should *not* have a compiler do this for you. Submitting compiler
output for an assembly assignment is worth zero points.

Example:

```
// A C program is a collection of functions.
// Here's a minimal program with one function
int
main(int argc, char* argv[])
{
  printf("Hello C program\n");
  return 0;
}

```

```
# Direct C => binary
$ gcc -o hello hello.c
$ ./hello

# C => asm
$ gcc -S -o hello.s hello.c
# take a look at hello.s

# asm => binary
$ gcc -o hello hello.s
$ ./hello

```

Interesting stuff in hello.s:

* The string is there, but no newline.
* The main function exists
  + Starts at label “main”
  + Ends at “ret”.
  + Declared “.globl”
* In the main function another function is called - not printf, but puts.
* The optimizer got to us.

Let’s tell it to be less clever:

```
# C => asm
$ gcc -fno-builtin -S -o hello.s hello.c
# take a look at hello.s

```

* Now the string has a newline.
* And the function called is “printf”.

How about with two functions:

```
// add1.c
long
add1(long x)
{
    return x + 1;
}

int
main(int _ac, char* _av[])
  // initial _ marks args as not used
{
    long x = add1(5);
    printf("%ld\n", x);
    return 0;
}

```

```
# C => asm
$ gcc -S -o add1.s add1.c
# take a look at hello.s

```

* Two functions: add1, main
  + each starts at label, ends at “ret”
* In main, the value 5 is moved to “%rdi”
  + That must be where the function’s first argument goes.
  + No, that’s “%edi”
  + I said “%rdi”, wait a second…
* Then add1 is called
* In add1, the value from %rdi goes to some places.
* Eventually, “addq $1, …” happens to it.
* Back in main, %rax is moved to %rsi, and printf is called.

This almost makes sense, but it’s a bit of a mess. Let’s figure it out.

AMD64: ISA and ASM [#](#amd64-isa-and-asm)
------------------------------------------

Intel released the 8086 processor in 1978. It was based on the earlier 8008
processor from 1972, but…

The 8086 was a 16-bit microproessor. That means:

* It had a 16-bit data bus connecting it to memory and maybe other stuff.
  + That means a processor and RAM connected by 16 wires.
* How much RAM can we address with 16 bits?
* In addtion to RAM, this system gives us another place to put stuff called
  registers. For a 16-bit processor, each register is 16 bits.
* The 8086 had 9-ish registers:
  + “general purpose”: ax, cx, dx, bx, si, di, bp, sp,
  + “special purpose”: ip, (segment registers, status register)
* What processors do is execute instructions. Kinds of instructions:
  + Arithmetic: Example: add $5, %cx
  + Test: cmp $5, %cx
  + Conditional branch: jge bigger\_label
  + Movement instruction: mov (%sp), %dx
  + A bunch of other stuff. You’ll want to have a reference sheet.
* Instructions tend to operate on at least one register.
* Instructions can operate on memory addresses. If they do, the CPU needs
  to stop and read or write from RAM.

The 80386 or i386 was a 32-bit microprocessor, backwards compatible with
the 8086. This was the first “Intel x86” processor:

* It had a 32-bit data bus.
  + How much RAM can we address with 32-bits?
* It had 32-bit registers.
  + If you used the old names (eg. %ax), you got the least significant
    16-bits of the register.
  + Each register got a new name with an “e” at the front to refer to
    the full 32 bit “extended” register:
    - eax, ecx, edx, …

The AMD Athlon 64 was a 64-bit microprocessor, backwards compatible with the Intel
8086 and i386. This was the first “AMD64” processor:

* It had a 48-bit data bus, designed to be extended up to 64-bit later.
  + How much RAM can we address with 64 bits?
  + How about 48 bits?
* It had 64-bit registers.
  + If you used the old names (e.g. %ax, %rax), you got the least significant
    16 or 32 bits of the register.
  + Each register got a new name with an “r” at the front to refer to
    the full 64 bit register.
    - rax, rcx, rdx, …
  + 8 new general purpose registers were added: %r9, %r10, …, %r15

And that’s where we are today. Let’s write an add2 program by hand in amd64
assembly:

```
# add2.s

  .global main
  
  .text
# long add2(long x)
#   - the argument comes in in %rdi
#   - we return the result by putting it in %rax
add2:
  enter $0, $0
 
  # long y = x;
  mov %rdi, %rax
  
  # y = y + 2;
  add $2, %rax

  # return y;
  leave
  ret

main:
  enter $0, $0

  # long x = 5;
  mov $5, %rdi
  
  # y = add1(x)
  call add2
  # result in %rax

# printf("%ld\n", y)
#  - first arg goes in %rdi
#  - second arg goes in %rsi
#  - for a variable arg function, we need to zero %al
#    - %al is the bottom 8 bits of %ax/%eax/%rax
  mov $long_fmt, %rdi
  mov %rax, %rsi
  mov $0, %al
  call printf

  leave
  ret
  
  .data
long_fmt: .string "%ld\n"

```

To compile this simple hand-written assembly, we use:

```
$ gcc -no-pie -o add2 add2.s

```

Another Assembly Example [#](#another-assembly-example)
-------------------------------------------------------

* Scan through the AMD64 instruction list on course site.
* Example: cond\_br

```
$ gcc -no-pie -o prog prog.s

```

```
.global main
.text

main:
  enter $0, $0

  # print prompt
  mov $prompt, %rdi
  call puts

  mov $long_fmt, %rdi
  mov $num, %rsi
  mov $0, %al
  call scanf

  # copy value at address
  # with dollar sign, copy literal address
  mov num, %rax

  # if (%rax <= 10)
  cmp $10, %rax
  jle smaller_than_ten

bigger_than_ten:
  mov $bigger, %rdi
  jmp main_done

smaller_than_ten:
  mov $smaller, %rdi

main_done:
  call puts

  leave
  ret

.data
num: .string "12345678" # 8 bytes, to fit a long
prompt: .string "enter a number"
long_fmt: .string "%ld"
eol: .string "\n"
bigger: .string "bigger than ten"
smaller: .string "smaller than ten"

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 01 Intro

2025 Jan 20](/classes/2025-01/cs4310/notes/01-welcome/)

[Lecture Notes: 03 More ASM

2025 Jan 24
→←](/classes/2025-01/cs4310/notes/03-more-asm/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)