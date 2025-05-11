use crate::inode;
use crate::disk;

pub const DATA_START:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: i32,
	pub active: bool,
}

pub fn tree_lookup(mmap: &[i8],path: [char; DIR_NAME]) -> (i32, i32) {
	// TODO : Lookup an item with a given path...
}

pub fn rename(ent: Dirent, to: [char; DIR_NAME] ) -> Dirent {
	let mut rnm: Dirent = Dirent { name: [0 as char; DIR_NAME], inum: ent.inum, active: ent.active };
	for i in 0..=DIR_NAME-1 {
		rnm.name[i] = to[i];
	}
	return rnm;
}

pub fn dirent_deserialize(mmap: &[i8]) -> Dirent {
	// TODO : Deserialize a Dirent from raw data...
}

pub fn dirent_serialize(ent: &Dirent) -> Vec<i8> {
	// TODO : Serialize a Dirent into raw data...
}

