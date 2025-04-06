#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub fn calibrate(getter: fn() -> usize, max_count: usize, interval: usize) -> usize {
    let mut curr_frame = 0;
    let mut cached_count = 0;
    let mut cache_sum = 0;

    // loop and collect the necessary data for the calibration
    while cached_count < max_count {
        // wait for the interval to go through
        if interval > curr_frame {
            curr_frame += 1;
            continue;
        }

        // reset the frame
        curr_frame = 0;

        // one more value cached
        cache_sum += getter();
        cached_count += 1;
    }

    // return the median
    cache_sum / cached_count
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
