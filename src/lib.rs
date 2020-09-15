use bindings::{Accumulator as AccumulatorSys, AccumulatorCallback};
use std::ffi::c_void;

mod bindings;

#[repr(C)]
pub struct CallbackHolder {
    inner: Option<*mut AccumulatorCallback>,
    on_result: Box<dyn FnMut(i32)>,
    on_limit: Box<dyn FnMut()>,
}

pub struct Accumulator {
    inner: *mut AccumulatorSys,
    callback: Option<Box<CallbackHolder>>,
}

extern "C" fn trampoline_accumulated(value: i32, data: *mut c_void) {
    let holder: &mut Box<CallbackHolder> = unsafe { &mut *(data as *mut Box<CallbackHolder>) };
    (holder.on_result)(value);
}

extern "C" fn trampoline_limit(data: *mut c_void) {
    let holder: &mut Box<CallbackHolder> = unsafe { &mut *(data as *mut Box<CallbackHolder>) };
    (holder.on_limit)();
}

impl Accumulator {
    pub fn new(max_numbers: i32, limit: i32) -> Self {
        let inner = unsafe { bindings::createAccumulator() };
        unsafe { bindings::setLimit(inner, limit) };
        unsafe { bindings::setMaxAccumulated(inner, max_numbers) };
        Self {
            inner,
            callback: None,
        }
    }

    pub fn register_callbacks<R, L>(&mut self, on_result: R, on_limit: L)
    where
        R: FnMut(i32) + 'static,
        L: FnMut() + 'static,
    {
        let holder = CallbackHolder {
            inner: None,
            on_result: Box::new(on_result),
            on_limit: Box::new(on_limit),
        };

        let mut boxed = Box::new(holder);
        let inner = unsafe { bindings::createCallback(&mut boxed as *mut _ as *mut c_void) };
        unsafe { bindings::registerAccumulatedCallback(inner, Some(trampoline_accumulated)) }
        unsafe { bindings::registerLimitCallback(inner, Some(trampoline_limit)) }
        unsafe { bindings::setCallback(self.inner, inner) };
        boxed.inner = Some(inner);
        self.callback = Some(boxed);
    }

    pub fn add(&mut self, value: i32) {
        unsafe { bindings::accumulate(self.inner, value) }
    }
}

impl Drop for Accumulator {
    fn drop(&mut self) {
        if let Some(ref mut cb) = self.callback {
            if let Some(inner) = cb.inner {
                unsafe { bindings::freeCallback(inner) };
            }
            cb.inner = None;
        }
        unsafe { bindings::freeAccumulator(self.inner) };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::mpsc::sync_channel;

    #[test]
    fn correct_sync_once() {
        let mut accumulator = Accumulator::new(2, 3);

        let (result_tx, result_rx) = sync_channel::<i32>(1);
        let on_result = move |res: i32| {
            result_tx.send(res).unwrap();
        };
        let on_limit = || {};
        accumulator.register_callbacks(on_result, on_limit);

        accumulator.add(2);
        accumulator.add(3);

        let result = result_rx.recv().expect("Should be able to receive result");
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn correct_async_once() {
        let mut accumulator = Accumulator::new(2, 3);
        let (result_tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<i32>();
        let accumulated = move |sum: i32| {
            result_tx.send(sum).unwrap();
        };

        let limit = || {};

        accumulator.register_callbacks(accumulated, limit);

        accumulator.add(1);
        accumulator.add(2);

        let val = rx.recv().await.unwrap();
        assert_eq!(val, 3);
    }

    #[test]
    fn incorrect_sync_twice() {
        let mut accumulator = Accumulator::new(2, 3);

        let (result_tx, result_rx) = sync_channel::<i32>(1);
        let on_result = move |res: i32| {
            result_tx.send(res).unwrap();
        };
        let on_limit = || {};
        accumulator.register_callbacks(on_result, on_limit);

        accumulator.add(2);
        accumulator.add(3);

        let result = result_rx.recv().expect("Should be able to receive result");
        assert_eq!(result, 5);

        accumulator.add(3);
        accumulator.add(4);

        let result = result_rx.recv().expect("Should be able to receive result");
        assert_eq!(result, 7);
    }

    #[tokio::test]
    async fn incorrect_async_twice() {
        let mut accumulator = Accumulator::new(2, 3);
        let (result_tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<i32>();
        let accumulated = move |sum: i32| {
            result_tx.send(sum).unwrap();
        };

        let limit = || {};

        accumulator.register_callbacks(accumulated, limit);

        accumulator.add(1);
        accumulator.add(2);

        let val = rx.recv().await.unwrap();
        assert_eq!(val, 3);

        accumulator.add(4);
        accumulator.add(5);

        let val = rx.recv().await.unwrap();
        assert_eq!(val, 9);
    }

    #[test]
    fn incorrect_sync() {
        let mut accumulator = Accumulator::new(2, 3);

        let (result_tx, result_rx) = sync_channel::<i32>(1);
        let (limit_tx, limit_rx) = sync_channel::<()>(1);

        let on_result = move |res: i32| {
            result_tx.send(res).unwrap();
        };
        let on_limit = move || {
            limit_tx.send(()).unwrap();
        };
        accumulator.register_callbacks(on_result, on_limit);

        accumulator.add(2);
        accumulator.add(3);

        let result = result_rx.recv().expect("Should be able to receive result");
        assert_eq!(result, 5);

        accumulator.add(4);
        limit_rx.recv().expect("Should be able to receive limit");
    }

    #[tokio::test]
    async fn incorrect_async() {
        let mut accumulator = Accumulator::new(2, 3);
        let (result_tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<i32>();
        let accumulated = move |sum: i32| {
            result_tx.send(sum).unwrap();
        };

        let (limit_tx, mut limit_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let limit = move || {
            limit_tx.send(()).unwrap();
        };

        accumulator.register_callbacks(accumulated, limit);

        accumulator.add(1);
        accumulator.add(2);

        let val = rx.recv().await.unwrap();
        assert_eq!(val, 3);

        accumulator.add(3);
        limit_rx.recv().await.unwrap();
    }
}
