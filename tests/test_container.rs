use ::inject::{container, get, Container, InjectError, Provider};
use std::sync::Arc;

use rstest::*;

mod fixtures;
use fixtures::*;

#[rstest]
fn test_get_without_provider_installed_errors() {
    let expected = Err(InjectError::MissingProvider);
    let container = Container::default();

    let provided = container.get::<Data>();

    assert_eq!(provided, expected)
}

#[rstest(data(2))]
fn test_get_with_provider_installed_returns_implementation(
    data: Data,
    data_provider: impl Provider + 'static,
) {
    let mut container = Container::new();
    container.install(data_provider);

    let provided = container.get::<Data>().unwrap();

    assert_eq!(provided, data)
}

#[rstest]
fn test_get_installed_trait_object(test_trait_provider: TestTraitProvider) {
    let mut container = Container::new();
    container.install(test_trait_provider);
    let provided = container.get::<Arc<dyn TestTrait>>().unwrap();

    assert_eq!(provided.hello(), "Hello")
}

#[rstest]
fn test_closure_as_provider(data: Data) {
    let expected = data;
    let mut container = Container::new();
    container.install(move |_: &_| Ok(data));
    let provided = container.get::<Data>().unwrap();

    assert_eq!(provided, expected)
}

#[rstest]
fn test_get_macro_gives_default_without_provider_and_default_available(data: Data) {
    let expected = data;
    let container = Container::new();
    let provided = get!(&container, Data).unwrap();

    assert_eq!(expected, provided)
}

#[rstest]
fn test_get_macro_resolves_default_in_nested_provider_when_provider_missing(
    depends_on_data: DependsOnData,
) {
    let expected = depends_on_data;
    let container = container![];
    let provided: DependsOnData = get!(&container, DependsOnData).unwrap();

    assert_eq!(expected, provided)
}

#[rstest]
fn test_get_macro_resolves_nested_provider(data_provider: impl Provider + 'static) {
    let expected = DependsOnData {
        data: Data::new(2),
        b: 0,
    };
    let container = container![data_provider];
    let provided = get!(&container, DependsOnData).unwrap();

    assert_eq!(expected, provided)
}

#[rstest]
fn test_get_macro_resolves_nested_provider_with_nested_provider(
    data_provider: impl Provider + 'static,
    depends_on_data_provider: impl Provider + 'static,
) {
    let expected = GenericData::new(DependsOnData::new(Data::new(2), 0));

    let container = container![data_provider, depends_on_data_provider];

    let provided = get!(&container, GenericData<DependsOnData>).unwrap();

    assert_eq!(expected, provided)
}

#[rstest]
fn test_get_macro_resolves_dyn_provider(test_trait_provider: impl Provider + 'static) {
    let container = container![test_trait_provider];

    let provided = get!(&container, DependsOnDyn).unwrap();

    assert_eq!(provided.test_trait.hello(), "Hello")
}

#[rstest]
fn test_get_macro_resolves_reference(data: Data) {
    let container = container![ref Box::new(data)];

    let provided = get!(&container, &Data).unwrap();

    assert_eq!(provided, &data)
}
