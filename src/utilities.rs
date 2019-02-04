use filebuffer::FileBuffer;
//use std::env;
use std::fs::File;
use std::io::Read;
//use std::iter::FromIterator;
use std::io;
use std::path::Path;

#[inline(always)]
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn compute_identity(data: &[u8]) -> String {
    use base58::ToBase58;
    use sha2::{Digest, Sha256};

    // create a Sha256 object
    let mut hasher = Sha256::default();

    // write input data
    hasher.input(data);

    // read hash digest and consume hasher
    hasher.result().to_vec().to_base58()
}

/*struct HashWriter<T: Hasher>(T);
impl<T: Hasher> io::Write for HashWriter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf).map(|_| ())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}*/

pub fn compute_file_identity<P: AsRef<Path>>(path: P) -> io::Result<String> {
    use base58::ToBase58;
    use sha2::{Digest, Sha256};

    let fbuffer = FileBuffer::open(&path)?;

    // create a Sha256 object
    let mut hasher = Sha256::default();

    // write input data
    hasher.input(&fbuffer);

    // read hash digest and consume hasher
    Ok(hasher.result().to_vec().to_base58())
}

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    std::fs::metadata(path.as_ref()).is_ok()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let meta = file.metadata()?;
    let size = meta.len() as usize;
    let mut data = vec![0; size];
    file.read_exact(&mut data)?;
    Ok(data)
}

pub fn read_file_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path.as_ref())?;
    let mut text = String::new();
    if let Ok(meta) = file.metadata() {
        text.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
    }
    file.read_to_string(&mut text)?;
    //let text = String::from_iter(normalized(text.chars()));
    Ok(text)
}

pub fn string_from_path(path: &Path) -> Option<String> {
    let path_os_str = path.as_os_str();
    if let Some(path_str) = path_os_str.to_str() {
        Some(path_str.to_string())
    } else {
        None
    }
}
