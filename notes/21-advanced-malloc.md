Lecture Notes: 21 Advanced Malloc · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 21 Advanced Malloc
=================================

2025 Mar 10·Updated: 2025 Mar 12·4 mins

Optimizing a Memory Allocator [#](#optimizing-a-memory-allocator)
=================================================================

Buddy System [#](#buddy-system)
-------------------------------

Buddy trick:

* Power of two allocations.
* We can split a power of two block in half, into two “buddies”.
  + e.g. The size 128 range 1024-1152 can be split into 1024-1088 and 1088-1152.
  + 128 is 2^7
  + The addresses of the two buddies (1024 and 1088) differ in only bit #7.
  + That’s 0100 0000 0000 and 0100 0100 0000
  + So given either address, we can find the other in O(1) time by XORing with
    2^7.
* If our allocations have a “used” header bit, then when either is freed we can
  find the other, check if it’s free, and merge in O(1) time.

Example:

* Start with a constant heap size, e.g. 1MB
  + If we run out of space, we allocate a whole new heap.
  + So we innately need to handle multiple arenas - each buddy system
    heap is a separate arena.
* Have an array of free list pointers.
  + One bucket for each power of two up to the heap size.
  + For a 1MB = 2^20 B heap, that’s slots numbered 0..20. We don’t use index
    zero since we can’t do 1B allocations, but that’s fine.
  + Doubly linked for O(1) insert / remove.
* Allocate 128k
  + Once: Split 1MB => 512k, 512k => 256k, 256k => 128k
  + Again: No need to split, we’ve got one.
  + Again: Split 256k => 128k
  + Free the first one, no merge.
  + Free the last one, no merge.
  + Free the middle one, merge back to 1MB.

Problem: Our free list structure too big. [#](#problem-our-free-list-structure-too-big)
---------------------------------------------------------------------------------------

```
struct cell {
    // in both cell and header
    long size;
    long arena;
    bool used;
    
    // not in header
    cell* next;
    cell* prev;
};

```

sizeof(cell) = 40

On AMD64, a reasonable minimum header size is 8. Pointers need to be 8-byte
aligned, so using a size 8 header maintains that alignment for the pointer we
return to the user.

Similarly, a reasonable minimum cell size is 16, since nobody should really
expect the allocator to be efficient for allocations smaller than 8.

So let’s fit in those sizes:

```
struct cell {
    // in both cell and header
    uint32_t arena;  // up to 4B of them
    uint8_t  size;   // store which power of two
    uint8_t  used;   // used flag
    uint16_t _pad;
    
    // not in header
    int32_t next;   // offset from start of arena, -1 is EOL
    int32_t prev;   // offset from start of arena, -1 is EOL
};

```

Crazy Chunk Allocator [#](#crazy-chunk-allocator)
-------------------------------------------------

Some of the ideas here are losely based on the jemalloc allocator from
Facebook, but much simpler.

How can we do allocations with no minimum size?

* Worst case: 1 byte allocations
* We can’t even have a size field in our allocations, much less stick
  a linked list in there.
* So we need to put that data outside the allocation.
* Idea: Have a whole page of allocations of the same size, and put our
  metatdata at the beginning of the page.
* Metadata:
  + The size of the items in this chunk (8 bytes).
  + One bit per item to track which items in the chunk are allocated.
* For 1 byte items in 4k chunk, allocated bitmap:
  + 4096 bits is 512 bytes. That leaves 4096-512-8 = 3576 bytes to allocate
* For 2 byte items in 4k chunk, allocated bitmap:
  + 2k bits is 256 bytes
* Problem 1: How to find metadata?
  + Round address down to the closest multiple of 4096.
* Problem 2: Large allocations don’t fit on a page.
  + Not a problem. These allocations will be some number of whole pages,
    so we can just have a size field a the start of the first page which
    and we can tell that it’s a large allocation with no bitmap from
    the size field.
* Problem 3: Medium size allocations (e.g. 2048 bytes).
  + Can’t fit two 2k items on one page with metadata.
  + Solution: Make the chunks bigger than one page.
  + In jemalloc they use 2MB chunks, but that’s probably too big
  + The chunk size should be bigger than the largest allocation handled
    by chunks. Why?

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 18 Condvar

2025 Mar 05](/classes/2025-01/cs4310/notes/19-condstack/)

[Lecture Notes: 22 Garbage Collection

2025 Mar 12
→←](/classes/2025-01/cs4310/notes/22-gc-slides/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)