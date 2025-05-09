Lecture Notes: 10 Data Structures · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 10 Data Structures
=================================

2025 Feb 12·Updated: 2025 Feb 15·2 mins

I want to do a quick review of three common data structures and what
they look like in C.

C gives us two building blocks:

* Array: A fixed-length sequence of same-sized items in memory.
* Struct: A collection of a fixed number of fields of varying size/type.

These are sufficient to build whatever data structures we need, but we
definiely want to build more complex structures on top of them.

[#](#heading)
-------------

Variable length array:

* This is the most common way to store a variable length sequence,
  it’s a List in python, an array in JS, an ArrayList in Java, or a
  vector in C++.
* You’ll build this for the homework.
* It’s a struct, with an array storing the actual data,
  the number of items in the array, and how big the currently
  allocated array is.
* Since the only way to get a bigger array is to allocate a new
  one and copy over the old items, allocating one at a time would
  be inefficient.
* So we allocate extra space whenever we need to allocate - it turns
  out the doubling the array size (or any constant factor increase)
  each time is efficient.

```
typedef struct vla {
    long size;
    long cap;
    TYPE* data;
} vla;

```

Hash table:

* It’s useful to store mappings of keys to values.
* The most common way is a hash table: A dict in Python, a HashMap in
  Java, an unordered\_map in C++, any object in JS, etc.
* We have variable length array of key-value pairs (a struct), and we
  use a hash function to map keys to slots in the table.

```
typedef struct kv_pair {
    TYPE key;
    TYPE val;
} kv_pair;

typedef struc hash_table {
    long size;
    long cap;
    kv_pair* data;
} hash_table;

```

Singly linked lists:

* Dynamically allocated cells.
* Can add items to the front in O(1).
* Not great for overhead or cache locality.
* Common because they’re simple.

```
#define EMPTY 0

typedef struct cons_cell {
    int head;
    struct cons_cell* tail;
} cons_cell;

cons_cell* 
cons(int head, cons_cell* tail)
{
    cons_cell* cell = malloc(sizeof(cons_cell));
    cell->head = head;
    cell->tail = tail;
}

void
list_free(cons_cell* xs)
{
    if (xs) {
        free_list(xs->cdr);
        free(xs);
    }
}

```

Write the demo that uses shared structure and crashes. See if
the students can figure it out.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 08 Virtual Memory

2025 Feb 06](/classes/2025-01/cs4310/notes/08-vmem/)

[Lecture Notes: 11 No Class

2025 Feb 15
→←](/classes/2025-01/cs4310/notes/11-no-class/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)