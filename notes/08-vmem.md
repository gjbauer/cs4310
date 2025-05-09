Lecture Notes: 08 Virtual Memory · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 08 Virtual Memory
================================

2025 Feb 06·Updated: 2025 Feb 15·1 min

ref: <https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/>

Sample Program [#](#sample-program)
-----------------------------------

* Find the top 10 words with the most A’s from
  /usr/share/dict/words
* Keep an array of the top 10 so far, replace as needed.
* Do it with malloc, show sys\_brk, then move to sbrk
  and start worrying about free.

Malloc and Brk [#](#malloc-and-brk)
-----------------------------------

* malloc
* sbrk

The brk syscall:

* Returns current brk on 0 argument.
* Lets you set a higher brk.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 09 Brk

2025 Feb 06](/classes/2025-01/cs4310/notes/09-brk/)

[Lecture Notes: 10 Data Structures

2025 Feb 12
→←](/classes/2025-01/cs4310/notes/10-data-structures/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)