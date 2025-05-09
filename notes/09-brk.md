Lecture Notes: 09 Brk · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 09 Brk
=====================

2025 Feb 06·Updated: 2025 Feb 10·1 min

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

Some extra time for stuff [#](#some-extra-time-for-stuff)
---------------------------------------------------------

* We implemented free with fixed 100 byte blocks.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 07 Syscalls

2025 Feb 03](/classes/2025-01/cs4310/notes/07-syscall/)

[Lecture Notes: 08 Virtual Memory

2025 Feb 06
→←](/classes/2025-01/cs4310/notes/08-vmem/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)