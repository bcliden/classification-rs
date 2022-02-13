/*
    =================
    Quantile classification (incl. Quartile)
    =================

# References:
- https://github.com/geotools/geotools/blob/1e036c73a4e420beeeb86c381c797a898a68c8a7/modules/library/main/src/main/java/org/geotools/feature/visitor/QuantileListVisitor.java#L58

*/

use crate::core::Breaks;
use num_traits::Float;

pub fn get_quantile<F>(sorted_nums: &[F], num_classes: usize) -> Option<Breaks<F>>
where
    F: Float,
{
    if sorted_nums.len() < num_classes {
        return None;
    }

    let step_size = sorted_nums.len() as f32 / num_classes as f32;
    let new_indices = (0..=num_classes)
        .into_iter()
        .map(|i| {
            let n = i as f32 * step_size;
            let item_index = n.ceil() as usize;
            let array_index = item_index.saturating_sub(1);
            sorted_nums.get(array_index).copied()
        })
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    Some(Breaks(new_indices))
}

pub fn get_quartile<F: Float>(nums: &[F]) -> Option<Breaks<F>> {
    get_quantile::<F>(nums, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::data::ALL_STATES_S1701_SORTED_F32;
    use crate::test::data::ALL_STATES_S1701_SORTED_F64;

    #[test]
    fn test_quantile() {
        let ans = get_quantile(&ALL_STATES_S1701_SORTED_F64, 7);
        assert!(ans.is_some());
        assert_eq!(
            ans.unwrap(),
            Breaks(vec![
                563528.0, 1018586.0, 1877629.0, 3167190.0, 5003235.0, 6656430.0, 10332523.0,
                38733295.0
            ])
        )
    }

    #[test]
    fn test_quartile() {
        let ans = get_quartile(&ALL_STATES_S1701_SORTED_F64);
        assert!(ans.is_some());
        assert_eq!(
            ans.unwrap(),
            Breaks(vec![563528., 1739050., 4136542., 7116266., 38733295.])
        )
    }
}
