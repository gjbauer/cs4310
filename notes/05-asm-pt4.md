Lecture Notes: 05 More Assembly Examples · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 05 More Assembly Examples
========================================

2025 Jan 30·4 mins

### Assembly Steps [#](#assembly-steps)

For each function

* Variable mapping
* Skeleton (prologue; including stack allocs, body, epilogue)

### Example 1: Fives [#](#example-1-fives)

Key point here: Dealing with individual characters in strings.

```
    .global main
    .data
all_fives: .string "all fives"
not_all: .string "not all fives"
intfmt: .string "%d\n"

/*
    - take a string as argv[1]
    - determine if all characters are 5's
    - print "all fives" if so
    - print "not all fives" if not
*/

.text

/*
int
all_match(char c, char* text)
{
    if (*text == 0) {
        return 1;
    }

    if (*text == c) {
        return all_match(c, text + 1);
    }

    return 0;
}
*/

/* variable mapping:
   - c: %rdi
   - text: %rsi
*/

all_match:
    enter $0, $0

    push %rdi
    push %rsi
    mov %rsi, %rdi
    call puts
    pop %rsi
    pop %rdi

    cmpb $0, (%rsi)
    jne am1_done
am1_then:
    mov $1, %rax
    jmp all_match_done 
am1_done:

    cmpb %dil, (%rsi)
    jne am2_done
am2_then:
    // correct value in %rdi 
    add $1, %rsi
    call all_match
    // mov %rax, %rax
    jmp all_match_done

am2_done:
    mov $0, %rax

all_match_done:

    push %rax
    push %rax
    mov $intfmt, %rdi
    mov %rax, %rsi
    mov $0, %al
    call printf
    pop %rax
    pop %rax

    leave
    ret


/*
int
all_match(char c, char* text)
{
    for (long ii = 0; text[ii] != 0; ++ii) {
        if (text[ii] != c) {
            return 0;
        }
    }
    return 1;
}
*/

/*
int 
main(int argc, char* argv[])
{
    if (all_match('5', argv[1])) {
        say "all fives";
    }
    else {
        say "not all fives";
    }
    return 0;
}
*/

/* variable mapping for main:
   - %r12 is the result of all_match
*/

main:
    push %r12
    enter $8, $0

    mov $'5, %rdi 
    mov 8(%rsi), %rsi
    call all_match
    mov %rax, %r12

    cmp $0, %r12
    je main_else
main_then:
    mov $all_fives, %rdi
    jmp main_done
main_else:
    mov $not_all, %rdi
main_done:     
    call puts

    mov $0, %rax
    leave
    pop %r12
    ret

```

Distraction: Address Space [#](#distraction-address-space)
----------------------------------------------------------

```

int glo = 5;
const char* ro_glo = "read only global";

int
main(int argc, char* argv[])
{
    int loc = 5;
    int* heap = malloc(sizeof(int));

    printf("&glo = %lx\n", &glo);
    printf("ro_glo = %lx\n", ro_glo);
    printf("main = %ld\n", main);
    printf("&loc = %lx\n", &loc);
    printf("heap = %lx\n", heap);

    free(heap);
    return 0;
}

```

Then draw the address space diagram.

Example 2: Sum Array [#](#example-2-sum-array)
----------------------------------------------

```
#include <stdio.h>
#include <stdlib.h>

long
sum_array(long* xs, long nn)
{
    long yy = 0;
    for (long ii = 0; ii < nn; ++ii) {
        yy += xs[ii];
    }
    return yy;
}

int
main(int argc, char* argv[])
{
    if (argc != 2) {
        puts("usage: ./sum_array N");
        return 1;
    }

    long nn = atol(argv[1]);
    long* xs = malloc(nn * sizeof(long));

    for (long ii = 0; ii < nn; ++ii) {
        int count = scanf("%ld", &(xs[ii]));
        if (count != 1) { abort(); }
    }

    printf("sum = %ld\n", sum_array(xs, nn));

    free(xs);
    return 0;
}

```

And in assembly:

```
.global main
.text
main:
  push %r12 /* n */
  push %r13 /* xs */
  /* rcx is temp and ii */
  enter $0, $0

  cmp $2, %rdi
  je args_ok
  call abort

args_ok:
  /* long n = atol(argv[1]); */
  mov 8(%rsi), %rdi
  call atol
  mov %rax, %r12

  /* long* xs = alloca(n * sizeof(long)) */
  mov $8, %rcx
  imul %rcx

  /* alloca */
  sub %rax, %rsp
  mov %rsp, %r13
  /* end alloca */
  /*
  mov %rax, %rdi
  call malloc
  mov %rax, %r13
  */

  /* align stack, reguardless of value of n */
  mov $15, %rcx
  not %rcx
  and %rcx, %rsp

  /* for (ii = 0; ii < n; ++ii) { xs[ii] = ii } */
  mov $0, %rcx
fill_loop_cond:
  cmp %r12, %rcx
  jge fill_loop_done

  push %rcx
  push %rcx
  mov $longfmt, %rdi
  lea (%r13,%rcx,8), %rsi
  mov $0, %al
  call scanf
  pop %rcx
  pop %rcx

  cmp $1, %rax
  je scanf_ok

  call abort

scanf_ok:


  inc %rcx
  jmp fill_loop_cond

fill_loop_done:

  /* printf("...", sum_array(xs)); */
  mov %r13, %rdi
  mov %r12, %rsi
  call sum_array

  mov $fmt, %rdi
  mov %rax, %rsi
  mov $0, %al
  call printf

  /*
  mov %r13, %rdi
  call free
  */

  leave
  pop %r13
  pop %r12
  ret

sum_array:
  /* %rdi is xs */
  /* %rsi is nn */
  /* %rcx is ii */
  /* %rax is yy */
  enter $0, $0

  mov $0, %rax
  mov $0, %rcx
sum_loop_cond:
  cmp %rsi, %rcx
  jge sum_loop_done

  add (%rdi,%rcx,8), %rax

  inc %rcx
  jmp sum_loop_cond

sum_loop_done:
  leave
  ret

.data
fmt: .string "sum = %ld\n"
longfmt: .string "%ld"

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 03 More ASM

2025 Jan 24](/classes/2025-01/cs4310/notes/03-more-asm/)

[Lecture Notes: 06 Command Line Args

2025 Feb 01
→←](/classes/2025-01/cs4310/notes/06-asm-argv/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)