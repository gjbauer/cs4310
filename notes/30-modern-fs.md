Lecture Notes: 30 Modern FS · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 30 Modern FS
===========================

2025 Apr 05·Updated: 2025 Apr 08·2 mins

ZFS Features [#](#zfs-features)
-------------------------------

Copy on Write

* Writes don’t overwrite blocks.
* Instead, the new version of the block is written to a new location.
* Metadata lives in a tree, like in a log-structured FS.
* Every change creates new parent nodes, up to a new root.
* (Draw a persistent tree)
* This doesn’t change the old tree, so corruption can always be
  dealt with by rolling back to the old root.
* To detect corruption, every node gets a checksum. This also helps
  detect corruption due to hardware issues.
* This also allows mostly sequential writes.
* Bonus: This gives easy snapshot functionality by pinning old roots.
* Reflink copies

Built-in Multi-Disk Support

* A ZFS filesystem exists on a pool of disks.
* This replaces traditional RAID modes.
* More advantages: e.g. node checksums can be used to detect bad disks.
* Modes: raid-z (1 parity), raid-z2, raid-z3

ZFS History / Licensing [#](#zfs-history--licensing)
----------------------------------------------------

* Solaris, OpenSolaris
* License issue for Linux
* FreeBSD

BTRFS [#](#btrfs)
-----------------

* Basically a rewrite of ZFS for Linux license compatibility.
* Provides good support for duplicate modes but not for parity modes.

Advantages over ZFS:

* Works on stock Linux
* Definitely legal on Linux
* Allows pools to be shrunk
* Broader recognition in some other tools that use reflinks, snapshots,
  subvolumes on Linux
* Better performance on lower RAM systems.

Advantages of ZFS over BTRFS:

* More mature.
* Better performance with lots of RAM.
* Has stable raid-z parity modes.
* Great on FreeBSD
* Doesn’t get completely borked occasionally because it can’t
  manage mostly-full disks.

Recommendations:

* If you’re building a big file server, consider ZFS on FreeBSD.
* If you don’t want to worry about filesystems and have backups,
  ext4 on Linux is fine.
* If you care about your data on Linux, have backups and run
  a 2+ disk BTRFS pool with “raid1”.

Homework [#](#homework)
-----------------------

Introduce the HW.

Key idea: Two terminals, run the FS in one and try stuff in the other.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 28 ext2

2025 Apr 03](/classes/2025-01/cs4310/notes/28-ext2/)

[Lecture Notes: 29 ext2

2025 Apr 05
→←](/classes/2025-01/cs4310/notes/29-ext3/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)