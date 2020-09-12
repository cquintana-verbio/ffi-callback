use accumulator::Accumulator;

#[test]
fn check_callback_channel() {
    let mut accumulator = Accumulator::new(2);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut cb = move |sum: i32| {
        tx.send(sum).unwrap();
    };
    accumulator.register_callback(&mut cb);

    accumulator.add(1);
    accumulator.add(2);

    let val = rx.recv().unwrap();
    assert_eq!(val, 3);
}
