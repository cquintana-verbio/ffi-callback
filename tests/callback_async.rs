use ffi_callback::Accumulator;

#[tokio::test]
async fn check_callback_async_channel() {
    let mut accumulator = Accumulator::new(2);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<i32>();
    let mut cb = move |sum: i32| {
        tx.send(sum).unwrap();
    };
    accumulator.register_callback(&mut cb);

    accumulator.add(1);
    accumulator.add(2);

    let val = rx.recv().await.unwrap();
    assert_eq!(val, 3);
}
