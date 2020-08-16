![Rust](https://github.com/tobni/inject-rs/workflows/Rust/badge.svg)

Experimental IOC library inpsired by [injector](https://github.com/alecthomas/injector) for Rust. Goals: IOC + ergonomics.

See [test suite](https://github.com/tobni/inject-rs/tree/master/tests) for all supported usages.

**Examples**

using `#[inject]`, `call!`, `get!` and `container!`.

1. Configure a container, add some provider, e.g an `Arc`
    ```rust
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
        let container = container![
            ref provider
        ];
    
        let instance: &Instance = get!(&container, &Instance).unwrap();
        
        assert_eq!(3, instance.0)
    }
    ```

2. Let the container resolve a dependency, using a closure as provider

    ```rust
    use ::inject::*;
   
    struct Instance(pub isize);
    
    impl Instance {
        #[inject]
        fn new(a: isize) -> Self {
            Instance(a)
        }
    }
    
    struct Service {
        a: Instance
    }
    
    impl Service {
        #[inject]
        fn new(instance: Instance) -> Self {
            Self { a: instance }
        }
    }
    
    
    fn main() {
        // Install a provider, this time a closure returning a value
        let container = container![
            |container: &Container| Ok(Instance(2))
        ];
    
        let service: Service = get!(&container, Service).unwrap();
        
        assert_eq!(service.a.0, 2)
    }
    ```

3. Sometimes, calling a function with injection is useful, 
    ```rust
   use ::inject::*;
   
   struct Service(isize);
   
   impl Service {
       #[inject]
       fn new() -> Self {
           Self
       }
    }
   
   #[inject] 
   fn acts_on_service(service: Service) -> usize { 
       2 + service.0
   }
   
   fn main() {
       let container = container![
           |container: &Container| Ok(Service(3))
       ];
       
       let result = call!(&container, acts_on_service).unwrap();
       
       assert_eq!(result, 5)
   }
   ```
4. `call!` supports a kwarg-flavored syntax
    ```rust
   use ::inject::*;
   
   struct Service(isize);
   
   impl Service {
       #[inject]
       fn new() -> Self {
           Self
       }
    }
   
   #[inject] 
   fn acts_on_service(service: Service) -> usize { 
       2 + service.0
   }
   
   fn main() {
       let container = container![];
       
       let result = call!(&container, acts_on_service, kwargs = { service: Service(2) }).unwrap();
       
       assert_eq!(result, 4)
   }
   ```

5. Dependency resolution can rely upon a type implementing the `Default` trait
    ```rust
   use ::inject::*;
   
   #[derive(Default)]
   struct Service(isize);
   
   fn main() {
       let container = container![];
       
       let service = get!(&container, Service).unwrap();
       
       assert_eq!(service.0, 0)
   }
   ```


**Details** 

The `get!` macro with a `container` resolves a type in order of: installed provider (1), calling the associated `inject` function (often generated with `#[inject]`) function on a type (2), and lastly the `Default` trait (3).

(2) & (3) can be opt-out by attribute `#[inject(no_inject(arg))]`, (name tbd) in which case only container held provider will be used for resolution of the type. Method specific defaults are annotated as `#[inject(defualt(arg = expression))]` where expression will lazy evaluate on failing attempt at (1) and (2).

Todo:
1. Support kwargs for "constructors" with a `create_object!` flavored macro.
2. Make `#[inject]` support Struct attribute notation with `#[inject(..)]` for individual struct fields. 
3. Make `default` and `no_inject` story less annoying.
