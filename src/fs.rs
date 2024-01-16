//! The `fs` module contains functionality for interfacing custom resource loading.
//!
//! Implement the FileSystem trait for your custom resource loading, with its open() method returning
//! objects satisfying the FileOperations trait.
use russimp_sys::{aiFile, aiFileIO, aiOrigin, aiReturn};
use std::convert::TryInto;
use std::ffi::CStr;
use std::io::SeekFrom;

/// Implement FileSystem to use custom resource loading using `Scene::from_filesystem()`.
///
/// Rusty version of the underlying aiFileIO type.
pub trait FileSystem {
    fn open(&self, file_path: &str, mode: &str) -> Option<Box<dyn FileOperations>>;
}

/// Implement this for a given resource to support custom resource loading.
///
/// This trait class is the rusty version of the underlying aiFile type.
pub trait FileOperations {
    /// Should return the number of bytes read, or Err if read unsuccessful.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    /// Should return the number of bytes written, or Err if write unsuccessful.
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
    fn tell(&mut self) -> usize;
    fn size(&mut self) -> usize;
    fn seek(&mut self, seek_from: SeekFrom) -> Result<(), ()>;
    fn flush(&mut self);
    fn close(&mut self);
}

/// This type allows us to generate C stubs for whatever trait object the user supplies.
pub(crate) struct FileOperationsWrapper<T: FileSystem> {
    ai_file: aiFileIO,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FileSystem> FileOperationsWrapper<T> {
    /// Returns a wrapper that can create an aiFileIO to be used with the assimp C-API.
    pub fn new(file_system: &T) -> FileOperationsWrapper<T> {
        let trait_obj: &dyn FileSystem = file_system;
        let managed_box = Box::new(trait_obj);
        let user_data = Box::into_raw(managed_box);
        let user_data = user_data as *mut i8;
        FileOperationsWrapper {
            ai_file: aiFileIO {
                OpenProc: Some(FileOperationsWrapper::<T>::io_open),
                CloseProc: Some(FileOperationsWrapper::<T>::io_close),
                UserData: user_data,
            },
            _phantom: Default::default(),
        }
    }
    /// Get the aiFileIO to pass to the C-interface.
    pub fn ai_file(&mut self) -> &mut aiFileIO {
        &mut self.ai_file
    }
    /// Implementation for aiFileIO::OpenProc.
    unsafe extern "C" fn io_open(
        ai_file_io: *mut aiFileIO,
        file_path: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> *mut aiFile {
        let file_system = Box::leak(Box::from_raw(
            (*ai_file_io).UserData as *mut &dyn FileSystem,
        ));

        let file_path = CStr::from_ptr(file_path)
            .to_str()
            .unwrap_or("Invalid UTF-8 Filename");
        let mode = CStr::from_ptr(mode)
            .to_str()
            .unwrap_or("Invalid UTF-8 Mode");
        let file = match file_system.open(file_path, mode) {
            None => return std::ptr::null_mut(),
            Some(file) => file,
        };

        // Take the returned file, and double box it here so that it can be converted to a single
        // raw pointer that can be stuffed in the UserData.
        let double_box = Box::new(file);
        let managed_box = Box::into_raw(double_box); // Cleaned up in io_close.
        let user_data = managed_box as *mut i8;
        let ai_file = aiFile {
            ReadProc: Some(Self::io_read),
            WriteProc: Some(Self::io_write),
            TellProc: Some(Self::io_tell),
            FileSizeProc: Some(Self::io_size),
            SeekProc: Some(Self::io_seek),
            FlushProc: Some(Self::io_flush),
            UserData: user_data,
        };
        // Lifetime of ai_file is managed by backend assimp library, cleaned up in io_close().
        Box::into_raw(Box::new(ai_file))
    }

    /// Implementation for aiFileIO::CloseProc.
    unsafe extern "C" fn io_close(_ai_file_io: *mut aiFileIO, ai_file: *mut aiFile) {
        // Given that this is close, we are careful to not leak, but instead drop the file when we
        // exit this scope.
        let ai_file = Box::from_raw(ai_file);
        let mut file: Box<Box<dyn FileOperations>> =
            Box::from_raw((*ai_file).UserData as *mut Box<dyn FileOperations>);
        file.close();
    }
    /// Turn an aiFile pointer into a the "self" object.
    ///
    /// Safety: Only safe to call once from within each of the io_* callbacks. This assumes that
    /// the loading library has ownership of the aiFile object that was returned by the FileSystem.
    /// It is expected to only be called serially on a single thread for the lifetype 'a, which
    /// *should* keep access scoped to within the io_* callback.
    unsafe fn get_file<'a>(ai_file: *mut aiFile) -> &'a mut Box<dyn FileOperations> {
        // We return a "leaked" pointer here, using the saved off double box pointer that we
        // stuffed in the UserData. We "leak" as we don't acutally want to return ownership, only
        // the mutable reference. The box is manually cleaned up as part of io_close.
        Box::leak(Box::from_raw(
            (*ai_file).UserData as *mut Box<dyn FileOperations>,
        ))
    }
    // Implementation for aiFile::ReadProc.
    unsafe extern "C" fn io_read(
        ai_file: *mut aiFile,
        buffer: *mut std::os::raw::c_char,
        size: usize,
        count: usize,
    ) -> usize {
        let file = Self::get_file(ai_file);
        let mut buffer =
            std::slice::from_raw_parts_mut(buffer as *mut u8, (size * count).try_into().unwrap());
        if size == 0 {
            panic!("Size 0 is invalid");
        }
        if count == 0 {
            panic!("Count 0 is invalid");
        }
        if size > std::usize::MAX {
            panic!("huge read size not supported");
        }
        let size = size as usize;
        if size == 1 {
            // This looks like a memcpy.
            if count > std::usize::MAX {
                panic!("huge read not supported");
            }
            let count = count as usize;

            let (buffer, _) = buffer.split_at_mut(count);
            match file.read(buffer) {
                Ok(size) => size,
                Err(_) => std::usize::MAX,
            }
        } else {
            // We have to copy in strides. Implement this by looping for each object and tally the
            // count of full objects read.
            let mut total: usize = 0;
            for _ in 0..count {
                let split = buffer.split_at_mut(size as usize);
                buffer = split.1;
                let bytes_read = match file.read(split.0) {
                    Err(_) => break,
                    Ok(bytes_read) => bytes_read,
                };
                if bytes_read != size {
                    break;
                }
                total = total + 1;
            }
            total
        }
    }
    // Implementation for aiFile::WriteProc.
    unsafe extern "C" fn io_write(
        ai_file: *mut aiFile,
        buffer: *const std::os::raw::c_char,
        size: usize,
        count: usize,
    ) -> usize {
        let file = Self::get_file(ai_file);
        let mut buffer =
            std::slice::from_raw_parts(buffer as *mut u8, (size * count).try_into().unwrap());
        if size == 0 {
            panic!("Write of size 0");
        }
        if count == 0 {
            panic!("Write of count 0");
        }
        if size > std::usize::MAX {
            panic!("huge write size not supported");
        }
        let size = size as usize;
        if size == 1 {
            if count > std::usize::MAX {
                panic!("huge write not supported");
            }
            let count = count as usize;

            let (buffer, _) = buffer.split_at(count);
            match file.write(buffer) {
                Ok(size) => size,
                Err(_) => std::usize::MAX,
            }
        } else {
            // Write in strides. Implement this by looping for each object and tally the
            // count of full objects written.
            let mut total: usize = 0;
            for _ in 0..count {
                let split = buffer.split_at(size as usize);
                buffer = split.1;
                let bytes_written = match file.write(split.0) {
                    Err(_) => break,
                    Ok(bytes_written) => bytes_written,
                };
                if bytes_written != size {
                    break;
                }
                total = total + 1;
            }
            total
        }
    }
    // Implementation for aiFile::TellProc.
    unsafe extern "C" fn io_tell(ai_file: *mut aiFile) -> usize {
        let file = Self::get_file(ai_file);
        file.tell()
    }
    // Implementation for aiFile::FileSizeProc.
    unsafe extern "C" fn io_size(ai_file: *mut aiFile) -> usize {
        let file = Self::get_file(ai_file);
        file.size()
    }
    // Implementation for aiFile::SeekProc.
    unsafe extern "C" fn io_seek(ai_file: *mut aiFile, pos: usize, origin: aiOrigin) -> aiReturn {
        let file = Self::get_file(ai_file);
        let seek_from = match origin {
            russimp_sys::aiOrigin_aiOrigin_SET => SeekFrom::Start(pos as u64),
            russimp_sys::aiOrigin_aiOrigin_CUR => SeekFrom::Current(pos as i64),
            russimp_sys::aiOrigin_aiOrigin_END => SeekFrom::End(pos as i64),
            _ => panic!("Assimp passed invalid origin"),
        };
        match file.seek(seek_from) {
            Ok(()) => 0,
            Err(()) => russimp_sys::aiReturn_aiReturn_FAILURE,
        }
    }
    // Implementation for aiFile::FlushProc.
    unsafe extern "C" fn io_flush(ai_file: *mut aiFile) {
        let file = Self::get_file(ai_file);
        file.flush();
    }
}

impl<T: FileSystem> Drop for FileOperationsWrapper<T> {
    fn drop(&mut self) {
        // Re-construct and drop the box that was used for the C-API.
        let _managed_box: Box<&dyn FileSystem> =
            unsafe { Box::from_raw(self.ai_file.UserData as *mut &dyn FileSystem) };
    }
}

#[cfg(test)]
mod test {
    use crate::scene::PostProcess;
    use crate::scene::Scene;
    use crate::utils;
    use std::fs::File;
    use std::io::{prelude::*, SeekFrom};

    struct MyFileOperations {
        file: File,
    }

    impl super::FileOperations for MyFileOperations {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
            self.file.read(buf).or_else(|_| Err(()))
        }

        fn write(&mut self, _buf: &[u8]) -> Result<usize, ()> {
            unimplemented!("write support");
        }

        fn tell(&mut self) -> usize {
            self.file
                .seek(SeekFrom::Current(0))
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0)
        }

        fn size(&mut self) -> usize {
            self.file
                .metadata()
                .expect("Missing metadata")
                .len()
                .try_into()
                .unwrap_or(0)
        }

        fn seek(&mut self, seek_from: SeekFrom) -> Result<(), ()> {
            match self.file.seek(seek_from) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }

        fn flush(&mut self) {
            // write suppot not implemented.
        }

        fn close(&mut self) {
            // Nothing to do.
        }
    }

    struct MyFS {}

    impl super::FileSystem for MyFS {
        fn open(&self, file_path: &str, mode: &str) -> Option<Box<dyn super::FileOperations>> {
            // We only support reading for this test.
            assert_eq!(mode, "rb");
            let file = File::open(file_path).expect("Couldn't open {file_path}");
            Some(Box::new(MyFileOperations { file }))
        }
    }

    #[test]
    fn test_file_operations() {
        // Load the cube.obj as it also has to load the cube.mtl through the filesystem to get the
        // materials right.
        let model_path = utils::get_model("models/OBJ/cube.obj");
        let mut myfs = MyFS {};
        let scene = Scene::from_file_system(
            model_path.as_str(),
            vec![
                PostProcess::CalculateTangentSpace,
                PostProcess::Triangulate,
                PostProcess::JoinIdenticalVertices,
                PostProcess::SortByPrimitiveType,
            ],
            &mut myfs,
        )
        .unwrap();

        assert_eq!(scene.meshes[0].texture_coords.len(), 8);
        assert_eq!(scene.materials.len(), 2);
    }
}
