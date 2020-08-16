//! Provides types
//!
//! Providers are implementations that can resolve a types construction,
//! given a [`Container`](struct.Container.html), using the `provide(_ref)` methods.
//!
//! When a provider provides a value, the provider implements [`Provider`](trait.Provider.html).
//!
//! When a provider provides a reference, it implements [`RefProvider`](trait.RefProvider.html).
//!
//!
//! # Examples
//!
//! For convenience, `Box`, `Rc` and `Arc` types all implement `RefProvider`.
//! ```
//! use ::inject::*;
//!
//! let container = container![];
//!
//! let boxed_value = Box::new(5);
//!
//! assert_eq!(Ok(&5), boxed_value.provide(&container));
//! ```
//!
//! `Arc` also implements `Provider` using `Arc::clone`.
//! ```
//! use std::sync::Arc;
//!
//! use ::inject::*;
//! let arc = Arc::new(5isize);
//! let container = container![
//!     arc
//! ];
//!
//! assert_eq!(&5, get!(&container, Arc<isize>).unwrap().as_ref());
//! ```
//!
//! Closures that implement `Fn(&Container) -> Result<T, InjectError>`
//! also serves as factory functions for providing values as they implement `Provider`.
//!
//! ```
//! use ::inject::*;
//!
//! let container = container![
//!     |container: &Container| Ok(5usize)
//! ];
//!
//! assert_eq!(Ok(5), get!(&container, usize))
//! ```

use std::any::TypeId;
use std::rc::Rc;
use std::sync::Arc;

use crate::inject::Inject;
use crate::Container;
use crate::InjectError;

/// Value provider.
pub trait Provider {
    type ProvidedType: 'static;

    /// Provides the value using the `Container`
    fn provide(&self, container: &Container) -> Result<Self::ProvidedType, InjectError>;

    /// Returns the type id of the provided type.
    fn id(&self) -> TypeId {
        TypeId::of::<Self::ProvidedType>()
    }
}

impl<T: ?Sized + 'static> Provider for Arc<T> {
    type ProvidedType = Self;

    fn provide(&self, container: &Container) -> Result<Self::ProvidedType, InjectError> {
        Ok(Arc::clone(&self))
    }
}

impl<F, T: Inject> Provider for F
where
    F: Fn(&Container) -> Result<T, InjectError>,
{
    type ProvidedType = T;
    fn provide(&self, container: &Container) -> Result<Self::ProvidedType, InjectError> {
        self(container)
    }
}

/// Reference provider.
pub trait RefProvider {
    type ProvidedRef: 'static;

    /// Provides the reference using the `Container`
    fn provide<'a>(
        &'a self,
        container: &'a Container,
    ) -> Result<&'a Self::ProvidedRef, InjectError>;

    /// Returns the type id of the provided reference.
    fn id(&self) -> TypeId {
        TypeId::of::<&Self::ProvidedRef>()
    }
}

impl<T: Inject> RefProvider for Arc<T> {
    type ProvidedRef = T;

    fn provide<'a>(&'a self, _: &'a Container) -> Result<&'a T, InjectError> {
        Ok(&self)
    }
}

impl<T: Inject> RefProvider for Rc<T> {
    type ProvidedRef = T;

    fn provide<'a>(&'a self, _: &'a Container) -> Result<&'a T, InjectError> {
        Ok(&self)
    }
}

impl<T: Inject> RefProvider for Box<T> {
    type ProvidedRef = T;

    fn provide<'a>(&'a self, _: &'a Container) -> Result<&'a T, InjectError> {
        Ok(&self)
    }
}
