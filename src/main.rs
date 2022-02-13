#![allow(non_snake_case, dead_code)]

/*

TODO thoughts:
- Jenks to return bounds, not groupings
- copy + sort data within functions(?)
- better use iterators
*/

mod core;
mod test;
mod naive_jenks;
mod combinatorics;
mod quantile;
mod equal;

use naive_jenks::{get_all_possible_jenks, get_best_jenks, get_jenks_above_tolerance};
use quantile::{get_quantile, get_quartile};
use equal::get_equal_interval;

const ALL_STATES_S1701: [i64; 52] = [
    4781688, 713725, 7116266, 2929117, 38733295, 5637904, 3460446, 944955, 673041, 21048884,
    10332523, 1379078, 1753946, 12373209, 6517430, 3058938, 2826818, 4326675, 4515876, 1304100,
    5898360, 6656430, 9772151, 5515416, 2877843, 5953025, 1042682, 1877629, 3037199, 1316495,
    8712974, 2053305, 18932499, 10199239, 738814, 11362386, 3841763, 4136542, 12387178, 3167190,
    1018586, 5003235, 854648, 6656385, 28361423, 3157996, 599030, 8279357, 7470152, 1739050,
    5675557, 563528,
];

fn main() {
    // let mut data = ALL_STATES_S1701;

    // /* Need to sort beforehand? */
    // data.sort_unstable();

    // mod a {
    //     use crate::core::{Breaks, Ranges, DataRange, RangeStyle};

    //     pub fn test() -> Ranges<f64> {
    //         let b = Breaks(vec![563528., 1739050., 4136542., 7116266., 38733295.]);
    //         let ranges = Ranges::from(b);
    //         assert_eq!(ranges, Ranges(vec![
    //             DataRange::new(563528.0, 1739050.0, RangeStyle::Exclusive),
    //             DataRange::new(1739050.0, 4136542.0, RangeStyle::Exclusive),
    //             DataRange::new(4136542.0, 7116266.0, RangeStyle::Exclusive),
    //             DataRange::new(7116266.0, 38733295.0, RangeStyle::Inclusive),
    //         ]));
    //         dbg!(ranges)
    //         // ranges
    //     }
    // }

    // for r in a::test() {
    //     let n = 38733295.;
    //     println!(" Comparing {} to range {:?}: found {}", 22, r, r.contains(n));
    // }
    // dbg!(a::test());

    // let number_of_classes = 5;
    // let tolerance = 0.95;
    // let result = get_jenks_above_tolerance(&data, number_of_classes, tolerance);
    // let result = get_quantile(&data, number_of_classes);
    // dbg!(result);
    // let combinations = get_all_possible_jenks(&data, number_of_classes);
    // println!("=================================");
    // println!("     Top classifications:");
    // println!("=================================");

    // for (idx, (classification, GVF)) in combinations.into_iter().take(3).enumerate() {
    //     println!("#{}", idx + 1);
    //     println!("GVF={:?}", GVF);
    //     println!("Classification=");

    //     println!("[");
    //     for class in classification {
    //         println!("\t{:?}", class);
    //     }
    //     println!("]");
    //     println!();
    // }

    // let (best_classification, best_gvf) = get_best_jenks(&mut data, number_of_classes).unwrap();
    // println!("BEST classification:\n\tGVF={}", best_gvf);
    // println!("[");
    // for class in best_classification {
    //     println!("{:?}", class);
    // }
    // println!("]");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main(){}
}
