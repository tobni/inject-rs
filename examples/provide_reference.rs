use std::sync::Arc;

use ::inject::*;

struct Instance(pub isize);

impl Instance {
    #[inject]
    fn new(a: isize) -> Self {
        Instance(a)
    }
}

fn main() {
    let provider = Arc::new(Instance(3));

    // Install the Arc as a reference provider, anytime using get!
    // will resolve to a reference of this Arc.
    let container = container![ref provider];

    let instance: &Instance = get!(&container, &Instance).unwrap();

    assert_eq!(3, instance.0)
}
