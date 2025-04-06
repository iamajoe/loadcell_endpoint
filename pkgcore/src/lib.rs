#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub fn calibrate_min_sleep(
    getter: &mut impl FnMut() -> usize,
    sleep: &impl Fn(),
    max_count: usize,
) -> usize {
    let mut cached_count = 0;
    let mut cache_sum = 0;

    // loop and collect the necessary data for the calibration
    while cached_count < max_count {
        sleep();

        // one more value cached
        cache_sum += getter();
        cached_count += 1;
    }

    // return the median
    cache_sum / cached_count
}

pub fn calibrate_min_frame(
    getter: &mut impl FnMut() -> usize,
    max_count: usize,
    interval: usize,
) -> usize {
    let sleep = || {
        let mut curr_frame = 0;
        while curr_frame < interval {
            curr_frame += 1;
        }
    };

    calibrate_min_sleep(getter, &sleep, max_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_min_frame() {
        struct SpecCase {
            getter: [usize; 5],
            max_count: usize,
            interval: usize,
            result: usize,
        }

        let cases = [
            SpecCase {
                getter: [1, 1, 1, 1, 1],
                max_count: 5,
                interval: 10,
                result: 1,
            },
            SpecCase {
                getter: [5, 2, 1, 3, 1],
                max_count: 5,
                interval: 1,
                result: 2,
            },
            SpecCase {
                getter: [5, 5, 1, 3, 2],
                max_count: 5,
                interval: 20,
                result: 3,
            },
            SpecCase {
                getter: [5, 5, 1, 3, 2],
                max_count: 5,
                interval: 2,
                result: 3,
            },
            SpecCase {
                getter: [1, 2, 3, 4, 5],
                max_count: 5,
                interval: 2,
                result: 3,
            },
            SpecCase {
                getter: [1, 0, 0, 0, 0],
                max_count: 5,
                interval: 1,
                result: 0,
            },
        ];
        for case in cases.iter() {
            let mut count = 0;
            let mut getter = || {
                count += 1;
                case.getter[count - 1]
            };

            let result = calibrate_min_frame(&mut getter, case.max_count, case.interval);
            assert_eq!(result, case.result);
        }
    }
}
