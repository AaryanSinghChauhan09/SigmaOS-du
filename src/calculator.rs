//! Size calculator model.

use super::DuEntry;

#[derive(Debug, Clone)]
pub struct SizeResult {
    pub path: String,
    pub size_bytes: u64,
    pub block_size: u32,
}

#[derive(Debug, Clone)]
pub struct DuCalculator {
    pub block_size: u32,
}

impl DuCalculator {
    pub fn new(block_size: u32) -> Self {
        Self { block_size }
    }

    pub fn calculate_size(&self, entry: &DuEntry) -> SizeResult {
        SizeResult {
            path: entry.path.clone(),
            size_bytes: entry.size_bytes,
            block_size: self.block_size,
        }
    }

    pub fn calculate_recursive(&self, _path: &str) -> SizeResult {
        SizeResult {
            path: "/".into(),
            size_bytes: 0,
            block_size: self.block_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn du_calculator() {
        let c = DuCalculator::new(512);
        let e = DuEntry { path: "/file".into(), entry_type: crate::EntryType::File, size_bytes: 1024, blocks: 2 };
        let r = c.calculate_size(&e);
        assert_eq!(r.size_bytes, 1024);
    }
}
