use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use crate::core::Breaks;
use itertools::{Itertools, MinMaxResult};
use num_traits::Float;

pub fn get_equal_interval<F>(nums: &[F], num_classes: usize) -> Option<Breaks<F>>
where
    F: Float + Sub + Add,
{
    if nums.len() < num_classes {
        return None;
    }
    let (min, max) = match nums.iter().minmax() {
        MinMaxResult::NoElements => {
            return None;
        }
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(x, y) => (x, y),
    };
    let (min, max) = (*min, *max); // working around issues with &F
    let classes_as_F = F::from(num_classes).expect("num_classes could not be cast to Float");

    let step_size = (max - min) / classes_as_F;
    let get_start = |i| min + (step_size * i);

    let steps: Vec<F> = (0..=num_classes)
        .into_iter()
        .map(|i| get_start(F::from(i).expect("usize could not be cast to Float")))
        .collect();

    Some(Breaks::<F>(steps))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::data::{ALL_STATES_S1701_SORTED_F32, ALL_STATES_S1701_SORTED_F64};

    #[test]
    fn test_equal_interval() {
        let data = ALL_STATES_S1701_SORTED_F64;
        let num_classes = 7;
        let ans = get_equal_interval(&data, num_classes);
        assert!(ans.is_some());

        // Matches GeoStats JS
        let correct_breaks: Breaks<f64> = Breaks(vec![
            563528.0,
            6016351.857142857,
            11469175.714285715,
            16921999.57142857,
            22374823.42857143,
            27827647.285714287,
            33280471.14285714,
            38733295.0,
        ]);
        assert_eq!(ans, Some(correct_breaks))
    }

    #[test]
    fn test_too_many_classes() {
        let data = [1.0];
        let num_classes = 2;
        let ans = get_equal_interval(&data, num_classes);
        assert!(ans.is_none());
    }

    #[test]
    fn test_equal_length_data_and_classes() {
        let data = [1.0, 2.0];
        let num_classes = 2;
        let ans = get_equal_interval(&data, num_classes);
        assert!(ans.is_some());
        assert_eq!(ans.unwrap(), Breaks(vec![1., 1.5, 2.]));
    }

    #[test]
    fn test_equal_length_data_and_classes2() {
        let data = [1.0, 3.0];
        let num_classes = 2;
        let ans = get_equal_interval(&data, num_classes);
        assert!(ans.is_some());
        assert_eq!(ans.unwrap(), Breaks(vec![1.0, 2.0, 3.0,]));
    }
}
