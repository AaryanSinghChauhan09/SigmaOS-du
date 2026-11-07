//! Formatter model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeFormat { Bytes, Kilobytes, Megabytes, Gigabytes, HumanReadable }

#[derive(Debug, Clone)]
pub struct HumanFormatter {
    pub base: u64,
}

impl Default for HumanFormatter {
    fn default() -> Self {
        Self { base: 1024 }
    }
}

impl HumanFormatter {
    pub fn format(&self, bytes: u64, format: SizeFormat) -> String {
        match format {
            SizeFormat::Bytes => format!("{}B", bytes),
            SizeFormat::Kilobytes => format!("{}K", bytes / self.base),
            SizeFormat::Megabytes => format!("{}M", bytes / (self.base * self.base)),
            SizeFormat::Gigabytes => format!("{}G", bytes / (self.base * self.base * self.base)),
            SizeFormat::HumanReadable => {
                if bytes < self.base {
                    format!("{}B", bytes)
                } else if bytes < self.base * self.base {
                    format!("{}K", bytes / self.base)
                } else if bytes < self.base * self.base * self.base {
                    format!("{}M", bytes / (self.base * self.base))
                } else {
                    format!("{}G", bytes / (self.base * self.base * self.base))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_formatter() {
        let f = HumanFormatter::default();
        assert_eq!(f.format(2048, SizeFormat::HumanReadable), "2K");
    }
}
