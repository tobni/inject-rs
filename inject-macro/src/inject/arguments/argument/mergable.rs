use syn::Result;
pub trait Mergable: Sized {
    fn merge(self, other: Self) -> Result<Self>;

    fn merge_many(mergables: impl IntoIterator<Item = Self>) -> Result<Option<Self>> {
        let mut mergables = mergables.into_iter();
        let first = mergables.next();
        Ok(if let Some(mut first) = first {
            for mergable in mergables {
                first = mergable.merge(first)?;
            }
            Some(first)
        } else {
            None
        })
    }
}
