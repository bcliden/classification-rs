/*

=================
Jenks Natural breaks classification
=================

GVF: goodness of variance fit ()
SDCM: sum of squared-deviations for class means
SDAM: sum of squared deviations for array means

GVF = SDAM - SDCM

Jenks Optimization Process

The method requires an iterative process. That is, calculations must be repeated using different breaks in the dataset to determine which set of breaks has the smallest in-class variance. The process is started by dividing the ordered data into classes in some way which may be arbitrary. There are two steps that must be repeated:
    1. Calculate the sum of squared deviations from the class means (SDCM).
    2. Choose a new way of dividing the data into classes, perhaps by moving one or more data points from one class to a different one.

New class deviations are then calculated, and the process is repeated until the sum of the within class deviations reaches a minimal value.

Alternatively, all break combinations may be examined, SDCM calculated for each combination, and the combination with the lowest SDCM selected. Since all break combinations are examined, this guarantees that the one with the lowest SDCM is found.

Finally the sum of squared deviations from the mean of the complete data set(SDAM), and the goodness of variance fit (GVF) may be calculated. GVF is defined as (SDAM - SDCM) / SDAM. GVF ranges from 0 (worst fit) to 1 (perfect fit).

## References
- https://en.wikipedia.org/wiki/Jenks_natural_breaks_optimization (**)
- https://www.ehdp.com/methods/jenks-natural-breaks-explain.htm (Surprisingly helpful **)
- https://www.spatialanalysisonline.com/HTML/index.html (see 'Univariate classification schemes' ***)

## Needs further review
- Matrix approach: (**)
    - https://web.archive.org/web/20100711083657/http://danieljlewis.org/2010/06/07/jenks-natural-breaks-algorithm-in-python/
        - https://stat.ethz.ch/pipermail/r-sig-geo/2006-March/000811.html 
        - https://github.com/geotools/geotools/blob/06d379a2fdbfdbf94641c16813c37ef02aa7f63c/modules/library/main/src/main/java/org/geotools/filter/function/JenksNaturalBreaksFunction.java#L77
- https://medium.com/analytics-vidhya/jenks-natural-breaks-best-range-finder-algorithm-8d1907192051 (*)
- https://macwright.com/2013/02/18/literate-jenks.html (simple-statistics author ♥)
    - http://web.archive.org/web/20130317024515/http://macwright.org/simple-statistics/docs/simple_statistics.html#section-96 (different markup)

*/

use std::cmp::Ordering;
use itertools::Itertools;
use crate::combinatorics::nCr;

pub fn get_jenks_above_tolerance(sorted_data: &[i64], num_classes: usize, tolerance: f64) -> Option<Vec<&[i64]>> {
    let combinations = get_combinations(sorted_data, num_classes);

    for combo in combinations {
        if calc_GVF(sorted_data, &combo) >= tolerance {
            return Some(combo);
        }
    }
    None
}

pub fn get_all_possible_jenks(nums: &[i64], num_classes: usize) -> Vec<(Vec<&[i64]>, f64)> {
    get_combinations(nums, num_classes)
        .into_iter()
        .map(|classification| {
            let gvf = calc_GVF(nums, &classification);
            (classification, gvf)
        })
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal))
        .collect()
}

pub fn get_best_jenks(nums: &[i64], num_classes: usize) -> Option<(Vec<&[i64]>, f64)> {
    get_all_possible_jenks(nums, num_classes)
        .into_iter()
        .take(1)
        .next()
}

/// The Goodness of Variance Fit (GVF) is found by taking the
/// difference between the squared deviations
/// from the array mean (SDAM) and the squared deviations from the
/// class means (SDCM), and dividing by the SDAM
fn calc_GVF(numbers: &[i64], classification: &[&[i64]]) -> f64 {
    let SDAM = calc_SDAM(numbers);
    let SDCM_ALL = calc_SDCM_ALL(classification);
    (SDAM - SDCM_ALL) / SDAM
}

fn calc_SDAM(data: &[i64]) -> f64 {
    let mean = data.iter().sum::<i64>() as f64 / data.len() as f64;
    data.iter()
        .fold(0.0, |acc, &n| acc + ((n as f64) - mean).powf(2.0))
}

fn calc_SDCM_ALL(classes: &[&[i64]]) -> f64 {
    classes.iter().map(|&class| calc_SDCM(class)).sum()
}

fn calc_SDCM(class: &[i64]) -> f64 {
    let mean = class.iter().sum::<i64>() as f64 / class.len() as f64;
    class.iter().map(|&n| f64::powf(n as f64 - mean, 2.0)).sum()
}

fn get_combinations(nums: &[i64], num_classes: usize) -> Vec<Vec<&'_ [i64]>> {
    let length = nums.len();
    let breaks = num_classes - 1;

    // generate combinations WITH min and max included
    let combos = (0..length)
        .into_iter()
        .combinations(breaks)
        .map(|mut c| {
            let mut v = vec![0]; // minimum
            v.append(&mut c); // append is weird, would prefer something that consumes the Other
            v.push(length - 1); // maximum
            v
        });

    // let mut res: Vec<Vec<&[i64]>> = Default::default(); // is this any worse than
    let mut results: Vec<Vec<&[i64]>> = vec![Vec::with_capacity(breaks); nCr(length as i64, breaks as i64) as usize]; // this, really?
    for (combo_idx, combination) in combos.enumerate() {
        // res.push(vec![]);
        let combination_storage = results.get_mut(combo_idx).unwrap();

        for (slice_idx, (prev_idx, cur_idx)) in (0..combination.len())
            .tuple_windows()
            .map(|(prev, cur)| (combination[prev], combination[cur]))
            .enumerate()
        {
            if
            /* is first */
            slice_idx == 0 {
                combination_storage.push(nums[prev_idx..=cur_idx].into());
            } else if
            /* is last */
            slice_idx == combination.len() - 1 {
                combination_storage.push(nums[prev_idx..cur_idx].into());
            } else {
                combination_storage.push(nums[(prev_idx + 1)..=cur_idx].into());
            };
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdam() {
        let nums: Vec<i64> = vec![4, 5, 9, 10];
        let sdam = calc_SDAM(&nums);
        assert_eq!(sdam, 26.0)
    }

    #[test]
    fn test_sdcm() {
        assert_eq!(calc_SDCM(&[5, 9, 10]), 14.0);
        assert_eq!(calc_SDCM(&[9, 10]), 0.5);
        assert_eq!(calc_SDCM(&[4, 5, 9]), 14.0);
        assert_eq!(calc_SDCM(&[10]), 0.0);
    }

    #[test]
    fn test_sdcm_all() {
        assert_eq!(calc_SDCM_ALL(&[&[4], &[5, 9, 10]]), 14.0);
        assert_eq!(calc_SDCM_ALL(&[&[4, 5], &[9, 10]]), 1.0);
        assert_eq!(calc_SDCM_ALL(&[&[4, 5, 9], &[10]]), 14.0);
    }

    #[test]
    fn test_gvf_fit() {
        assert_eq!(
            calc_GVF(&[4, 5, 9, 10], &[&[4], &[5, 9, 10]]),
            0.46153846153846156
        );
        assert_eq!(
            calc_GVF(&[4, 5, 9, 10], &[&[4, 5], &[9, 10]]),
            0.9615384615384616
        );
        assert_eq!(
            calc_GVF(&[4, 5, 9, 10], &[&[4, 5, 9], &[10]]),
            0.46153846153846156
        );
    }
}
