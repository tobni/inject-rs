use ::inject::*;

struct Instance(pub isize);

impl Instance {
    #[inject]
    fn new(a: isize) -> Self {
        Instance(a)
    }
}

struct Service {
    a: Instance,
}

impl Service {
    #[inject]
    fn new(instance: Instance) -> Self {
        Self { a: instance }
    }
}

fn main() {
    // Install a provider, this time a closure returning a value
    let container = container![|container: &Container| Ok(Instance(2))];

    let service: Service = get!(&container, Service).unwrap();

    assert_eq!(service.a.0, 2)
}
