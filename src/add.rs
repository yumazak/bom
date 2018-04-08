use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, target: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ta = &target.join(&path.file_name().unwrap());
            if path.is_dir() {
                fs::create_dir(ta);
                visit_dirs(&path, ta)?;
            } else {
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}