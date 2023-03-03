use std::ffi::c_char;
use crate::workspace::Workspace;

#[no_mangle]
pub unsafe extern "C" fn create_workspace(name: *const c_char, path: *const c_char) -> *mut Workspace {
    if name.is_null() || path.is_null() {
        return std::ptr::null_mut();
    }

    let (name, path) = (std::ffi::CStr::from_ptr(name), std::ffi::CStr::from_ptr(path));
    let workspace = Workspace { name: name.to_string_lossy().to_string(), path: path.to_string_lossy().to_string().into() };

    Box::into_raw(Box::new(workspace))
}

#[no_mangle]
pub unsafe extern "C" fn destroy_workspace(workspace: &mut Workspace) {
    drop(workspace);
}
