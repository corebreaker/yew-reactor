use backtrace::Backtrace;
use std::ffi::c_void;

#[inline(never)]
pub(crate) fn get_backtrace(skip: usize) -> Backtrace {
    let addr = (get_backtrace as usize) as *mut c_void;
    let frames = Backtrace::new().frames().iter()
        .skip_while(|f| f.symbols().iter().any(|s| s.addr() != Some(addr)))
        .skip(skip)
        .cloned()
        .collect::<Vec<_>>();

    Backtrace::from(frames)
}
