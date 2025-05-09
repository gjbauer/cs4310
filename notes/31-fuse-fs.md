Lecture Notes: 31 Fuse Filesystem · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 31 Fuse Filesystem
=================================

2025 Apr 09·Updated: 2025 Apr 10·2 mins

FUSE [#](#fuse)
---------------

Filesystem in UserSpacE

This is a mechanism that allows filesystem drivers to be written as userspace
programs that interface with a generic kernel module. This was initially a Linux
specific thing, but it looks like there’s also some support now on Mac and
FreeBSD.

For HW09, you will build a FUSE filesystem that implements some of the features
of a Linux filesystem, specifically:

* Create files
* List files in the FS root directory
* Write to small files
* Read from small files
* Delete files
* Rename files

When a program tries to do any of these things in your mounted filesystem - by
making the appropriate system calls - the kernel will call one or more callback
functions in your driver program. Those callback functions are where you
implement your filesystem.

Let’s give this a try.

* Open two terminal windows to the unpacked starter code.
* In terminal A, we run “make” and “make mount”. This starts the FS
  driver in the foreground so it can print out debugging info.
* In terminal B, we can run commands that will interact with
  the mounted filesystem.
* Note: Try “make unmount”.
* The fs is mounted at the “mnt” directory.
* Let’s see what’s in there with “ls mnt”.
* There’s a file. Let’s read it with “cat mnt/hello.txt”.
* Let’s add our own file with “echo boat > mnt/boat.txt”.

Now let’s look at the starter code. We start in “nufs.c”.

* main is at the bottom
* It initializes a fuse\_operations struct with a bunch of
  pointers to callback functions.
* What functions do we get?
* Go through all of them, noting that some are not needed for hw09.
* Hint 1: Whenever anything doesn’t work, it’s because getattr
  is wrong. It needs to return the correct error codes.
* To return an error code, return its negation from the callback.

The Hints Directory [#](#the-hints-directory)
---------------------------------------------

* This includes some of my solution.
* You can look at or ignore it.
* Some of it is overkill for this assignment (show inode.h)
* I recommend using the following stuff from hints/pages.c:
  + pages\_init
  + pages\_get\_page
* That’ll let you pretend you have a block device (pages\_get\_page
  gives you a pointer to one 4k disk “block”).

Let’s mess with this a bit [#](#lets-mess-with-this-a-bit)
----------------------------------------------------------

* Add another hard-coded file.
* Make it so that you can create files, but they always contain
  their name and throw away any data written to them.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 29 ext2

2025 Apr 05](/classes/2025-01/cs4310/notes/29-ext3/)

[Lecture Notes: 32 Virtual Machines

2025 Apr 12
→←](/classes/2025-01/cs4310/notes/32-vmms/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)