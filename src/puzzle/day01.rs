use crate::util::str2vec_numbers;

pub fn silver() {
    let input_string = crate::util::file2str("inputs/day01_sonar_sweep.txt");
    let input = str2vec_numbers(&input_string);
    let increases = sonar_sweep(&input);
    println!("Day 1 sonar sweep - increases {}", increases);
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
    fn example_test() {
        assert_eq!(
            sonar_sweep(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }
}
