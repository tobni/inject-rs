use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

pub mod error;
pub mod inject;
pub mod module;
pub mod provider;
pub mod providers;

pub use crate::inject::{Inject, InjectExt};
pub use error::InjectError;
pub use inject_macro::*;
pub use provider::{Provider, RefProvider};

#[derive(Debug, Default)]
pub struct Container {
    providers: HashMap<TypeId, Arc<dyn Any>>,
}

impl Container {
    pub fn new() -> Container {
        Self::default()
    }

    pub fn install<T: Inject, P: 'static + Provider<ProvidedType = T>>(&mut self, provider: P) {
        self.providers
            .insert(provider.id(), Arc::new(Self::box_provider(provider)));
    }

    pub fn install_ref<T: Inject, P: 'static + RefProvider<ProvidedRef = T>>(
        &mut self,
        provider: P,
    ) {
        self.providers
            .insert(provider.id(), Arc::new(Self::box_ref_provider(provider)));
    }

    pub fn get<T: Inject>(&self) -> Result<T, InjectError> {
        let provider = self
            .providers
            .get(&inject::id::<T>())
            .ok_or_else(|| InjectError::MissingProvider)?
            .downcast_ref::<Box<dyn Provider<ProvidedType = T>>>()
            .ok_or_else(|| InjectError::FailedCast)?;
        provider.provide(self)
    }

    pub fn get_ref<T: 'static>(&self) -> Result<&T, InjectError> {
        let provider = self
            .providers
            .get(&inject::id::<&T>())
            .ok_or_else(|| InjectError::MissingProvider)?
            .downcast_ref::<Box<dyn RefProvider<ProvidedRef = T>>>()
            .ok_or_else(|| InjectError::FailedCast)?;
        provider.provide(self)
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
