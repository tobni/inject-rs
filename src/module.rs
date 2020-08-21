#[macro_export]
macro_rules! singleton {
    ($injectable:ty) => {{
        struct SingletonProvider {
            instance: std::sync::Mutex<Option<std::sync::Arc<$injectable>>>,
        }

        impl $crate::Provider for SingletonProvider {
            type ProvidedType = std::sync::Arc<$injectable>;
            fn provide(
                &self,
                c: &$crate::Container,
            ) -> Result<Self::ProvidedType, $crate::InjectError> {
                loop {
                    let mut instance = self.instance.lock().unwrap();

                    if let Some(instance) = instance.as_ref() {
                        return Ok(std::sync::Arc::clone(&instance));
                    } else {
                        let maybe_instance = $crate::get!(&c, $injectable);
                        match maybe_instance {
                            Ok(maybe_instance) => {
                                *instance = Some(std::sync::Arc::new(maybe_instance));
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
            }
        }
        SingletonProvider {
            instance: std::sync::Mutex::default(),
        }
    }};
}
