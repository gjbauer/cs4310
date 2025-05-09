Lecture Notes: 07 Syscalls · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 07 Syscalls
==========================

2025 Feb 03·Updated: 2025 Feb 07·2 mins

lines.S

```
#include <stdio.h>

int
main(int argc, char* argv[])
{
    char* buf[128];
    FILE* msg = fopen("msg.txt", "r");
    int yy = fread(buf, 128, 1, msg); 
    buf[yy] = 0;
    printf("%s\n", buf);
    fclose(msg);
    return 0;
}

```

* Talk about stdin, stdout, stderr
* Talk about file descriptor table
* Rewrite this with syscall wrappers.

```

/* lines.S - reads "msg.txt", counts lines ('\n' characters) */

  .global main
  .text
main:
  push %r14   /* file descriptor for msg.txt */
  push %r15   /* size of msg.txt contents */
  /* %rcx is loop counter */
  /* %rdx is count */
  /* %rax is current byte */
  enter $256, $0
  /* 0(%rsp) is read buffer, 192 bytes */
  /* 192(%rsp) is print buffer, 64 bytes */
  mov $intro, %rdi
  call strlen

  mov $1, %rdi       /* fd 1 is stdout */
  mov $intro, %rsi   /* buffer */
  mov %rax, %rdx     /* size */
  mov $1, %rax       /* syscall 1 is write */
  syscall

  mov $file, %rdi   /* file name */
  mov $0, %rsi      /* O_RDONLY ; see bits/fcntl-linux.h */
  mov $0, %rdx      /* mode is unused on read */
  mov $2, %rax      /* syscall 2 is open */
  syscall
  mov %rax, %r14    /* save fd */

  mov %r14, %rdi    /* read from file */
  lea 0(%rsp), %rsi /* read to stack */
  mov $192, %rdx    /* buffer size */
  mov $0, %rax      /* syscall 0 is read */
  syscall
  mov %rax, %r15    /* save bytes read */

  mov $0, %rcx
  mov $0, %rdx
loop_cond:
  cmp %r15, %rcx
  jge loop_done

  mov $0, %rax
  mov (%rsp,%rcx,1), %al

  cmp $'\n', %al
  jne loop_next

  inc %rdx

loop_next:
  inc %rcx
  jmp loop_cond

loop_done:
  lea 192(%rsp), %rdi
  mov $fmt, %rsi
  /* count already in %rdx */
  mov $0, %al
  call sprintf

  mov $1, %rdi        /* fd 1 is stdout */
  lea 192(%rsp), %rsi /* buffer */
  mov %rax, %rdx      /* size */
  mov $1, %rax        /* syscall 1 is write */
  syscall

  /* TODO: look up close syscall to close file */

  mov $0, %rdi
  mov $60, %rax   /* syscall 60 is exit */
  syscall

  leave
  pop %r15
  pop %r14
  ret

  .data
intro: .string "lines in msg.txt:\n"
file: .string "msg.txt"
fmt: .string "lines = %ld\n"

```

ref: <https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/>

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 06 Command Line Args

2025 Feb 01](/classes/2025-01/cs4310/notes/06-asm-argv/)

[Lecture Notes: 09 Brk

2025 Feb 06
→←](/classes/2025-01/cs4310/notes/09-brk/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)