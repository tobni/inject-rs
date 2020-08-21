use ::inject::providers::DefaultProvider;
use ::inject::*;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct FakeImpl {
    val: isize,
}

impl FakeImpl {
    #[inject]
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

struct RefProvide {
    pub fake_impl: FakeImpl,
}

impl RefProvider for RefProvide {
    type ProvidedRef = FakeImpl;

    fn provide<'a>(
        &'a self,
        _container: &'a Container,
    ) -> Result<&'a Self::ProvidedRef, InjectError> {
        Ok(&self.fake_impl)
    }
}

struct BoxProvider {
    pub a: Box<FakeImpl>,
}

impl RefProvider for BoxProvider {
    type ProvidedRef = FakeImpl;

    fn provide<'a>(
        &'a self,
        _container: &'a Container,
    ) -> Result<&'a Self::ProvidedRef, InjectError> {
        Ok(&self.a)
    }
}

#[test]
fn test_default_provider() {
    let boxed_ref = FakeImpl { val: 1 };
    let box_provider = Box::from(boxed_ref);

    let container = container![DefaultProvider::<FakeImpl>::new(), ref box_provider,];
    let expected = Ok(FakeImpl { val: 0 });
    let provided = get!(&container, FakeImpl);

    assert_eq!(provided, expected);

    let reference: &FakeImpl = get!(&container, &FakeImpl).unwrap();
    assert_eq!(&boxed_ref, reference);
}
