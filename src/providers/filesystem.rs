use std::{
    self,
    borrow::Borrow,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    task::Poll,
};

use bytes::Bytes;
use futures::{Stream, StreamExt};

use super::Kind;

#[derive(Clone)]
pub struct FilesystemObject {
    pub name: String,
    pub dir: Option<PathBuf>,
    pub kind: Kind,
}

pub struct FileBytesStream {
    reader: BufReader<File>,
    size: usize,
}

impl FileBytesStream {
    pub fn new(file: File) -> FileBytesStream {
        let file_len = file.metadata().unwrap().len() as usize;
        FileBytesStream {
            reader: BufReader::new(file),
            size: file_len,
        }
    }
}

impl Stream for FileBytesStream {
    type Item = Result<Bytes, io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.reader.fill_buf() {
            Ok(bytes_read) => {
                let consumed = bytes_read.len();
                if consumed > 0 {
                    let bytes_read = Bytes::copy_from_slice(bytes_read);
                    self.reader.consume(consumed);
                    Poll::Ready(Some(Ok(bytes_read)))
                } else {
                    Poll::Ready(None)
                }
            }
            Err(err) => Poll::Ready(Some(Err(err))),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

pub fn get_files_list(path: &Path) -> Result<Vec<FilesystemObject>, io::Error> {
    if !path.exists()  {
        return Err(io::Error::new(io::ErrorKind::NotFound, 
            "File with given path was not found"));
    }
    if let Ok(dir_metadata) = fs::metadata(path) {
        if !dir_metadata.is_dir() {
            if let Ok(dir_entries) = fs::read_dir(path) {
                return Ok(dir_entries.map(|f| {
                    let path = f.unwrap().path();
                    let mut file_name = String::from(path.file_name()
                        .unwrap()
                        .to_str()
                        .expect("Cannot convert non-utf8 filename to string"));
                    let kind: Kind;
                    if fs::metadata(&path).unwrap().is_dir() {
                        file_name.push_str("/");
                        kind = Kind::Directory
                    } else {
                        kind = Kind::File;
                    }
                    FilesystemObject {
                        name: file_name,
                        dir: path.parent().and_then(|p| Some(p.to_path_buf())),
                        kind: kind,
                    }
                }).collect());
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, 
                    "Couldn't read from the file, possible permissions issue"));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::Unsupported, 
                "Given path points to a non-directory file"));
        }
    } else {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, 
                "Couldn't access directory metadata, possible permissions issue"));
    }
}

pub fn get_file_byte_stream(path: &Path) -> Result<FileBytesStream, io::Error> {
    let file = fs::OpenOptions::new()
        .create(false)
        .truncate(false)
        .open(path)?;
    Ok(FileBytesStream::new(file))
}

pub async fn write_file_from_stream<S>(path: &Path, stream: S) -> Result<(), io::Error>
where
    S: Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
{
    if let Ok(new_file) = File::create(path) {
        let mut writer = BufWriter::new(new_file);
        let mut stream = Box::pin(stream);
        while let Some(chunk) = stream.next().await {
            if let Err(err) = writer.write(chunk.unwrap().borrow()) {
                match err.kind() {
                    io::ErrorKind::Interrupted => continue,
                    _ => return Err(err)
                }
            };
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, 
            "Couldn't create file for writing, possible permissions issue"));
    }
    Ok(())
}

pub fn remove_file(path: &Path) -> Result<(), io::Error> {
    if !path.exists()  {
        return Err(io::Error::new(io::ErrorKind::NotFound, 
            "File with given path was not found"));
    }
    if let Ok(file_metadata) = fs::metadata(path) {
        if file_metadata.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Unsupported,
                "Deleteion of directories is unsupported!"));
        }
        if let Err(_) = fs::remove_file(path) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Coudn't delete file, possible permissions issue"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied,
            "Couldn't access file metadata, possible permissions issue"));
    }
    Ok(())
}
