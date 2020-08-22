//! Library for dependency injection
//!
//! Inject implements macros for dependency resolution, attempting to promote the
//! [Inversion of control](https://en.wikipedia.org/wiki/Inversion_of_control) (IoC) principle.
//!
//! # Quick Start
//!
//! To get you started quickly, there are 3 procedural macros and one attribute macro to keep track of:
//! [`container!`](macro.container.html), [`get!`](macro.get.html), [`call!`](macro.call.html) and
//! [`#[inject]`](macro.inject.html).
//!
//! [`container!`](macro.container.html) constructs a new container with [providers](provider/index.html).
//!
//! [`get!`](macro.get.html) resolves a type using a container.
//!
//! [`call!`](macro.call.html) invokes a function using a container.
//!
//! [`#[inject]`](attr.inject.html) generates code to enable the above two macros.
//!
//! # Example
//!
//! ```
//! use ::inject::{container, get, inject};
//!
//! struct Connection(isize);
//!
//! impl Connection {
//!     #[inject]
//!     fn new(foo: isize) -> Self {
//!         Self(foo)
//!     }
//! }
//!
//! struct Instance(isize);
//!
//! impl Instance {
//!     #[inject]
//!     fn new(conn: &Connection) -> Self {
//!         Self(conn.0)
//!     }
//! }
//! let conn = Box::new(Connection(2));
//! let container = container![
//!     ref conn
//! ];
//!
//! let instance = get!(&container, Instance).unwrap();
//!
//! assert_eq!(instance.0, 2)
//!
//! ```
//!
//! The container resolves the dependencies of the `Instance` struct, using the installed provider to
//! resolve the `&Connection` dependency.

use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// Call a function with dependency resolution for its arguments
///
/// `call!(..) accepts 2-3 arguments.
/// 1. The first argument can be any expression, and should return a
/// reference to a [`Container`](struct.Container.html) instance.
/// 2. The second argument should be the name of a function that has been annotated using the
/// #[inject] attribute.
/// 3. Optionally, a sequence of keyword-value-arguments (kwargs) can be supplied on the form
/// `kwargs = {arg1: expr1, arg2: expr2}`, for a method with arguments
///
/// # Examples
///
/// ```
/// use ::inject::{call, container, inject};
///
/// // A function that can be called with call!(..)
/// #[inject]
/// fn foo(a: isize) -> isize {
///     a + 1
/// }
///
/// // An empty container.
/// let container  = container![];
///
/// // call "foo", argument(s) resolved using injection.
/// // This time, it will be resolved to 0 (default).
/// let result = call!(&container, foo).unwrap();
///
/// assert_eq!(result, 1);
///
/// // call "foo", this time using the provided kwarg "a = 2"
/// let result = call!(&container, foo, kwargs = { a: 2 }).unwrap();
///
/// assert_eq!(result, 3);
/// ```
///
///
/// Kwargs are parsed recursively and can be supplied in any order. They take precedence over any
/// installed provider for the invoking of the function.
///
/// ```
/// use ::inject::{call, Container, container, inject};
///
/// // A struct which is allowed to be constructed
/// // with a provided "isize" type.
/// struct Injectable(isize);
///
/// impl Injectable {
///     #[inject]
///     fn new(a: isize) -> Self {
///         Self(a)
///     }
/// }
///
/// #[inject]
/// fn callable(injectable: Injectable) {
///     println!("{} + {} = {}", injectable.0, 2, injectable.0 + 2);
/// }
///
/// let mut container = container![];
///
/// // prints 0 + 2 = 2
/// call!(&container, callable);
///
/// container.install(|container: &Container| Ok(Injectable::new(3)));
///
/// // prints 3 + 2 = 5
/// call!(&container, callable);
///
/// // prints 1 + 2 = 3
/// call!(&container, callable, kwargs = { injectable: Injectable::new(1) });
///
/// ```
///
/// Under the hood, if an arg is not provided a corresponding kwarg, the
/// [`get!`](macro.get.html) macro is used to evaluate the argument.
///
pub use inject_macro::call;

/// Create a container with providers
///
/// `container![..]` accepts any number of arguments, each which is expected to implement one of the
/// [provider traits](provider.mod.html)
///
pub use inject_macro::container;

/// Resolve a dependency from a container
///
/// `get!(..)` accepts 2-3 arguments.
/// 1. The first argument can be any expression, and should return a
/// reference to a [`Container`](struct.Container.html) instance.
/// 2. The second argument should be
/// a type which we want to resolve, optionally prepended by an '`&`' to indicate that we
/// want a reference.
/// 3. Lastly, the `create: (true|false)` key-value can be supplied to indicate
/// that we only want to use a `Provider` for the type, NOT the associated `inject` method.
///
/// # Example
///
/// ```
/// use inject::{Container, get};
///
/// // Derive default for brevity, see #[inject] for more intricate usages.
/// #[derive(Default)]
/// struct Foo;
///
/// let container = Container::new();
///
/// // 1. Attempt to resolve a reference
/// let result = get!(&container, &Foo);
///
/// // Fails because no reference-provider is installed into "container".
/// assert!(result.is_err());
///
/// // 2. Attempt to resolve a value
/// let result = get!(&container, Foo);
///
/// // Succeeds because the type could be resolved with injection using Foo::inject(&container).
/// assert!(result.is_ok());
///
/// // 3. Attempt to resolve a value, but ONLY using a Provider
/// let result = get!(&container, Foo, create: false);
///
/// // Fails because no value-provider is installed into "container".
/// assert!(result.is_err());
///
/// ```
///
/// The `create: (true|false)` key-value only holds meaning for value types. New references cannot be
/// created by the macro, as their corresponding instance is dropped on return.
///
pub use inject_macro::get;

/// Generate functionality for a function/constructor to be injectable
///
/// #[inject] accepts two positions: in a "free" function, or a struct impl method that returns `Self`.
///
/// # Examples
///
/// When in struct impl position, a new associated method `inject` is generated, in which
/// the `get!` macro is invoked for each argument.
/// ```
/// use ::inject::{Container, inject};
///
/// #[derive(Debug, PartialEq)]
/// struct A(String);
///
/// impl A {
///     #[inject]
///     fn new(string: String) -> Self {
///         Self(string)
///     }
/// }
///
/// let container = Container::new();
/// assert_eq!(A::new("".into()), A::inject(&container).unwrap())
///
/// ```
///
/// When in free function position, a set of macro_rules macros are generated (and hidden), which
/// enables injection and kwarg-style resolution of the function arguments.
///
/// ```
/// use ::inject::{call, Container, container, inject};
///
/// #[inject]
/// fn injectable(a: usize, b: usize) -> usize { a + b }
///
/// // A container with a value provider for "usize"
/// let container = container![
///     |container: &Container| Ok(2usize),
/// ];
///
/// // Call the function, letting injection resolve the values
/// let result = call!(&container, injectable).unwrap();
/// assert_eq!(result, 4);
///
/// // Use kwargs to provide a value for one of the args of the function
/// // By using macros generated by the #[inject] attribute.
/// let result = call!(&container, injectable, kwargs = { b: 12 }).unwrap();
/// assert_eq!(result, 14);
///
/// ```
pub use inject_macro::inject;

pub use error::InjectError;
pub use provider::{Provider, RefProvider};

pub use crate::inject::{Inject, InjectExt};

pub mod error;
pub mod inject;
pub mod module;
pub mod provider;
pub mod providers;

/// Contains providers for resolvable types.
///
/// The macro `container!` simplifies the construction of `Container`s calling
/// [`container.install(..)`](struct.Container.html#method.install) and
/// [`container.install_ref(..)`](struct.Container.html#method.install_ref) (if `ref` keyword is supplied)
/// on the macro arguments provided.
///
/// To resolve a dependecy by using the installed providers, methods
/// [`container.get()`](struct.Container.html#method.get) and
/// [`container.get_ref()`](struct.Container.html#method.get_ref) are called for values and references,
/// respectively.
///
///
/// # Example
///
/// ```
/// use inject::{Container, container};
///
/// let reference_provider = Box::new(5usize);
/// let container = container![
///     |container: &Container| Ok(2i32),
///     ref reference_provider,
/// ];
///
/// assert_eq!(5usize, *container.get_ref().unwrap());
/// assert_eq!(2i32, container.get().unwrap());
///
/// ```
#[derive(Clone, Debug, Default)]
pub struct Container {
    providers: HashMap<TypeId, Arc<dyn Any>>,
}

impl Container {
    /// Create a new `Container`, used to store implementors of
    /// [`Provider`](provider/trait.Provider.html)s and [`RefProvider`](provider/trait.RefProvider.html)s.
    pub fn new() -> Container {
        Self::default()
    }

    /// Install a [`Provider`](provider/trait.Provider.html) into this `Container`
    pub fn install<T: Inject, P: 'static + Provider<ProvidedType = T>>(&mut self, provider: P) {
        self.providers
            .insert(provider.id(), Arc::new(Self::box_provider(provider)));
    }

    /// Install a [`RefProvider`](provider/trait.RefProvider.html) into this `Container`
    pub fn install_ref<T: Inject, P: 'static + RefProvider<ProvidedRef = T>>(
        &mut self,
        provider: P,
    ) {
        self.providers
            .insert(provider.id(), Arc::new(Self::box_ref_provider(provider)));
    }

    /// Resolve a value-type from the installed [`Provider`](provider/trait.Provider.html)s.
    pub fn get<T: Inject>(&self) -> Result<T, InjectError> {
        let provider = self
            .providers
            .get(&inject::id::<T>())
            .ok_or_else(|| InjectError::MissingProvider)?
            .downcast_ref::<Box<dyn Provider<ProvidedType = T>>>()
            .ok_or_else(|| InjectError::FailedCast)?;
        provider.provide(self)
    }

    /// Resolve a reference-type from the installed [`RefProvider`](provider/trait.RefProvider.html)s.
    pub fn get_ref<T: 'static>(&self) -> Result<&T, InjectError> {
        let provider = self
            .providers
            .get(&inject::id::<&T>())
            .ok_or_else(|| InjectError::MissingProvider)?
            .downcast_ref::<Box<dyn RefProvider<ProvidedRef = T>>>()
            .ok_or_else(|| InjectError::FailedCast)?;
        provider.provide(self)
    }

    /// Clones the `Container`, returning a new container with the same providers. This is exactly
    /// equal to `Container::clone(&container)`.
    ///
    /// # Example
    /// ```
    /// use inject::{Container, container};
    ///
    /// let container = container![|container: &Container| Ok(2usize)];
    /// let child_container = container.create_child();
    ///
    /// assert_eq!(child_container.get::<usize>(), container.get())
    /// ```
    pub fn create_child(&self) -> Self {
        self.clone()
    }

    fn box_provider<T: 'static, P: 'static + Provider<ProvidedType = T>>(
        provider: P,
    ) -> Box<dyn Provider<ProvidedType = T>> {
        Box::new(provider)
    }

    fn box_ref_provider<T: 'static, P: 'static + RefProvider<ProvidedRef = T>>(
        provider: P,
    ) -> Box<dyn RefProvider<ProvidedRef = T>> {
        Box::new(provider)
    }
}
