use std::sync::Arc;

use crate::inject::Inject;
use crate::provider::Provider;
use crate::Container;
use crate::InjectError;
use std::marker::PhantomData;

pub struct InstanceProvider<T: Inject> {
    pub instance: Arc<T>,
}

impl<T: Inject> Provider for InstanceProvider<T> {
    type ProvidedType = Arc<T>;

    fn provide(&self, _: &Container) -> Result<Self::ProvidedType, InjectError> {
        Ok(self.instance.clone())
    }
}

impl<T: Inject> InstanceProvider<T> {
    pub fn new(instance: T) -> Self {
        Self {
            instance: Arc::from(instance),
        }
    }

    pub fn install_into(self, container: &mut Container) {
        let cloned = Arc::clone(&self.instance);
        container.install(self);
        container.install_ref(cloned);
    }
}

#[derive(Debug, Default)]
pub struct DefaultProvider<T: Inject + Default> {
    type_: PhantomData<T>,
}

impl<T: Inject + Default> DefaultProvider<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Inject + Default> Provider for DefaultProvider<T> {
    type ProvidedType = T;

    fn provide(&self, _: &Container) -> Result<Self::ProvidedType, InjectError> {
        Ok(T::default())
    }
}
