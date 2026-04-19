use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn decode_zip_name(raw: &[u8]) -> String {
    if std::str::from_utf8(raw).is_ok() {
        return String::from_utf8_lossy(raw).into_owned();
    }
    let (cow, _encoding, had_errors) = encoding_rs::SHIFT_JIS.decode(raw);
    if !had_errors {
        return cow.into_owned();
    }
    String::from_utf8_lossy(raw).into_owned()
}

pub fn extract_dirs_as_zips(src_zip_path: &str, output_dir: &str) -> zip::result::ZipResult<()> {
    let output_path = Path::new(output_dir);
    std::fs::create_dir_all(output_path)?;

    let src_stem = Path::new(src_zip_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let file = File::open(src_zip_path)?;
    let mut src_zip = zip::ZipArchive::new(file)?;

    let mut dir_members: HashMap<String, Vec<(String, Vec<u8>)>> = HashMap::new();

    for i in 0..src_zip.len() {
        let mut entry = src_zip.by_index(i)?;
        let name = decode_zip_name(entry.name_raw());
        if entry.is_dir() {
            continue;
        }

        let full_path = PathBuf::from(&name);
        let direct_parent = match full_path.parent() {
            Some(p) if p != Path::new("") => p.to_path_buf(),
            _ => PathBuf::new(),
        };

        let rel_path = full_path.file_name().unwrap().to_string_lossy().to_string();
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;

        dir_members
            .entry(direct_parent.to_string_lossy().to_string())
            .or_default()
            .push((rel_path, buf));
    }

    let src_zip_canonical = std::fs::canonicalize(src_zip_path).ok();

    for (dir_path, members) in &dir_members {
        let zip_name = if dir_path.is_empty() {
            src_stem.clone()
        } else {
            dir_path.replace('/', "_")
        };
        let candidate = output_path.join(format!("{}.zip", zip_name));
        let out_zip_path =
            if src_zip_canonical.as_deref() == std::fs::canonicalize(&candidate).ok().as_deref() {
                output_path.join(format!("{}_extracted.zip", zip_name))
            } else {
                candidate
            };
        let out_file = File::create(&out_zip_path)?;
        let mut out_zip = zip::ZipWriter::new(out_file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for (rel_path, buf) in members {
            out_zip.start_file(rel_path, options)?;
            out_zip.write_all(buf)?;
        }

        out_zip.finish()?;
        println!("Created: {}", out_zip_path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::SimpleFileOptions;

    fn create_test_zip(dir: &Path) -> PathBuf {
        let zip_path = dir.join("test.zip");
        let file = File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        zip.start_file("root.txt", options).unwrap();
        zip.write_all(b"root").unwrap();
        zip.start_file("dir_a/foo.txt", options).unwrap();
        zip.write_all(b"hello").unwrap();
        zip.start_file("dir_a/sub/bar.txt", options).unwrap();
        zip.write_all(b"world").unwrap();
        zip.start_file("dir_b/baz.txt", options).unwrap();
        zip.write_all(b"test").unwrap();

        zip.finish().unwrap();
        zip_path
    }

    #[test]
    fn test_extract_creates_zips() {
        let tmp = tempdir().unwrap();
        let zip_path = create_test_zip(tmp.path());
        let out_dir = tmp.path().join("output");

        extract_dirs_as_zips(zip_path.to_str().unwrap(), out_dir.to_str().unwrap()).unwrap();

        assert!(out_dir.join("test.zip").exists());
        assert!(out_dir.join("dir_a.zip").exists());
        assert!(out_dir.join("dir_a_sub.zip").exists());
        assert!(out_dir.join("dir_b.zip").exists());
    }

    #[test]
    fn test_dir_a_contains_only_direct_files() {
        let tmp = tempdir().unwrap();
        let zip_path = create_test_zip(tmp.path());
        let out_dir = tmp.path().join("output");

        extract_dirs_as_zips(zip_path.to_str().unwrap(), out_dir.to_str().unwrap()).unwrap();

        let dir_a_zip = File::open(out_dir.join("dir_a.zip")).unwrap();
        let mut archive = zip::ZipArchive::new(dir_a_zip).unwrap();
        let names: Vec<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().to_string())
            .collect();

        assert!(names.contains(&"foo.txt".to_string()));
        assert!(!names.contains(&"sub/bar.txt".to_string()));
    }
}
