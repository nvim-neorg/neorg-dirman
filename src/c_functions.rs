use crate::workspace::Workspace;
use std::{
    ffi::{c_char, CString},
    mem::ManuallyDrop,
};

#[repr(C)]
pub struct FileList {
    pub data: *const *const char,
    pub length: usize,
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
                    .into_raw() as *const char
            })
            .collect::<Vec<*const char>>(),
    );

    let file_list = FileList {
        data: files.as_ptr(),
        length: files.len(),
    };

    Box::into_raw(file_list.into())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_files(files: *mut FileList) {
    if !files.is_null() {
        drop(Box::from_raw(files));
        // drop(files) ?? This is probably required
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_workspace(workspace: *mut Workspace) {
    if !workspace.is_null() {
        drop(Box::from_raw(workspace));
    }
}
