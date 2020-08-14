![Rust](https://github.com/tobni/inject-rs/workflows/Rust/badge.svg)

Experimental IOC library inpsired by [injector](https://github.com/alecthomas/injector) for Rust. Goals: IOC + ergonomics.

See [test suite](https://github.com/tobni/inject-rs/tree/master/tests) for all supported usages.

Example quick-fire usages:




```rust
use std::sync::Arc;

use ::inject::*;

struct Instance(pub isize);

impl Instance {
    #[inject]
    pub fn new(a: isize) -> Self {
        Instance(a)
    }
}

struct Service {
    a: Instance
}

impl Default for Service {
    fn default() -> Self {
        Service{ a: Instance(0) }
    }
}

impl Service {
    #[inject(default(instance = Instance(4)), no_inject(instance))]
    pub fn new(instance: Instance) -> Self {
        Self { a: instance }
    }
}

#[inject]
fn injectable(a: isize, b: &Instance) -> isize {
    a + b.0
}

fn main() {
    let provider = Arc::new(Instance(3));
    let container = container![
        ref provider,
        |_: &_| Ok(Instance(1))
    ];
    let instance = get!(&container, Instance).unwrap();
    
    let result = call!(&container, injectable, kwargs = { b: &instance }).unwrap();
    
    // isize::default() + Instance(1).0
    assert_eq!(result, 1);
    
    // 12 + Arc(Instance(3)).as_ref().0
    let result = call!(&container, injectable, kwargs = { a: 12 }).unwrap();
    
    assert_eq!(result, 15);
    
    let service = get!(&container, Service).unwrap();
    
    assert_eq!(service.a.0, 1); // 1 since value-provider returns Instance(1)
    
    let container = container![]; // new container, no providers
    
    let service = get!(&container, Service).unwrap();
    
    assert_eq!(service.a.0, 4); // The injection specific default, resolved since no provider for Instance
    
    let service = Service::default();
    
    assert_eq!(service.a.0, 0); // The trivial default impl, untouched
}
```

The `get!` macro with a `container` resolves a type in order of: installed provider (1), calling the associated `inject` function on a type (2), and lastly the `Default` implementation by a blanket impl of the `InjectExt` trait (3) by the default associated function resolution order.

The interaction with the `Default` impl is to imitiate ergonomics in [injector](https://github.com/alecthomas/injector).

(2) & (3) can be opt-out by attribute `#[inject(no_inject(arg))]`, (name tbd) in which case only container held provider will be used for resolution of the type. Method specific defaults are annotated as `#[inject(defualt(arg = expression))]` where expression will lazy evaluate on failing attempt at (1) and (2).

`call!` macro behaves similiarily to `get!`, but supports kwargs.

Todo:
1. Support kwargs for "constructors" with a `create_object!` flavored macro.
2. Make `#[inject]` support Struct attribute notation with `#[inject(..)]` for individual struct fields. 
3. Make `default` and `no_inject` story less annoying.
