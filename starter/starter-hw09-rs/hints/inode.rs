use crate::disk;
use crate::directory;
use crate:: hash;

const INS: usize = 2 * 4096;	// get_inode_start();

pub struct Inode {
	pub refs: u32,
	pub mode: u32,
	pub size: [u16; 2],
	pub ptrs: [u16; 2],
	pub iptr: i32,
	pub inum: i32,
}

pub fn alloc_inode(path: [char; directory::DIR_NAME]) -> i32 {
	// TODO : Allocate an available inode...
}

pub fn inode_find(path: [char; directory::DIR_NAME], mmap: &[i8]) -> i32 {
	// TODO : Find and return an available inode...
}

pub fn inode_deserialize(mmap: &[i8], num: i32) -> Inode {
	// TODO : Deserialize an Inode from raw data...
}

pub fn inode_serialize(mmap: &mut [u8], d: Inode) -> i32 {
	// TODO : Serialize an Inode into raw data...
}

pub fn inode_read(d: Inode, mmap: &[u8]) -> (Vec<u8>, i32) {
	let mut c: Vec<u8> = vec![];
	for i in 0..=d.size[0]-1 {
		c.push(mmap[INS+(d.ptrs[0] as usize)..INS+(d.ptrs[0] as usize)+1][0]);
	}
	for i in 0..=d.size[1]-1 {
		c.push(mmap[INS+(d.ptrs[1] as usize)..INS+(d.ptrs[1] as usize)+1][0]);
	}
	return (c, d.iptr);
}



