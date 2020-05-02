use ::inject::{container, get, singleton, Provider};

mod fixtures;

use fixtures::*;
use rstest::*;

#[rstest]
fn test_construct(data: Data) {
    let container = container![];
    let injected_struct = get!(&container, Data).unwrap();

    assert_eq!(injected_struct, data);
}

#[rstest]
fn test_construct_with_dependency_using_inject(depends_on_data: DependsOnData) {
    let container = container![];
    let injected_struct = get!(&container, DependsOnData).unwrap();

    assert_eq!(injected_struct, depends_on_data);
}

#[rstest]
fn test_construct_with_dependency_using_provider(data_provider: impl Provider + 'static) {
    let expected_data = DependsOnData::new(Data::new(2), 0);
    let container = container![data_provider];

    let injected_struct = get!(&container, DependsOnData).unwrap();

    assert_eq!(expected_data, injected_struct);
}

#[rstest]
fn test_construct_with_generic_dependency(generic_data: GenericData<DependsOnData>) {
    let container = container![];

    let injected_struct = get!(&container, GenericData<DependsOnData>).unwrap();

    assert_eq!(generic_data, injected_struct);
}

#[rstest]
fn test_construct_with_arc_dependecy(
    arc_data: ArcData,
    data_arc_provider: impl Provider + 'static,
) {
    let container = container![data_arc_provider];

    let injected_struct = get!(&container, ArcData).unwrap();

    assert_eq!(arc_data, injected_struct);
}

#[rstest]
fn test_install_singleton_returns_same_instance(data: Data, arc_data: ArcData) {
    let container = container![singleton!(Data), singleton!(ArcData),];

    let injected_data_1 = get!(&container, std::sync::Arc<Data>).unwrap();
    let injected_data_2 = get!(&container, std::sync::Arc<Data>).unwrap();
    assert_eq!(injected_data_1, injected_data_2)
}
