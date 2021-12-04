use crate::util::file2str;

pub fn solve() {
    println!("--- Day 3: Binary Diagnostic ---");
    let (diagnostics, significant_bits) = input();
    let mut decoder = Decoder::new(significant_bits);
    decoder.decode(diagnostics);

    println!(
        "\tsilver - power consumption {}",
        decoder.power_consumption()
    );

    println!(
        "\tgold   - life support rating {}",
        decoder.life_support_rating()
    );
}

struct Decoder {
    significant_bits: usize,
    gamma: usize,
    epsilon: usize,
    oxygen: usize,
    co2: usize,
}

impl Decoder {
    pub fn new(significant_bits: usize) -> Self {
        Self {
            significant_bits,
            gamma: 0,
            epsilon: 0,
            oxygen: 0,
            co2: 0,
        }
    }

    pub fn decode(&mut self, diagnostics: Vec<usize>) {
        let diagnostics_length = diagnostics.len();
        let bitcounts = calculate_bitcounts(&diagnostics, self.significant_bits);
        let (gamma, epsilon) =
            calculate_rates(&bitcounts, diagnostics_length, self.significant_bits);
        self.gamma = gamma;
        self.epsilon = epsilon;
        self.oxygen = oxygen_generator_rating(&diagnostics, self.significant_bits);
        self.co2 = c02_scrubber_rating(&diagnostics, self.significant_bits);
    }

    pub fn power_consumption(&self) -> usize {
        self.gamma * self.epsilon
    }

    pub fn life_support_rating(&self) -> usize {
        self.oxygen * self.co2
    }
}

fn oxygen_generator_rating(data: &[usize], significant_bits: usize) -> usize {
    let mut data: Vec<usize> = Vec::from(data);
    for bit in (0..significant_bits).rev() {
        let bitcount = calculate_bitcount(&data, bit);
        let keep_ones = bitcount * 2 >= data.len();
        data = data
            .iter()
            .filter(|&&x| {
                if keep_ones {
                    bit_read(x, bit)
                } else {
                    !bit_read(x, bit)
                }
            })
            .map(|&x| x)
            .collect();

        if data.len() == 1 {
            return *data.first().unwrap();
        }
    }
    0
}

fn c02_scrubber_rating(data: &[usize], significant_bits: usize) -> usize {
    let mut data: Vec<usize> = Vec::from(data);
    for bit in (0..significant_bits).rev() {
        let bitcount = calculate_bitcount(&data, bit);
        let keep_zeroes = bitcount * 2 >= data.len();
        data = data
            .iter()
            .filter(|&&x| {
                if keep_zeroes {
                    !bit_read(x, bit)
                } else {
                    bit_read(x, bit)
                }
            })
            .map(|x| *x)
            .collect();

        if data.len() == 1 {
            return *data.first().unwrap();
        }
    }
    0
}

fn calculate_bitcounts(data: &[usize], significant_bits: usize) -> Vec<usize> {
    let mut bitcounts = vec![0usize; significant_bits];
    for (index, value) in bitcounts.iter_mut().enumerate() {
        *value = calculate_bitcount(&data, index);
    }
    bitcounts
}

fn calculate_bitcount(data: &[usize], index: usize) -> usize {
    data.iter().filter(|&&x| bit_read(x, index)).count()
}

fn calculate_rates(
    bitcounts: &[usize],
    total_count: usize,
    significant_bits: usize,
) -> (usize, usize) {
    let mut gamma = 0;
    for (index, bitcount) in bitcounts.iter().enumerate() {
        if *bitcount as usize > total_count / 2 {
            bit_set(&mut gamma, index);
        }
    }
    let mut epsilon = !gamma;
    let significant_bits_mask = (1 << significant_bits) - 1;
    epsilon &= significant_bits_mask;
    (gamma, epsilon)
}

fn bit_read(input: usize, n: usize) -> bool {
    input & (1 << n) != 0
}

#[allow(dead_code)]
fn bit_unset(input: &mut usize, n: usize) {
    let mask = !(1 << n);
    *input &= mask;
}

fn bit_set(input: &mut usize, n: usize) {
    let mask = 1 << n;
    *input |= mask;
}

#[allow(dead_code)]
fn print_vec(data: &[usize], significant: usize) {
    for val in data {
        bit_print(*val, significant);
    }
}

fn bit_print(input: usize, significant: usize) {
    let mut string = String::new();
    for bit in (0..significant).rev() {
        if bit_read(input, bit) {
            string.push('1');
        } else {
            string.push('0');
        }
    }
    println!("{}", string);
}

fn input() -> (Vec<usize>, usize) {
    let input_string = file2str("inputs/day03_binary_diagnostic.txt");

    let significant_bits = &input_string
        .split_whitespace()
        .next()
        .map(|line| line.len())
        .expect("Could not read length of first line in input");

    let mut data: Vec<usize> = Vec::new();
    for line in input_string.lines() {
        if let Some(bits) = bitline2usize(line) {
            data.push(bits);
        }
    }
    (data, *significant_bits)
}

fn bitline2usize(line: &str) -> Option<usize> {
    if let Ok(value) = usize::from_str_radix(line, 2) {
        return Some(value);
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_bitcounts_test() {
        let input = vec![0b11, 0b11];
        let expected = vec![2, 2];
        assert_eq!(calculate_bitcounts(&input, 2), expected);
    }

    #[test]
    fn calculate_bitcounts_test2() {
        let input = vec![0b11, 0b11, 0b10];
        let expected = vec![2, 3];
        assert_eq!(calculate_bitcounts(&input, 2), expected);
    }

    #[test]
    fn calculate_bitcounts_test3() {
        let input = vec![0b000000, 0b000000, 0b000001];
        let expected = vec![1, 0, 0, 0, 0, 0];
        assert_eq!(calculate_bitcounts(&input, 6), expected);
    }

    #[test]
    fn calculate_bitcounts_test4() {
        let input = vec![0b000000, 0b000010, 0b00000];
        let expected = vec![0, 1, 0, 0, 0, 0];
        assert_eq!(calculate_bitcounts(&input, 6), expected);
    }

    #[test]
    fn calculate_bitcounts_test5() {
        let input = vec![0b111111, 0b111111, 0b111111];
        let expected = vec![3, 3, 3, 3, 3, 3];
        assert_eq!(calculate_bitcounts(&input, 6), expected);
    }

    #[test]
    fn calculate_bitcounts_test6() {
        let input = vec![
            0b1000000000,
            0b0100000000,
            0b0010000000,
            0b0001000000,
            0b0000100000,
            0b0000010000,
            0b0000001000,
            0b0000000100,
            0b0000000010,
            0b0000000001,
        ];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(calculate_bitcounts(&input, 10), expected);
    }

    #[test]
    fn calculate_bitcounts_example_test() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let expected = vec![5, 7, 8, 5, 7];
        assert_eq!(calculate_bitcounts(&input, 5), expected);
    }

    #[test]
    fn calculate_rates_test() {
        let input = vec![2, 3, 4];
        let expected = (0b110, 0b001);
        assert_eq!(calculate_rates(&input, 4, 3), expected);
    }

    #[test]
    fn silver_example_test() {
        let diagnostics = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let mut decoder = Decoder::new(5);
        decoder.decode(diagnostics);
        assert_eq!(decoder.power_consumption(), 198);
    }

    #[test]
    fn oxygen_test() {
        let diagnostics = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let rating = oxygen_generator_rating(&diagnostics, 5);
        assert_eq!(rating, 23);
    }

    #[test]
    fn co2_test() {
        let diagnostics = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let rating = c02_scrubber_rating(&diagnostics, 5);
        assert_eq!(rating, 10);
    }
}
