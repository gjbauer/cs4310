Lecture Notes: 12 Mmap (2) · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 12 Mmap (2)
==========================

2025 Feb 16·Updated: 2025 Feb 20·5 mins

Previously we looked at the brk syscall, which provides the
traditional implementation of heap allocation.

Today we’re going to look at the one syscall to rule them all:
mmap(2).

In the executon of most modern computer programs, all runtime changes
to the address space mappings are done with the mmap syscall.

Staring at the mmap API, the most obvious use case is for I/O.

save-array.c:

```
#include <sys/mman.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

// create array.mem
// save squares 1..10

void
assert_ok(long rv, char* call)
{
    if (rv == -1) {
        fprintf(stderr, "Failed call: %s\n", call);
        perror("Error:");
        exit(1);
    }
}

int
main(int _ac, char* _av[])
{
    int size = 10 * sizeof(int);

    // Talk about ORing flags.
    int fd = open("./array.mem", O_CREAT|O_TRUNC|O_RDWR, 0644);
    assert_ok(fd, "open");

    int rv = ftruncate(fd, size);
    assert_ok(rv, "ftruncate");
    
    int* xs = mmap(0, size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    assert_ok((long)xs, "mmap");

    for (int ii = 0; ii < 10; ++ii) {
        xs[ii] = ii * ii;
    }
   
    rv = munmap(xs, size);
    assert_ok(rv, "munmap");

    rv = close(fd);
    assert_ok(rv, "close");

    return 0;
}

```

print-array.c:

```
#include <sys/mman.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

// create array.mem
// save squares 1..10

void
assert_ok(long rv, char* call)
{
    if (rv == -1) {
        fprintf(stderr, "Failed call: %s\n", call);
        perror("Error:");
        exit(1);
    }
}

int
main(int _ac, char* _av[])
{
    int size = 10 * sizeof(int);

    // Talk about ORing flags.
    int fd = open("./array.mem", O_RDWR, 0644);
    assert_ok(fd, "open");

    int* xs = mmap(0, size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    assert_ok((long)xs, "mmap");

    for (int ii = 0; ii < 10; ++ii) {
        printf("%d: %d\n", ii, xs[ii]);
    }
   
    int rv = munmap(xs, size);
    assert_ok(rv, "munmap");

    rv = close(fd);
    assert_ok(rv, "close");

    return 0;
}

```

But mmap still works if there’s no file descriptor specified, using
the MAP\_ANONYMOUS flag. This is how modern systems allocate memory.

Notes:

* All mappings are backed by something.
* File mappings are backed by the filesystem.
* Anonymous mappings are backed by swap.

Memory Allocator [#](#memory-allocator)
---------------------------------------

* A new process starts with no heap space allocated.
* To get heap space, we need to make a system call.
  + Traditionally, the call was sbrk(2)
  + sbrk still exists in POSIX, but nobody uses it.
  + We allocate memory with mmap.
* To allocate private memory with mmap, we use MAP\_PRIVATE | MAP\_ANONYMOUS
* The kernel can allocate in 4K pages. Requesting smaller allocations
  is meaningless - if you ask for 1 byte with mmap, you get 4K.

So a memory allocator wants to use mmap to get memory from the OS and fix a
couple of sharp edges in the process:

* We want to handle allocations under 1 page.
* We want to do many small allocations efficiently.
* We want to be able to free memory without remembering the size of the allocation.
* Especially for small allocations, we want to reuse memory rather than returning it
  to the OS and then asking for it back.

The traditional solution to this problem is to maintain a “Free List”.

* This is a list of free blocks with, logically, location and size for each.

To allocate a chunk of memory, we do the following:

* See if there’s a block of memory on the free list big enough to use.
* If not, allocate a new block with mmap.
* If the block is too big, split it and return the extra to the free list.
* Remember the size of the allocated block for later.
* Return the selected block to the user.

To free a chunk of memory:

* Stick it on the free list.

Problems and Solutions [#](#problems-and-solutions)
---------------------------------------------------

This leaves some problems:

Problem 1: Where do we store the size of a chunk?

* We’re the memory allocator. We can just allocate a bigger chunk and
  store the size *in* the chunk.
* We want to put it at the beginning of the chunk so we can find it again.

So when someone requests a chunk of size B, we allocate B + 8 bytes
layed out as follows:

* Size (size\_t = 8 bytes)
* That many bytes of memory for the user.

We return a pointer to the memory after the size.

When the memory is freed, we can find the size by subtracting 8 bytes
from the pointer we got.

Problem 2: Where do we store the free list?

* The free list is made up of chunks of free memory.
* We can store the list in the memory itself.
* Easy to lay out a singly or doubly linked list in the
  memory.
* This makes our minimum actual memory allocation be the
  size of a list cell.

Problem 3: Fragmentation

```
   for (1..200) {
     xs[ii] = malloc(800);
   }
   
   for (1..10) {
     free(xs[ii]);
   }
   
   y = malloc(5000);

```

* We’d like to reuse the memory rather than requesting more from the OS.
* When we free memory, we want to check the free list to see if we can
  combine the chunk we’re freeing with other chunks already on the list.
* We may need to combine more than once (A,C on free list, free B).

Problem 4: Big Allocations

* What if a program requests 10 GB of RAM and then frees it?
* We’d like to return that to the OS, not put it on the free list.
* Solution: Send large allocations directly to mmap, and then do a
  munmap when that memory is freed.
* For allocations over some threshold size, the cost of the syscall
  is going to be irrelevent.
* In this case we always allocate some number of whole pages.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 11 No Class

2025 Feb 15](/classes/2025-01/cs4310/notes/11-no-class/)

[Lecture Notes: 13 Page Tables

2025 Feb 19
→←](/classes/2025-01/cs4310/notes/13-page-tables/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)