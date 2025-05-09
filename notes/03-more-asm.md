Lecture Notes: 03 More ASM · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 03 More ASM
==========================

2025 Jan 24·3 mins

Another example:

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

AMD64 Assembly Review [#](#amd64-assembly-review)
-------------------------------------------------

Registers: rax, rcx, rdx, rbx, rdi, rsi, rbp, rsp, r9, …, r15

Size variants: rax, eax, ax, ah/al

Calling convention:

* arguments go in, in order: rdi, rsi, rdx, rcx, r8, r9, (stack)
* return value comes out in rax
* (second return in rdx)
* To call a varargs, function, you must first zero %al

Rewrite the assembly manually [#](#rewrite-the-assembly-manually)
-----------------------------------------------------------------

```
  .global main
# add2.s
  
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

Lecture Notes: 02 ASM and C

2025 Jan 22](/classes/2025-01/cs4310/notes/02-asm-and-c/)

[Lecture Notes: 05 More Assembly Examples

2025 Jan 30
→←](/classes/2025-01/cs4310/notes/05-asm-pt4/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)