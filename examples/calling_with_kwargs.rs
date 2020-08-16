use ::inject::*;

struct Service(isize);

impl Service {
    #[inject]
    fn new() -> Self {
        Self(0)
    }
}

#[inject]
fn acts_on_service(service: Service) -> isize {
    2 + service.0
}

fn main() {
    let container = container![];

    let result = call!(&container, acts_on_service, kwargs = { service: Service(2) }).unwrap();

    assert_eq!(result, 4)
}
