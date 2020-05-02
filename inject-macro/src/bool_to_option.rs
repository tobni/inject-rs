pub(crate) trait BoolToOption: Copy + std::ops::Not<Output = Self> {
    fn and<T>(self, t: T) -> Option<T>;

    fn and_then<T>(self, f: impl FnOnce() -> T) -> Option<T>;

    #[inline]
    fn or<T>(self, t: T) -> Option<T> {
        (!self).and(t)
    }

    #[inline]
    fn or_then<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        (!self).and_then(f)
    }
}

impl BoolToOption for bool {
    #[inline]
    fn and<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    #[inline]
    fn and_then<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BoolToOption;
    #[test]
    fn test_and() {
        let a = true.and(1);
        let b = false.and(0);
        assert_eq!(a, Some(1));
        assert_eq!(b, None);
    }

    #[test]
    fn test_and_then() {
        let a = true.and_then(|| 1);
        let b = false.and_then(|| 0);
        assert_eq!(a, Some(1));
        assert_eq!(b, None);
    }

    #[test]
    fn test_or() {
        let a = true.or(1);
        let b = false.or(0);
        assert_eq!(a, None);
        assert_eq!(b, Some(0));
    }

    #[test]
    fn test_or_then() {
        let a = true.or_then(|| 1);
        let b = false.or_then(|| 0);
        assert_eq!(a, None);
        assert_eq!(b, Some(0));
    }
}
