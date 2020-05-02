use std::any::TypeId;
use std::rc::Rc;
use std::sync::Arc;

use crate::inject::Inject;
use crate::Container;
use crate::InjectError;

pub trait Provider {
    type ProvidedType: 'static;

    fn provide(&self, container: &Container) -> Result<Self::ProvidedType, InjectError>;

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

pub trait RefProvider {
    type ProvidedRef: 'static;

    fn provide<'a>(
        &'a self,
        container: &'a Container,
    ) -> Result<&'a Self::ProvidedRef, InjectError>;

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
