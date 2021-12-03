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

    println!("\tgold   - ");
}

struct Decoder {
    significant_bits: usize,
    gamma: usize,
    epsilon: usize,
}

impl Decoder {
    pub fn new(significant_bits: usize) -> Self {
        Self {
            significant_bits,
            gamma: 0,
            epsilon: 0,
        }
    }

    pub fn decode(&mut self, diagnostics: Vec<usize>) {
        let diagnostics_length = diagnostics.len();
        let bitcounts = calculate_bitcounts(diagnostics, 12);
        let (gamma, epsilon) =
            calculate_rates(bitcounts, diagnostics_length, self.significant_bits);
        self.gamma = gamma;
        self.epsilon = epsilon;
    }

    pub fn power_consumption(&self) -> usize {
        self.gamma * self.epsilon
    }
}

fn calculate_bitcounts(data: Vec<usize>, significant_bits: usize) -> Vec<usize> {
    let mut bitcounts: Vec<usize> = vec![0; significant_bits];
    for value in &data {
        let mut value = *value;
        let mut index: u8 = 0;
        while value > 0 && (index as usize) < significant_bits {
            if bit_read(value, index) {
                bitcounts[index as usize] += 1;
                bit_unset(&mut value, index);
            }
            index += 1;
        }
    }
    bitcounts
}

fn calculate_rates(
    bitcounts: Vec<usize>,
    total_count: usize,
    significant_bits: usize,
) -> (usize, usize) {
    let mut gamma = 0;
    for (index, bitcount) in bitcounts.iter().enumerate() {
        if *bitcount as usize > total_count / 2 {
            bit_set(&mut gamma, index as u8);
        }
    }
    let mut epsilon = !gamma;
    let significant_bits_mask = (1 << significant_bits) - 1;
    epsilon &= significant_bits_mask;
    (gamma, epsilon)
}

fn bit_read(input: usize, n: u8) -> bool {
    input & (1 << n) != 0
}

fn bit_unset(input: &mut usize, n: u8) {
    let mask = !(1 << n);
    *input &= mask;
}

fn bit_set(input: &mut usize, n: u8) {
    let mask = 1 << n;
    *input |= mask;
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
        assert_eq!(calculate_bitcounts(input, 2), expected);
    }

    #[test]
    fn calculate_bitcounts_test2() {
        let input = vec![0b11, 0b11, 0b10];
        let expected = vec![2, 3];
        assert_eq!(calculate_bitcounts(input, 2), expected);
    }

    #[test]
    fn calculate_bitcounts_test3() {
        let input = vec![0b000000, 0b000000, 0b000001];
        let expected = vec![1, 0, 0, 0, 0, 0];
        assert_eq!(calculate_bitcounts(input, 6), expected);
    }

    #[test]
    fn calculate_bitcounts_test4() {
        let input = vec![0b000000, 0b000010, 0b00000];
        let expected = vec![0, 1, 0, 0, 0, 0];
        assert_eq!(calculate_bitcounts(input, 6), expected);
    }

    #[test]
    fn calculate_bitcounts_test5() {
        let input = vec![0b111111, 0b111111, 0b111111];
        let expected = vec![3, 3, 3, 3, 3, 3];
        assert_eq!(calculate_bitcounts(input, 6), expected);
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
        assert_eq!(calculate_bitcounts(input, 10), expected);
    }

    #[test]
    fn calculate_bitcounts_example_test() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let expected = vec![5, 7, 8, 5, 7];
        assert_eq!(calculate_bitcounts(input, 5), expected);
    }

    #[test]
    fn calculate_rates_test() {
        let input = vec![2, 3, 4];
        let expected = (0b110, 0b001);
        assert_eq!(calculate_rates(input, 4, 3), expected);
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
}
