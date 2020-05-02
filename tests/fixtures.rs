use std::sync::Arc;

use ::inject::*;
use rstest::*;

pub trait TestTrait: Inject {
    fn hello(&self) -> &'static str {
        "Hello"
    }
}

pub struct TestTraitProvider {
    pub data: Arc<Data>,
}

impl Provider for TestTraitProvider {
    type ProvidedType = Arc<dyn TestTrait>;

    fn provide(&self, _: &Container) -> Result<Self::ProvidedType, InjectError> {
        Ok(self.data.clone())
    }
}

impl TestTrait for Data {}

pub struct DataArcProvider {
    pub data: Arc<Data>,
}

impl Provider for DataArcProvider {
    type ProvidedType = Arc<Data>;

    fn provide(&self, _: &Container) -> Result<Self::ProvidedType, InjectError> {
        Ok(self.data.clone())
    }
}

pub struct DependsOnDyn {
    pub test_trait: Arc<dyn TestTrait>,
}

impl DependsOnDyn {
    #[inject]
    pub fn new(test_trait: Arc<dyn TestTrait>) -> Self {
        Self { test_trait }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Data {
    pub a: isize,
}

impl Data {
    #[inject(default(a = 1), no_inject(a))]
    pub fn new(a: isize) -> Self {
        Data { a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DependsOnData {
    pub data: Data,
    pub b: isize,
}

impl DependsOnData {
    #[inject]
    pub fn new(data: Data, b: isize) -> Self {
        Self { data, b }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericData<T: Inject + Clone + PartialEq> {
    pub c: T,
}

impl GenericData<DependsOnData> {
    #[inject]
    pub fn new(c: DependsOnData) -> Self {
        Self { c }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArcData {
    pub data: Arc<Data>,
}

impl ArcData {
    #[inject]
    pub fn new(data: Arc<Data>) -> Self {
        Self { data }
    }
}

#[fixture(a = 1)]
pub fn data(a: isize) -> Data {
    Data::new(a)
}

#[fixture(b = 0)]
pub fn depends_on_data(data: Data, b: isize) -> DependsOnData {
    DependsOnData::new(data, b)
}

#[fixture]
pub fn generic_data(depends_on_data: DependsOnData) -> GenericData<DependsOnData> {
    GenericData::new(depends_on_data)
}

#[fixture]
pub fn arc_data(data: Data) -> ArcData {
    ArcData::new(Arc::new(data))
}

#[fixture(data = Data { a: 2 })]
pub fn data_provider(data: Data) -> impl Provider<ProvidedType = Data> {
    move |_: &_| Ok(data)
}

#[fixture]
pub fn depends_on_data_provider() -> impl Provider<ProvidedType = DependsOnData> {
    move |container: &Container| Ok(DependsOnData::new(container.get()?, 0))
}

#[fixture]
pub fn data_arc_provider(data: Data) -> impl Provider<ProvidedType = Arc<Data>> {
    DataArcProvider {
        data: Arc::new(data),
    }
}

#[fixture]
pub fn test_trait_provider(data: Data) -> TestTraitProvider {
    TestTraitProvider {
        data: Arc::new(data),
    }
}

#[fixture]
pub fn depends_on_dyn(data: Data) -> DependsOnDyn {
    DependsOnDyn::new(Arc::new(data))
}
