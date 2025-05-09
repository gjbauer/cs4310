Lecture Notes: 25 xv6 Syscall · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 25 xv6 Syscall
=============================

2025 Mar 26·Updated: 2025 Mar 30·3 mins

Today [#](#today)
-----------------

* Let’s take a look at xv6

xv6: An Operating System [#](#xv6-an-operating-system)
------------------------------------------------------

* Clone the xv6 repo: <https://github.com/NatTuck/xv6.git>

Building and running it:

* open dedicated terminal
* make
* make qemu-nox
* ls
* cat README
* need to kill qemu to exit

Boot process [#](#boot-process)
-------------------------------

* bootasm.S
* bootmain.c
* main.c
  + main
  + mpmain
* proc.c: scheduler

Syscall Process [#](#syscall-process)
-------------------------------------

* The main() function called tvinit
* Show tvinit in trap.c
* This installs interrupt handlers.
  + Mostly for hardware events.
  + Interrupt #64 is reserved for syscalls.
  + Note that this interrupt is set to allow it to be invoked
    from user code.
* So what’s in the “vectors” global?
  + It’s in vectors.S
  + It’s an array of pointers.
  + Which point to a bunch of different chunks of code.
  + Which are all nearly the same.
  + And jump to alltraps after pushing the interrupt #.
* Then what happens?
  + alltraps is in trapasm.S
  + It makes sure memory is set up for kernel-mode code execution.
  + Then it calls the “trap” function.
  + Then it does an iret to return to user code.
* What does trap() do?
  + Handles interrupts.
  + If this is a syscall, call syscall()
  + Otherwise, it’s probably a hardware interrupt and we determine
    which one and call the appropriate driver handler.
* What does syscall() do?
  + Calls the appropriate syscall implementation, which does whatever
    actual work needs doing.
  + Makes sure that the result ends up in %eax for the caller.

Adding a Syscall [#](#adding-a-syscall)
---------------------------------------

The stock xv6 doesn’t include an exit system call. We need that for
automated tests, so we’re going to add it.

This has already been added to our master branch, first checkout the
raw mit code.

```
$ git checkout mit-default
$ git checkout -b add-halt-syscall

```

### Step 1: Add a syscall number [#](#step-1-add-a-syscall-number)

* open syscall.h
* Syscall numbers 1-21 are taken, so we’ll make our new syscall # 22

```
#define SYS_halt   22

```

### Step 2: Add syscall to the list in syscall.c [#](#step-2-add-syscall-to-the-list-in-syscallc)

```
extern int sys_halt(void);
...
[SYS_halt]    sys_halt,

```

### Step 3: Implement the syscall [#](#step-3-implement-the-syscall)

Add the implementation to sysproc.c:

```
int
sys_halt()
{
  outw(0x604, 0x2000);
  return 0;
}

```

Finding the shutdown command was interesting. Qemu has apparently changed the
shutdown interface several times, and most of the old examples are wrong.

Eventually I looked at Redox OS, since I knew they had this problem and would
care enough to solve it.

### Step 4: Add a userspace wrapper. [#](#step-4-add-a-userspace-wrapper)

Add the declaration to user.h:

```
int halt(void);

```

### Step 5: Add a test program. [#](#step-5-add-a-test-program)

Create halt.c:

```
#include "types.h"
#include "stat.h"
#include "user.h"

int
main(int _ac, char *_av[])
{
  halt();
  return 0;
}

```

Add it to the Makefile

```
UPROGS=\
   ...
   _halt\
   ...

```

References [#](#references)
---------------------------

* <https://pdos.csail.mit.edu/6.828/2012/homework/xv6-syscall.html>
* <https://medium.com/@silvamatteus/adding-new-system-calls-to-xv6-217b7daefbe1>
* <https://gitlab.redox-os.org/redox-os/kernel/blob/master/src/arch/x86_64/stop.rs>

Extra [#](#extra)
-----------------

* Try to figure out existing syscalls.
* Maybe start with “write”.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 24 OS Kernel

2025 Mar 24](/classes/2025-01/cs4310/notes/24-os-kernel/)

[Lecture Notes: 26 Disks

2025 Mar 29
→←](/classes/2025-01/cs4310/notes/26-disks/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)