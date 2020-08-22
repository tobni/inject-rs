use inject::{container, get};

#[derive(Default)]
struct Service(isize);

fn main() {
    let container = container![];

    let service = get!(&container, Service).unwrap();

    assert_eq!(service.0, 0)
}
