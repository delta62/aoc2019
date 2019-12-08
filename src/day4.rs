use std::cmp;

const MIN_VALID: u32 = 111_111;
const MAX_VALID: u32 = 999_999;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (u32, u32) {
    let nums = input
        .split('-')
        .map(|s| s.parse::<u32>().expect("Invalid input given"))
        .collect::<Vec<_>>();

    if nums.len() != 2 {
        panic!("Invalid input given");
    }

    let min = cmp::max(MIN_VALID, nums[0]);
    let max = cmp::min(MAX_VALID, nums[1]);

    (min, max)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(u32, u32)) -> usize {
    let (min, max) = input;
    let mut solutions = 0;
    let mut state = normalize(digits(*min));
    let max = digits(*max);

    loop {
        if has_double(&state) {
            solutions += 1;
        }

        if !increment(&mut state, &max) {
            break;
        }
    }

    solutions
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(u32, u32)) -> usize {
    let (min, max) = input;
    let mut solutions = 0;
    let mut state = normalize(digits(*min));
    let max = digits(*max);

    loop {
        if has_exclusive_double(&state) {
            solutions += 1;
        }

        if !increment(&mut state, &max) {
            break;
        }
    }

    solutions
}

fn normalize(state: Vec<u32>) -> Vec<u32> {
    let mut last: u32 = 0;
    state
        .iter()
        .map(|&x| {
            if x < last {
                last
            } else {
                last = x;
                x
            }
        })
        .collect::<Vec<_>>()
}

fn digits(num: u32) -> Vec<u32> {
    num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

/// Try to increment digits, or return false if it isn't possible
fn increment(digits: &mut Vec<u32>, max: &Vec<u32>) -> bool {
    for i in (0..digits.len()).rev() {
        let digit = digits[i];
        if digit < 9 {
            let newdigit = digit + 1;
            for j in i..digits.len() {
                digits[j] = newdigit;
            }

            // Ensure that the new number is <= the max number
            let lte =  is_lte(digits, max);
            if lte {
                return true
            } else {
                return false
            }
        } else if i == 0 {
            return false
        }
    }
    false
}

fn is_lte(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    for (a, b) in a.iter().zip(b) {
        if a < b {
            return true
        }
        if b < a {
            return false
        }
    }
    return true
}

fn has_double(digits: &Vec<u32>) -> bool {
    digits
        .windows(2)
        .any(|pair| pair[0] == pair[1])
}

fn has_exclusive_double(digits: &Vec<u32>) -> bool {
    let mut streak_count = 0;
    let mut streak_digit = 0;
    for digit in digits.iter() {
        if *digit == streak_digit {
            streak_count += 1;
        } else {
            if streak_count == 2 {
                return true;
            }
            streak_count = 1;
            streak_digit = *digit;
        }
    }
    streak_count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_by_one() {
        let mut data = vec![ 1, 1, 1 ];
        let max = vec![ 9, 9, 9 ];
        assert!(increment(&mut data, &max));
        assert_eq!(data, vec![ 1, 1, 2 ]);
    }

    #[test]
    fn increments_middle_digit() {
        let mut data = vec![ 1, 1, 9 ];
        let max = vec![ 9, 9, 9 ];
        assert!(increment(&mut data, &max));
        assert_eq!(data, vec![ 1, 2, 2 ]);
    }

    #[test]
    fn increment_fails_on_max() {
        let mut data = vec![ 9, 9, 9 ];
        let max = vec![ 9, 9, 9 ];
        assert!(!increment(&mut data, &max));
        assert_eq!(data, vec![ 9, 9, 9 ]);
    }

    #[test]
    fn increment_fails_on_max_param() {
        let mut data = vec![ 1, 1, 1 ];
        let max = vec![ 1, 1, 1 ];
        assert!(!increment(&mut data, &max));
        assert_eq!(data, vec![ 1, 1, 2 ]);
    }

    #[test]
    fn has_double_detects_double() {
        assert!(has_double(&vec![ 1, 2, 3, 3, 4, 5 ]));
    }

    #[test]
    fn has_double_detects_no_double() {
        assert!(!has_double(&vec![ 1, 2, 3, 4, 5 ]));
    }

    #[test]
    fn has_exclusive_double_detects_double() {
        assert!(has_exclusive_double(&vec![ 1, 2, 3, 3, 4 ]));
    }

    #[test]
    fn has_exclusive_double_detects_double_and_triple() {
        assert!(has_exclusive_double(&vec![ 1, 1, 1, 2, 2 ]));
    }

    #[test]
    fn has_exclusive_double_fails_to_detect_triple() {
        assert!(!has_exclusive_double(&vec![ 1, 2, 2, 2, 3 ]));
    }

    #[test]
    fn is_lte_detects_gt() {
        assert!(!is_lte(&vec![ 6, 7 ], &vec![ 6, 6 ]));
    }

    #[test]
    fn is_lte_detects_lt() {
        assert!(is_lte(&vec![ 6, 7 ], &vec![ 6, 8 ]));
    }

    #[test]
    fn is_lte_detects_eq() {
        assert!(is_lte(&vec![ 6, 6 ], &vec![ 6, 6, ]));
    }
}
