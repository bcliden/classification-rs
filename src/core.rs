use itertools::Itertools;
use num_traits::Float;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RangeStyle {
    Inclusive,
    Exclusive,
}

unsafe impl Send for RangeStyle {}
unsafe impl Sync for RangeStyle {}
impl Unpin for RangeStyle {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataRange<T: PartialOrd> {
    pub start: T,
    pub end: T,
    pub style: RangeStyle,
}

unsafe impl<T: PartialOrd> Send for DataRange<T> {}
unsafe impl<T: PartialOrd> Sync for DataRange<T> {}
impl<T: PartialOrd> Unpin for DataRange<T> {}

impl<T: PartialOrd> DataRange<T> {
    pub fn new(start: T, end: T, style: RangeStyle) -> Self {
        DataRange { start, end, style }
    }

    pub fn contains(&self, value: T) -> bool {
        match self.style {
            RangeStyle::Exclusive => self.start <= value && value < self.end,
            RangeStyle::Inclusive => self.start <= value && value <= self.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Breaks<T: Float>(pub Vec<T>);

unsafe impl<T: Float> Send for Breaks<T> {}
unsafe impl<T: Float> Sync for Breaks<T> {}
impl<T: Float> Unpin for Breaks<T> {}

impl<T: Float> From<Ranges<T>> for Breaks<T> {
    fn from(other: Ranges<T>) -> Self {
        let mut v = vec![other.0[0].start];
        for r in other.0.iter() {
            v.push(r.end);
        }
        Breaks(v)
    }
}

impl<T: Float> IntoIterator for Breaks<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ranges<T: Float>(pub Vec<DataRange<T>>);

unsafe impl<T: Float> Send for Ranges<T> {}
unsafe impl<T: Float> Sync for Ranges<T> {}
impl<T: Float> Unpin for Ranges<T> {}

impl<T: Float> From<Breaks<T>> for Ranges<T> {
    fn from(other: Breaks<T>) -> Self {
        let end_idx = other.0.len().saturating_sub(2); // tuple windows below necessitates another -1
        Ranges(
            other
                .0
                .iter()
                .tuple_windows()
                .enumerate()
                .map(|(idx, (&from, &to))| {
                    if idx == end_idx {
                        DataRange::new(from, to, RangeStyle::Inclusive)
                    } else {
                        DataRange::new(from, to, RangeStyle::Exclusive)
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: Float> IntoIterator for Ranges<T> {
    type Item = DataRange<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaks_to_ranges() {
        let b = Breaks(vec![563528., 1739050., 4136542., 7116266., 38733295.]);
        let r = Ranges::from(b);
        assert_eq!(
            r,
            Ranges(vec![
                DataRange::new(563528.0, 1739050.0, RangeStyle::Exclusive),
                DataRange::new(1739050.0, 4136542.0, RangeStyle::Exclusive),
                DataRange::new(4136542.0, 7116266.0, RangeStyle::Exclusive),
                DataRange::new(7116266.0, 38733295.0, RangeStyle::Inclusive),
            ])
        );
        assert!(!r.0[1].contains(563528.0));
        assert!(r.0[1].contains(1739050.0));
        assert!(r.0[3].contains(38733295.0));
    }

    #[test]
    fn ranges_to_breaks() {
        let r = Ranges(vec![
            DataRange::new(563528.0, 1739050.0, RangeStyle::Exclusive),
            DataRange::new(1739050.0, 4136542.0, RangeStyle::Exclusive),
            DataRange::new(4136542.0, 7116266.0, RangeStyle::Exclusive),
            DataRange::new(7116266.0, 38733295.0, RangeStyle::Inclusive),
        ]);
        let b = Breaks::from(r);
        assert_eq!(
            b,
            Breaks(vec![563528., 1739050., 4136542., 7116266., 38733295.])
        );
    }
}
