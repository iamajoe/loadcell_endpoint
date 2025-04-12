#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub async fn calibrate_min_sleep(
    getter: &mut impl FnMut() -> f32,
    sleep: &impl AsyncFn(),
    max_count: usize,
) -> f32 {
    let mut cached_count = 0;
    let mut cache_sum = 0.0;

    // loop and collect the necessary data for the calibration
    while cached_count < max_count {
        sleep().await;

        // one more value cached
        cache_sum += getter();
        cached_count += 1;
    }

    cache_sum / cached_count as f32
}

pub async fn calibrate_min_frame(
    getter: &mut impl FnMut() -> f32,
    max_count: usize,
    interval: usize,
) -> f32 {
    let sleep = || async {
        let mut curr_frame = 0;
        while curr_frame < interval {
            curr_frame += 1;
        }
    };

    calibrate_min_sleep(getter, &sleep, max_count).await
}

pub fn is_num_over(num: f32, min: f32, deadzone: f32) -> bool {
    num - deadzone >= min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_min_frame() {
        struct SpecCase {
            getter: [f32; 5],
            max_count: usize,
            interval: usize,
            result: f32,
        }

        let cases = [
            SpecCase {
                getter: [1.0, 1.0, 1.0, 1.0, 1.0],
                max_count: 5,
                interval: 10,
                result: 1.0,
            },
            SpecCase {
                getter: [5.0, 2.0, 1.0, 3.0, 1.0],
                max_count: 5,
                interval: 1,
                result: 2.4,
            },
            SpecCase {
                getter: [5.0, 5.0, 1.0, 3.0, 2.0],
                max_count: 5,
                interval: 20,
                result: 3.2,
            },
            SpecCase {
                getter: [1.0, 2.0, 3.0, 4.0, 5.0],
                max_count: 5,
                interval: 2,
                result: 3.0,
            },
            SpecCase {
                getter: [1.0, 0.0, 0.0, 0.0, 0.0],
                max_count: 5,
                interval: 1,
                result: 0.2,
            },
        ];
        for case in cases.iter() {
            let mut _count = 0;
            let mut _getter = || {
                count += 1;
                case.getter[count - 1]
            };

            // TODO: tests cant be async, need something else for this
            // let _result = calibrate_min_frame(&mut getter, case.max_count, case.interval);
            // assert_eq!(result, case.result);
        }
    }

    #[test]
    fn test_is_num_over() {
        struct SpecCase {
            num: f32,
            min: f32,
            deadzone: f32,
            result: bool,
        }

        let cases = [
            SpecCase {
                num: 2.0,
                min: 1.0,
                deadzone: 1.0,
                result: true,
            },
            SpecCase {
                num: 1.0,
                min: 1.0,
                deadzone: 1.0,
                result: false,
            },
            SpecCase {
                num: 3.0,
                min: 1.0,
                deadzone: 0.0,
                result: true,
            },
        ];
        for case in cases.iter() {
            let result = is_num_over(case.num, case.min, case.deadzone);
            assert_eq!(result, case.result);
        }
    }
}
