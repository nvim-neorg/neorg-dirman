use crate::workspace::Workspace;
use std::ffi::c_char;

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

    Box::into_raw(Box::new(workspace))
}

#[no_mangle]
// TODO: Is this return type okay? Any better way of signaling an array of strings?
pub unsafe extern "C" fn workspace_files(workspace: *const Workspace) -> *const *const c_char {
    if workspace.is_null() {
        return std::ptr::null_mut();
    }

    let files = (&*workspace).files();

    files
        .into_iter()
        .map(|path| path.to_string_lossy().as_ptr() as *const c_char)
        .collect::<Vec<*const c_char>>()
        .as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn destroy_workspace(workspace: &mut Workspace) {
    drop(workspace);
}
