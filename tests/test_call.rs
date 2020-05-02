use ::inject::{call, container, inject};
use rstest::*;

mod fixtures;
use fixtures::*;

#[inject]
fn injected_func() {}

#[inject]
fn func_with_args(a: Data, b: Data) -> isize {
    a.a + b.a
}

#[rstest]
fn test_call() {
    let container = container![];
    call!(&container, injected_func).unwrap();
}

#[rstest]
fn test_call_with_injectable_args() {
    let container = container![];
    let a = call!(&container, func_with_args).unwrap();
    assert_eq!(a, 2)
}

#[rstest(data(2))]
fn test_call_with_injectable_args_with_kwarg(data: Data) {
    let container = container![];
    let expected_value = data.a + Data::inject(&container).unwrap().a;
    let return_value = call!(&container, func_with_args, kwargs = { b: data }).unwrap();
    assert_eq!(return_value, expected_value)
}

#[rstest(data(3))]
fn test_call_with_injectable_args_provider(data: Data) {
    let provider = move |_: &_| Ok(data);
    let container = container![provider];
    let a = call!(&container, func_with_args, kwargs = { b: Data::new(1) }).unwrap();
    assert_eq!(a, 4)
}
