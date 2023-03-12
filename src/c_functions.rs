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
    assert!(!name.is_null(), "Parameter `name` must not be `null`!");
    assert!(!path.is_null(), "Parameter `path` must not be `null`!");

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
pub unsafe extern "C" fn workspace_files(workspace: *const Workspace) -> *mut FileList {
    assert!(
        !workspace.is_null(),
        "Parameter `workspace` must not be `null`!"
    );

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
pub unsafe extern "C" fn destroy_files(file_list: *mut FileList) {
    if file_list.is_null() {
        return;
    }

    let file_list = Box::from_raw(file_list);

    let file_path_ptrs: Vec<*mut c_char> = Vec::from_raw_parts(
        file_list.data as *mut *mut c_char,
        file_list.length,
        file_list._capacity,
    );

    for file_path_ptr in file_path_ptrs {
        drop(CString::from_raw(file_path_ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_workspace(workspace: *mut Workspace) {
    if workspace.is_null() {
        return;
    }

    drop(Box::from_raw(workspace));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_ffi_boundary() {
        unsafe {
            let name = CString::new("test").unwrap().into_raw();
            let path = CString::new("test/example_workspace/").unwrap().into_raw();

            let workspace = create_workspace(name, path);

            destroy_workspace(workspace);

            drop(Box::from_raw(name));
            drop(Box::from_raw(path));
        }
    }

    #[test]
    fn test_workspace_files_ffi_boundary() {
        unsafe {
            let name = CString::new("test").unwrap().into_raw();
            let path = CString::new("test/example_workspace/").unwrap().into_raw();

            let workspace = create_workspace(name, path);

            let files = workspace_files(workspace);

            destroy_files(files);
            destroy_workspace(workspace);

            drop(Box::from_raw(name));
            drop(Box::from_raw(path));
        }
    }
}
