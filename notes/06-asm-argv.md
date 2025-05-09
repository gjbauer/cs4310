Lecture Notes: 06 Command Line Args · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 06 Command Line Args
===================================

2025 Feb 01·Updated: 2025 Feb 04·1 min

args.S

```
   .global main
    .text

main:
    push %r12 // argv
    push %r13 // argc
    enter $0, $0

    mov %rdi, %r13
    mov %rsi, %r12
aa:
    mov (%r12), %rdi
    cmp $0, %rdi
    je bb

    call puts
   
    add $8, %r12
    jmp aa

bb:
    leave
    pop %r13
    pop %r12
    ret

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 05 More Assembly Examples

2025 Jan 30](/classes/2025-01/cs4310/notes/05-asm-pt4/)

[Lecture Notes: 07 Syscalls

2025 Feb 03
→←](/classes/2025-01/cs4310/notes/07-syscall/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)