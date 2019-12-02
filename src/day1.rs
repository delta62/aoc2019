#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse().expect("Unable to parse integer from input"))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, mass| acc + mass / 3 - 2)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, mass| {
        let mut tmp = *mass;
        let mut total = 0;
        loop {
            match tmp / 3 - 2 {
                r if !r.is_positive() => break,
                r => {
                    total += r;
                    tmp = r;
                },
            }
        }
        acc + total
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1ex1() {
        assert_eq!(solve_part1(&input_generator("12")), 2);
    }

    #[test]
    fn p1ex2() {
        assert_eq!(solve_part1(&input_generator("14")), 2);
    }

    #[test]
    fn p1ex3() {
        assert_eq!(solve_part1(&input_generator("1969")), 654);
    }

    #[test]
    fn p1ex4() {
        assert_eq!(solve_part1(&input_generator("100756")), 33583);
    }

    #[test]
    fn p2ex1() {
        assert_eq!(solve_part2(&input_generator("14")), 2);
    }

    #[test]
    fn p2ex2() {
        assert_eq!(solve_part2(&input_generator("1969")), 966);
    }

    #[test]
    fn p2ex3() {
        assert_eq!(solve_part2(&input_generator("100756")), 50346);
    }
}
