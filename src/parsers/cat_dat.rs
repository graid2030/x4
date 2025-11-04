use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct XmlEntry {
    pub name: String,
    pub content: String,
}

/// Reads XML files from X4's CAT/DAT file pairs
pub struct CatDatReader {
    cat_path: String,
    dat_path: String,
}

impl CatDatReader {
    pub fn new<P: AsRef<Path>>(cat_path: P) -> Result<Self> {
        let cat_path = cat_path.as_ref().to_string_lossy().to_string();
        let dat_path = cat_path.replace(".cat", ".dat");

        Ok(Self { cat_path, dat_path })
    }

    /// Extract all XML files from the CAT/DAT pair
    pub fn extract_xml_files(&self) -> Result<Vec<XmlEntry>> {
        let cat_file = File::open(&self.cat_path)
            .with_context(|| format!("Failed to open CAT file: {}", self.cat_path))?;
        let mut dat_file = File::open(&self.dat_path)
            .with_context(|| format!("Failed to open DAT file: {}", self.dat_path))?;

        let reader = BufReader::new(cat_file);
        let mut entries = Vec::new();
        let mut offset: u64 = 0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.rsplitn(4, ' ').collect();

            if parts.len() < 4 {
                continue;
            }

            let filename = parts[3];
            let size: u64 = parts[2].parse()
                .with_context(|| format!("Failed to parse size for {}", filename))?;

            if filename.ends_with(".xml") {
                dat_file.seek(SeekFrom::Start(offset))?;

                let mut buffer = vec![0u8; size as usize];
                dat_file.read_exact(&mut buffer)?;

                let content = String::from_utf8_lossy(&buffer).to_string();

                entries.push(XmlEntry {
                    name: filename.to_string(),
                    content,
                });
            }

            offset += size;
        }

        Ok(entries)
    }
}

/// Find all CAT files in the X4 installation directory
pub fn find_cat_files<P: AsRef<Path>>(x4_folder: P, patterns: &[&str]) -> Result<Vec<String>> {
    let base_path = x4_folder.as_ref();
    let mut cat_files = Vec::new();

    // Base game CAT files
    for pattern in patterns {
        let cat_path = base_path.join(pattern);
        if cat_path.exists() {
            cat_files.push(cat_path.to_string_lossy().to_string());
        }
    }

    // Extension CAT files (scan recursively for ext_NN.cat and NN.cat under each extension)
    let extensions_path = base_path.join("extensions");
    if extensions_path.exists() {
        let mut stack = vec![extensions_path];
        while let Some(dir) = stack.pop() {
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for entry in rd.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.extension().and_then(|s| s.to_str()) == Some("cat") {
                        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                            // Match patterns like "ext_01.cat", "ext_02.cat", "ext_03.cat", or "01.cat", "02.cat", etc.
                            // But exclude "ext_NN_sig.cat" signature files
                            let is_ext_cat = name.starts_with("ext_")
                                && name.ends_with(".cat")
                                && !name.ends_with("_sig.cat");
                            let is_numbered_cat = name.len() == 6
                                && &name[2..] == ".cat"
                                && name.chars().take(2).all(|c| c.is_ascii_digit());

                            if is_ext_cat || is_numbered_cat {
                                cat_files.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(cat_files)
}
