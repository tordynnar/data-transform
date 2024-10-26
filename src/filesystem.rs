#![allow(dead_code)]

use wasmtime_wasi::{HostInputStream, HostOutputStream, InputStream, OutputStream, Subscribe, TrappableError, StreamResult};
use wasmtime::Result;
use wasmtime::component::Resource;
use anyhow::Error;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use derive_new::new;
use crate::bindings::wasi::filesystem::*;

pub type FsResult<T> = Result<Result<T, types::ErrorCode>, Error>;

pub type FsError = TrappableError<types::ErrorCode>;

pub enum Descriptor {
    InputDirectory,
    OutputDirectory,
    InputFile,
    OutputFile(crate::OutputFile),
}

#[derive(new, Clone, Debug)]
pub struct OutputFile {
    #[new(default)]
    pub data: Arc<Mutex<Vec<u8>>>,
}

pub struct DataInputStream {
    pub offset: usize,
    pub data: Arc<Vec<u8>>,
}

impl HostInputStream for DataInputStream {
    fn read(&mut self, size: usize) -> StreamResult<Bytes>  {
        let end = std::cmp::min(self.offset + size, self.data.len());
        let result = Bytes::copy_from_slice(self.data.get(self.offset..end).unwrap_or_default());
        self.offset = end;
        Ok(result)
    }
}

#[async_trait::async_trait]
impl Subscribe for DataInputStream {
    async fn ready(&mut self) {}
}

impl HostOutputStream for crate::OutputFile {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()>  {
        let mut data = self.data.lock().unwrap();
        data.extend(bytes);
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()>  {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize>  {
        Ok(1024 * 1024)
    }
}

#[async_trait::async_trait]
impl Subscribe for crate::OutputFile {
    async fn ready(&mut self) {}
}

impl types::HostDirectoryEntryStream for crate::Context {
    fn read_directory_entry(&mut self, _self_: Resource<types::DirectoryEntryStream>) -> FsResult<Option<types::DirectoryEntry>> {
        Ok(Ok(None))
    }

    fn drop(&mut self, stream: Resource<types::DirectoryEntryStream>) -> Result<(), Error> {
        self.table.delete(stream)?;
        Ok(())
    }
}

impl types::HostDescriptor for crate::Context {
    fn read_via_stream(&mut self, fd: Resource<types::Descriptor>, offset: types::Filesize) -> FsResult<Resource<InputStream>> {
        match self.table.get(&fd)? {
            Descriptor::InputFile => {
                let stream : InputStream = Box::new(DataInputStream {
                    offset: offset as usize,
                    data: self.input.clone(),
                });
        
                Ok(Ok(self.table.push(stream)?))
            },
            _ => {
                Ok(Err(types::ErrorCode::Access))
            },
        }
    }

    fn write_via_stream(&mut self, fd: Resource<types::Descriptor>, _offset: types::Filesize) -> FsResult<Resource<OutputStream>> {
        // Ignoring offset
        match self.table.get(&fd)? {
            Descriptor::OutputFile(f) => {
                let stream : OutputStream = Box::new(f.clone());
                Ok(Ok(self.table.push(stream)?))
            },
            _ => {
                Ok(Err(types::ErrorCode::Access))
            },
        }
    }

    fn append_via_stream(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<Resource<OutputStream>> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn advise(&mut self, _fd: Resource<types::Descriptor>, _offset: types::Filesize, _length: types::Filesize, _advice: types::Advice) -> FsResult<()> {
        Ok(Ok(()))
    }

    fn sync_data(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<()> {
        Ok(Ok(()))
    }

    fn get_flags(&mut self, fd: Resource<types::Descriptor>) -> FsResult<types::DescriptorFlags> {
        Ok(Ok(match self.table.get(&fd)? {
            Descriptor::InputDirectory => types::DescriptorFlags::empty(),
            Descriptor::OutputDirectory => types::DescriptorFlags::MUTATE_DIRECTORY,
            Descriptor::InputFile => types::DescriptorFlags::READ,
            Descriptor::OutputFile(_) => types::DescriptorFlags::WRITE,
        }))
    }

    fn get_type(&mut self, fd: Resource<types::Descriptor>) -> FsResult<types::DescriptorType> {
        Ok(Ok(match self.table.get(&fd)? {
            Descriptor::InputDirectory | Descriptor::OutputDirectory => types::DescriptorType::Directory,
            Descriptor::InputFile | Descriptor::OutputFile(_) => types::DescriptorType::RegularFile,
        }))
    }

    fn set_size(&mut self, _fd: Resource<types::Descriptor>, _size: types::Filesize) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn set_times(&mut self, _fd: Resource<types::Descriptor>, _data_access_timestamp: types::NewTimestamp, _data_modification_timestamp: types::NewTimestamp) -> FsResult<()> {
        Ok(Ok(()))
    }

    fn read(&mut self, _fd: Resource<types::Descriptor>, _length: types::Filesize, _offset: types::Filesize) -> FsResult<(Vec<u8>,bool)> {
        todo!()
    }

    fn write(&mut self, _fd: Resource<types::Descriptor>, _buffer: Vec<u8>, _offset: types::Filesize) -> FsResult<types::Filesize> {
        todo!()
    }

    fn read_directory(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<Resource<types::DirectoryEntryStream>> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn sync(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<()> {
        Ok(Ok(()))
    }

    fn create_directory_at(&mut self, _fd: Resource<types::Descriptor>, _path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn stat(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<types::DescriptorStat> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn stat_at(&mut self, _fd: Resource<types::Descriptor>, _path_flags: types::PathFlags, _path: String) -> FsResult<types::DescriptorStat> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn set_times_at(&mut self, _fd: Resource<types::Descriptor>, _path_flags: types::PathFlags, _path: String, _data_access_timestamp: types::NewTimestamp, _data_modification_timestamp: types::NewTimestamp) -> FsResult<()> {
        Ok(Ok(()))
    }

    fn link_at(&mut self, _fd: Resource<types::Descriptor>, _old_path_flags: types::PathFlags, _old_path: String, _new_descriptor: Resource<types::Descriptor>, _new_path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn open_at(&mut self, fd: Resource<types::Descriptor>, _path_flags: types::PathFlags, path: String, _open_flags: types::OpenFlags, flags: types::DescriptorFlags) -> FsResult<Resource<types::Descriptor>> {  
        let descriptor = self.table.get(&fd)?;
        match descriptor {
            Descriptor::InputDirectory => {
                if path != "data" {
                    return Ok(Err(types::ErrorCode::NoEntry));
                }
                if flags.contains(types::DescriptorFlags::WRITE) {
                    return Ok(Err(types::ErrorCode::ReadOnly));
                }
                return Ok(Ok(self.table.push(Descriptor::InputFile)?));
            },
            Descriptor::OutputDirectory => {
                if !flags.contains(types::DescriptorFlags::WRITE) {
                    return Ok(Err(types::ErrorCode::Access)); 
                }

                let output_file = crate::OutputFile::new();

                if path.ends_with("json") {
                    self.metadata.push(output_file.clone());
                } else {
                    self.content.push(output_file.clone());
                }
                
                return Ok(Ok(self.table.push(Descriptor::OutputFile(output_file))?));
            },
            _ => {
                return Ok(Err(types::ErrorCode::Access))
            }
        }
    }

    fn readlink_at(&mut self, _fd: Resource<types::Descriptor>, _path: String) -> FsResult<String> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn remove_directory_at(&mut self, _fd: Resource<types::Descriptor>, _path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn rename_at(&mut self, _fd: Resource<types::Descriptor>, _old_path: String, _new_descriptor: Resource<types::Descriptor>, _new_path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn symlink_at(&mut self, _fd: Resource<types::Descriptor>, _old_path: String, _new_path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn unlink_file_at(&mut self, _fd: Resource<types::Descriptor>, _path: String) -> FsResult<()> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn is_same_object(&mut self, _fd: Resource<types::Descriptor>, _other: Resource<types::Descriptor>) -> Result<bool, Error> {
        Ok(false)
    }

    fn metadata_hash(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<types::MetadataHashValue> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn metadata_hash_at(&mut self, _fd: Resource<types::Descriptor>, _path_flags: types::PathFlags, _path: String) -> FsResult<types::MetadataHashValue> {
        Ok(Err(types::ErrorCode::Access))
    }

    fn drop(&mut self, r: Resource<types::Descriptor>) -> Result<(), Error> {
        self.table.delete(r)?;
        Ok(())
    }
}

impl types::Host for crate::Context {
    fn filesystem_error_code(&mut self, _err: Resource<Error>) -> Result<Option<types::ErrorCode>, Error> {
        Ok(None)
    }
}

impl preopens::Host for crate::Context {
    fn get_directories(&mut self) -> Result<Vec<(Resource<types::Descriptor>, String)>, Error>  {
        Ok(vec![
            (self.table.push(Descriptor::InputDirectory)?, "/input".to_owned()),
            (self.table.push(Descriptor::OutputDirectory)?, "/output".to_owned())
        ])
    }
}