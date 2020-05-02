use std::any::TypeId;
use std::sync::Arc;

pub trait Inject: 'static {}

pub trait InjectExt: Inject + Default {
    fn inject(container: &crate::Container) -> Result<Self, crate::InjectError> {
        Ok(Self::default())
    }
}

pub fn id<T: 'static>() -> TypeId {
    TypeId::of::<T>()
}

impl<T: 'static> Inject for T {}
impl<T: Inject + Default> InjectExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inject::Inject;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct FakeImpl {
        val: isize,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct FakeImpl2 {
        val: isize,
    }

    trait FakeTrait: Sized + Send {}

    impl FakeTrait for FakeImpl {}

    #[test]
    fn test_reference_of_type_does_not_share_type_id_with_type() {
        assert_ne!(id::<FakeImpl>(), id::<Arc<FakeImpl>>())
    }

    #[test]
    fn test_references_of_different_types_do_not_share_type_id() {
        assert_ne!(id::<Arc<FakeImpl>>(), id::<Arc<FakeImpl2>>())
    }
}
