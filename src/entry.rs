//! Du entry model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType { File, Directory, Symlink, BlockDevice, CharDevice }

#[derive(Debug, Clone)]
pub struct DuEntry {
    pub path: String,
    pub entry_type: EntryType,
    pub size_bytes: u64,
    pub blocks: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn du_entry() {
        let e = DuEntry { path: "/home/user/file.txt".into(), entry_type: EntryType::File, size_bytes: 1024, blocks: 2 };
        assert_eq!(e.size_bytes, 1024);
    }
}
