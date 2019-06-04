use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use zip::write::FileOptions;
use zip::result::ZipError;

use walkdir::{WalkDir, DirEntry};
use std::path::Path;
use std::fs::File;


fn zip_dir<T>(it: &mut Iterator<Item=DirEntry>, prefix: &str, writer: T)
              -> zip::result::ZipResult<()>
    where T: Write+Seek
{

    let buffer = std::io::BufWriter::with_capacity(65536, writer);

    let mut zip = zip::ZipWriter::new(buffer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2);

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip).unwrap();
        } 
        else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn doit(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file)?;

    Ok(())
}