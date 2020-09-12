use std::ffi::c_void;
use std::os::raw::c_int;

unsafe impl Send for AccumulatorSys {}
unsafe impl Sync for AccumulatorSys {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AccumulatorSys {
    pub accumulated_count: ::std::os::raw::c_int,
    pub accumulated_sum: ::std::os::raw::c_int,
    pub max_accumulated: ::std::os::raw::c_int,
    pub callback: OnAccumulated,
    pub callback_data: *mut ::std::os::raw::c_void,
}

pub type OnAccumulated =
    unsafe extern "C" fn(accumulated: ::std::os::raw::c_int, data: *mut ::std::os::raw::c_void);

extern "C" {
    pub fn createAccumulator() -> AccumulatorSys;
    pub fn setMaxAccumulated(
        accumulator: *mut AccumulatorSys,
        max_accumulated: ::std::os::raw::c_int,
    );
    pub fn registerCallback(
        accumulator: *mut AccumulatorSys,
        callback: OnAccumulated,
        callback_data: *mut ::std::os::raw::c_void,
    );
    pub fn accumulate(accumulator: *mut AccumulatorSys, number: ::std::os::raw::c_int);
}

unsafe fn unpack_closure<F>(closure: &mut F) -> (OnAccumulated, *mut c_void)
where
    F: FnMut(c_int),
{
    extern "C" fn trampoline<F>(sum: c_int, data: *mut c_void)
    where
        F: FnMut(c_int),
    {
        let closure: &mut F = unsafe { &mut *(data as *mut F) };
        (*closure)(sum);
    }

    (trampoline::<F>, closure as *mut F as *mut c_void)
}

pub struct Accumulator {
    inner: AccumulatorSys,
}

impl Accumulator {
    pub fn new(max_numbers: i32) -> Self {
        let inner = unsafe {
            let mut accumulator = createAccumulator();
            setMaxAccumulated(&mut accumulator, max_numbers);
            accumulator
        };
        Self { inner }
    }

    pub fn add(&mut self, num: i32) {
        unsafe { accumulate(&mut self.inner, num) };
    }

    pub fn register_callback<F>(&mut self, cb: &mut F)
    where
        F: FnMut(i32),
    {
        unsafe {
            let (closure, callback) = unpack_closure(cb);
            registerCallback(&mut self.inner, closure, callback);
        }
    }
}
