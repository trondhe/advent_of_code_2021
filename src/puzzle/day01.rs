use crate::util::str2vec_numbers;

fn input() -> Vec<usize> {
    let input_string = crate::util::file2str("inputs/day01_sonar_sweep.txt");
    str2vec_numbers(&input_string)
}

pub fn solve() {
    println!("--- Day 1: Sonar Sweep ---");
    let input = input();

    let increases = sonar_sweep(&input);
    println!("\tsilver - increases {}", increases);

    let increases = sonar_sweep_sliding_sum(&input);
    println!("\tgold   - increases {}", increases);
}

fn sonar_sweep(depths: &[usize]) -> usize {
    if depths.len() == 0 {
        return 0;
    }
    let mut previous = depths.first().unwrap();
    let mut increases = 0;
    for depth in depths {
        if depth > previous {
            increases += 1;
        }
        previous = depth;
    }
    increases
}

fn sonar_sweep_sliding_sum(depths: &[usize]) -> usize {
    const WINDOW_SIZE: usize = 3;
    if depths.len() < 4 {
        return 0;
    }
    let mut previous = slice_sum(&depths[..WINDOW_SIZE]);
    let mut increases = 0;
    for index in 0..=depths.len() - WINDOW_SIZE {
        let depth_sum = slice_sum(&depths[index..index + WINDOW_SIZE]);
        if depth_sum > previous {
            increases += 1;
        }
        previous = depth_sum;
    }
    increases
}

fn slice_sum(window: &[usize]) -> usize {
    window.iter().sum::<usize>()
}

#[cfg(test)]
mod sonar_sweep_test {
    use super::*;

    #[test]
    fn given_none_return_zero() {
        assert_eq!(sonar_sweep(&[]), 0);
    }

    #[test]
    fn given_one_return_no_increment() {
        assert_eq!(sonar_sweep(&[10]), 0);
    }

    #[test]
    fn once_incremented() {
        assert_eq!(sonar_sweep(&[10, 20]), 1);
    }

    #[test]
    fn thrice_incremented() {
        assert_eq!(sonar_sweep(&[10, 20, 30, 40]), 3);
    }

    #[test]
    fn many_measurements_no_increases() {
        assert_eq!(sonar_sweep(&[50, 40, 40, 30, 20, 10]), 0);
    }

    #[test]
    fn silver_example_test() {
        assert_eq!(
            sonar_sweep(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn gold_example_test() {
        assert_eq!(
            sonar_sweep_sliding_sum(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
