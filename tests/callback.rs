use ffi_callback::Accumulator;

#[test]
fn check_callback_no_outer_memory() {
    let mut accumulator = Accumulator::new(2);
    let mut cb = move |sum: i32| {
        assert_eq!(sum, 3);
    };
    accumulator.register_callback(&mut cb);

    accumulator.add(1);
    accumulator.add(2);
}

#[test]
#[should_panic]
fn check_callback_no_outer_memory_works() {
    let mut accumulator = Accumulator::new(2);
    let mut cb = move |sum: i32| {
        assert_eq!(sum, 2);
    };
    accumulator.register_callback(&mut cb);

    accumulator.add(1);
    accumulator.add(2);
}
