# Required Software for cs4310
This course is “Operating Systems”.

We will be exploring programming interfaces provided by the operating system. Different operating system families provide different interfaces, so we need to pick one.

We’ll be using Linux. Most version of Linux will provide the same POSIX system call interface for C programming. Different Linux distributions have slightly different commands; the following versions will be supported for this class:

 * Ubuntu 22.04, as installed on the lab workstations
 * Debian 12, as installed on the Inkfish testing containers
 * Linux Mint 22.x, recommended for your personal machine
There are slight differences between these versions and we will run into them. That’s expected.

You can run a different version of Linux or even try to run the assignments on MacOS or Windows with a POSIX compatability layer. In those cases, if it breaks only a limited amount of help will be available.

#### Installing Linux in a VM#
This is the recommended procedure if you’re not running Linux directly on your hardware. Aside from the first homework, this is not required.

###### Install a Hypervisor

 * On Windows: VirtualBox
 * On ARM Mac: UTM Blog Post
This course includes some AMD64 assembly code, so you will need to install the AMD64 version of your Linux OS even if that means emulating it on a modern Mac.

###### Install a Linux Mint VM

 * Download the Linux Mint 22.1 AMD64 Cinnamon Edition ISO
 * Create a new virtual machine in your hypervisor, and install Linux Mint using the ISO you downloaded.
 * Make sure your new VM has at least 4 cores, 4 GB of RAM, and 20 GB of disk space. Not pre-allocating the disk is fine.

###### Install Some Packages

Once you have Linux mint up and running, open a terminal and run the following commands

```
sudo apt update -y && sudo apt upgrade -y
sudo apt install -y build-essential git neovim time util-linux \
    manpages manpages-dev manpages-posix manpages-posix-dev \
    libipc-run-perl libarchive-zip-perl libbsd-dev pkg-config
```

