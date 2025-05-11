extern crate fuse;
extern crate libc;
extern crate time;

use std::env;
use std::path::Path;
use std::slice;
use libc::{ENOENT, ENOSYS};
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
mod inode;
mod directory;
mod disk;
mod hash;

struct Nufs;

impl Filesystem for Nufs {
	fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
		println!("getattr(ino={})", ino);
		reply.error(ENOSYS);
	}
}

fn main() {
	let mountpoint = match env::args().nth(1) {
		Some(path) => path,
		None => {
			println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
			return;
		}
	};
	
	// Open the file
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("data.nufs")?;
	
	// TODO: Create a memory map for the file
	let mmap: memmap2::MmapMut = unsafe {  };
	
	fuse::mount(Nufs, &mountpoint, &[]);
}
