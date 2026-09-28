#![cfg_attr(any(), rustfmt::skip)]
pub const CreateObjectsAsync_ADDRESS: usize = 0x14055C160;
/// `CResourceLoaderRuntimeContainerHandler_CreateObjectsAsync`: the background job body that turns a
/// loaded runtime container into game objects. It walks the container's objects, creates each
/// through its registered game-object creator, applies the set-properties batches (attached
/// effects, physics parameters, events, model lookups), builds the hierarchy transforms, and hands
/// the result to `CGameWorldLoaderRC::OnLoaded`. `job_data` is the handler's job context; the
/// return value is the job status.
pub unsafe fn CreateObjectsAsync(job_data: *mut ::std::ffi::c_void) -> i32 {
    unsafe {
        let f: unsafe extern "system" fn(job_data: *mut ::std::ffi::c_void) -> i32 = ::std::mem::transmute(
            CreateObjectsAsync_ADDRESS,
        );
        f(job_data)
    }
}
