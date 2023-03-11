use crate::workspace::Workspace;
use std::{
    ffi::{c_char, CString},
    mem::ManuallyDrop,
};

#[repr(C)]
pub struct FileList {
    pub data: *const *const c_char,
    pub length: usize,
    pub _capacity: usize,
}

#[no_mangle]
pub unsafe extern "C" fn create_workspace(
    name: *const c_char,
    path: *const c_char,
) -> *mut Workspace {
    if name.is_null() || path.is_null() {
        return std::ptr::null_mut();
    }

    let (name, path) = (
        std::ffi::CStr::from_ptr(name),
        std::ffi::CStr::from_ptr(path),
    );
    let workspace = Workspace {
        name: name.to_string_lossy().to_string(),
        path: path.to_string_lossy().to_string().into(),
    };

    Box::into_raw(workspace.into())
}

#[no_mangle]
// TODO: Indexing item 0 of the returned array here yields a segfault. Any idea why?
pub unsafe extern "C" fn workspace_files(workspace: *const Workspace) -> *mut FileList {
    if workspace.is_null() {
        return std::ptr::null_mut();
    }

    let files = ManuallyDrop::new(
        (*workspace)
            .files()
            .into_iter()
            .map(|path| {
                CString::new(path.to_string_lossy().into_owned())
                    .unwrap()
                    .into_raw() as *const c_char
            })
            .collect::<Vec<*const c_char>>(),
    );

    let file_list = FileList {
        data: files.as_ptr(),
        length: files.len(),
        _capacity: files.capacity(),
    };

    Box::into_raw(file_list.into())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_files(files: *mut FileList) {
    if !files.is_null() {
        drop(Vec::from_raw_parts(files, (*files).length, (*files)._capacity));
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_workspace(workspace: *mut Workspace) {
    if !workspace.is_null() {
        drop(Box::from_raw(workspace));
    }
}
