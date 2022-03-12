use std::io::Write;
use std::collections::LinkedList;
use std::ops::{Shl, Shr};
use std::mem;

#[derive(PartialEq, Debug)]
pub struct Overflow;

#[derive(PartialEq, Debug)]
pub struct NotApplicable;

pub fn power_of_two(power: u32) -> Result<u32, Overflow> {
    if power < u32::BITS {
        Ok(1 << power)
    } else {
        Err(Overflow)
    }
}

fn process_binary_ones<F>(number: u32, mut f: F) where
    F: FnMut(u32) {
    let mut number = number;
    loop {
        f(number);
        let shifted = number >> 1;
        if shifted == 0 {
            break;
        }
        number = shifted;
    }
}

pub fn write_binary_representation<W: Write>(number: u32, bw: &mut W) {
    const ZERO_CHAR_NUM: u32 = 48;
    let mut binary_number_list = LinkedList::<u8>::new();
    let write = |number| {
        binary_number_list.push_front(u8::try_from(ZERO_CHAR_NUM + (number & 1)).unwrap());
    };
    process_binary_ones(number, write);
    bw.write_all(binary_number_list.into_iter().collect::<Vec<u8>>().as_slice()).unwrap();
}

pub fn binary_ones_count(number: u32) -> u32 {
    let mut count = 0u32;
    process_binary_ones(number, |number| count += number & 1);
    count
}

pub fn binary_ones_count_sub_method(number: u32) -> u32 {
    match number {
        0 => 0,
        number => {
            let mut number = number;
            let mut count = 0u32;
            loop {
                number &= number - 1;
                count += 1;
                if number == 0 {
                    break;
                }
            }
            count
        }
    }
}

pub fn hob(number: u32) -> Option<u32> {
    if number == 0 {
        return None;
    }

    let mut index = 0;
    process_binary_ones(number, |_| index += 1);
    Some(index - 1)
}

pub fn hob_thr(number: u32) -> Option<u32> {
    if number == 0 {
        return None;
    }

    let mut index = 0;
    let mut threshold = 1 << u32::BITS - 1;
    while number < threshold {
        threshold >>= 1;
        index += 1;
    }
    Some(u32::BITS - index - 1)
}

pub fn hob_comp_pot(number: u32) -> Option<u32> {
    if number == 0 {
        return None;
    }

    for i in (0 ..= u32::BITS - 1).rev() {
        let pow_of_two = 1u32 << i;
        if number & pow_of_two == pow_of_two {
            return Some(i);
        }
    }
    None
}

fn manipulate_bit<F>(index: u32, f: F) -> Option<u32> where
    F: Fn() -> u32 {
    if index >= u32::BITS {
        return None;
    }

    Some(f())
}

pub fn set_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, || number | 1 << index)
}

pub fn reset_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        (number | 1 << index) - (1 << index)
    )
}

pub fn reset_bit_xor(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number & (number ^ 1 << index)
    )
}

pub fn reset_bit_bitwise_not(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number & ! (1 << index)
    )
}

pub fn invert_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number ^ 1 << index
    )
}

fn circular_sh_base<F1, F2>(byte: u8, count: u32, f1: F1, f2: F2) -> u8 where
    F1: Fn(u8, u32) -> u8,
    F2: Fn(u8, u32) -> u8 {
    match byte {
        0 => 0,
        byte => {
            let safe_count = count % u8::BITS;
            match safe_count {
                0 => byte,
                count => f1(byte, count) | f2(byte, u8::BITS - count)
            }
        }
    }
}

pub fn circular_shl(byte: u8, count: u32) -> u8 {
    circular_sh_base(byte, count, u8::shl, u8::shr)
}

pub fn circular_shr(byte: u8, count: u32) -> u8 {
    circular_sh_base(byte, count, u8::shr, u8::shl)
}

fn consecutive_ones_number(consecutive_ones_count: u32) -> Option<u32> {
    const PEN_BIT: u32 = u32::BITS - 1;
    Some(
        match consecutive_ones_count {
            count @ 1..=PEN_BIT => (1 << count) - 1,
            u32::BITS => u32::MAX,
            _ => return None
        }
    )
}

pub fn consecutive_ones_entries_count(number: u32, consecutive_ones_count: u32) -> Option<u32> {
    let mut pattern = consecutive_ones_number(consecutive_ones_count)?;
    let mut matches = 0;
    const MAX_BIT: u32 = 1 << u32::BITS - 1;
    loop {
        if pattern & number == pattern {
            matches += 1;
        }
        
        if pattern & MAX_BIT == MAX_BIT {
            break;
        }

        pattern <<= 1;
    }
    Some(matches)
}

fn swap_bits_base<F>(number: u32, index1: u32, index2: u32, f: F) -> Option<u32> where
    F: Fn() -> u32 {
    let limits = 0..u32::BITS;
    if ! limits.contains(&index1) || ! limits.contains(&index2) {
        return None;
    }
    if index1 == index2 {
        return Some(number);
    }
    Some(f())
}

pub fn swap_bits(number: u32, index1: u32, index2: u32) -> Option<u32> {
    swap_bits_base(number, index1, index2, || {
        let mut index1 = index1;
        let mut index2 = index2;
        if index1 > index2 {
            mem::swap(&mut index1, &mut index2);
        }
        let distance = index2 - index1;
        let bit_index1 = 1 << index1;
        let bit_index2 = 1 << index2;
        number & (number ^ bit_index1 ^ bit_index2) | number >> distance & bit_index1 | number << distance & bit_index2
    })
}

pub fn swap_bits_xor(number: u32, index1: u32, index2: u32) -> Option<u32> {
    swap_bits_base(number, index1, index2, || {
        let bit1 = (number >> index1) & 1;
        let bit2 = (number >> index2) & 1;
        let mut swapper = bit1 ^ bit2;
        swapper = swapper << index1 | swapper << index2;
        number ^ swapper
    })
}

pub fn remove_bit(number: u32, index: u32) -> Option<u32> {
    if ! (0..u32::BITS).contains(&index) {
        return None;
    }

    let mut remover = number >> index + 1 ^ number >> index;
    remover <<= index;
    Some(number ^ remover)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_of_two() {
        assert_eq!(power_of_two(0), Ok(1));
        assert_eq!(power_of_two(2), Ok(4));
        assert_eq!(power_of_two(3), Ok(8));
        assert_eq!(power_of_two(31), Ok(2147483648));
        assert_eq!(power_of_two(32), Err(Overflow));
        assert_eq!(power_of_two(u32::MAX), Err(Overflow));
    }

    #[test]
    fn test_write_binary_representation() {
        let bin_rep_str = |number| -> String {
            let mut vec = Vec::<u8>::with_capacity(u32::BITS as usize);
            write_binary_representation(number, &mut vec);
            std::str::from_utf8(vec.as_slice()).unwrap().to_owned()
        };
        
        let mut str = bin_rep_str(0b11100100);
        assert_eq!(str, String::from("11100100"));

        str = bin_rep_str(u32::MAX);
        assert_eq!(str, String::from("11111111111111111111111111111111"));
        
        str = bin_rep_str(u32::MIN);
        assert_eq!(str, String::from("0"));
        
        str = bin_rep_str(1);
        assert_eq!(str, String::from("1"));
    }

    fn general_test_binary_ones_count<F>(f: F) where
        F: Fn(u32) -> u32 {
        let mut count = f(0b11100100);
        assert_eq!(count, 4);
        
        count = f(u32::MAX);
        assert_eq!(count, 32);
        
        count = f(u32::MIN);
        assert_eq!(count, 0);
        
        count = f(1);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_binary_ones_count() {
        general_test_binary_ones_count(binary_ones_count);
    }

    #[test]
    fn test_binary_ones_count_sub_method() {
        general_test_binary_ones_count(binary_ones_count_sub_method);
    }

    fn general_test_hob<F>(f: F) where
        F: Fn(u32) -> Option<u32> {
        let mut index = f(0b11100100);
        assert_eq!(index, Some(7));
        
        index = f(u32::MAX);
        assert_eq!(index, Some(31));
        
        index = f(1);
        assert_eq!(index, Some(0));
        
        index = f(4);
        assert_eq!(index, Some(2));
        
        index = f(u32::MIN);
        assert_eq!(index, None);
        
        index = f(1982);
        assert_eq!(1 << index.unwrap(), 0b10000000000); 
    }

    #[test]
    fn test_hob() {
        general_test_hob(hob);
    }

    #[test]
    fn test_hob_thr() {
        general_test_hob(hob_thr);
    }
    
    #[test]
    fn test_hob_comp_pot() {
        general_test_hob(hob_comp_pot);
    }
    
    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(9, 1), Some(11));
        assert_eq!(set_bit(9, 32), None);
        assert_eq!(set_bit(0, 0), Some(1));
        assert_eq!(set_bit(1, 0), Some(1));
        assert_eq!(set_bit(0b10110110000, 18), Some(263600));
    }

    fn general_test_reset_bit<F>(f: F) where
        F: Fn(u32, u32) -> Option<u32> {
        assert_eq!(f(11, 1), Some(9));
        assert_eq!(f(9, 32), None);
        assert_eq!(f(0, 0), Some(0));
        assert_eq!(f(1, 0), Some(0));
        assert_eq!(f(263600, 18), Some(0b10110110000));
        assert_eq!(f(0b11010, 5), Some(0b11010));
    }

    #[test]
    fn test_reset_bit() {
        general_test_reset_bit(reset_bit);
    }

    #[test]
    fn test_reset_bit_xor() {
        general_test_reset_bit(reset_bit_xor);
    }

    #[test]
    fn test_reset_bit_bitwise_not() {
        general_test_reset_bit(reset_bit_bitwise_not);
    }

    #[test]
    fn test_set_reset_bit() {
        for reset_bit_f in [reset_bit, reset_bit_xor] {
            for i in (0..u32::BITS).step_by(2) {
                assert_eq!(reset_bit_f(set_bit(0b10101010101010101010101010101010, i).unwrap(), i).unwrap(), 0b10101010101010101010101010101010);
            }
        }
    }
    
    #[test]
    fn test_invert_bit() {
        assert_eq!(invert_bit(0, 0), Some(1));
        assert_eq!(invert_bit(0, 1), Some(0b10));
        assert_eq!(invert_bit(5, 1), Some(7));
        
        for i in 0..u32::BITS {
            assert_eq!(invert_bit(invert_bit(0, i).unwrap(), i).unwrap(), 0);
        }
    }

    #[test]
    fn test_circular_shl() {
        assert_eq!(circular_shl(0b10000010, 1), 0b00000101);
        assert_eq!(circular_shl(0b11000010, 2), 0b00001011);
        assert_eq!(circular_shl(0b11000010, 10), 0b00001011);
        assert_eq!(circular_shl(0, 5), 0);
        assert_eq!(circular_shr(228, 0), 228);
        assert_eq!(circular_shl(0b10111010, 5), 0b1010111);
    }
    
    #[test]
    fn test_circular_shr() {
        assert_eq!(circular_shr(0b10000010, 1), 0b1000001);
        assert_eq!(circular_shr(0b10000011, 3), 0b1110000);
        assert_eq!(circular_shr(0, 5), 0);
        assert_eq!(circular_shr(0b11000010, 8), 0b11000010);
        assert_eq!(circular_shr(0b11000010, 9), 0b1100001);
        assert_eq!(circular_shr(0b10111010, 5), 0b11010101);
    }
    
    #[test]
    fn test_circular_shifts() {
        for count in 0..2 * u8::BITS {
            for number in 0 ..= u8::MAX {
                assert_eq!(circular_shl(circular_shr(circular_shr(circular_shl(number, count), count), count), count), number);
            }
        }
    }

    #[test]
    fn test_consecutive_ones_number() {
        let mut number = 1;
        for count in 1..u32::BITS {
            assert_eq!(consecutive_ones_number(count), Some(number));
            number |= 1 << count;
        }
        assert_eq!(consecutive_ones_number(0), None);
        assert_eq!(consecutive_ones_number(32), Some(u32::MAX));
        assert_eq!(consecutive_ones_number(33), None);
        assert_eq!(consecutive_ones_number(u32::MAX), None);
    }

    #[test]
    fn test_consecutive_ones_entries_count() {
        assert_eq!(consecutive_ones_entries_count(0b111011011, 1).unwrap(), 7);
        assert_eq!(consecutive_ones_entries_count(0b111011011, 2).unwrap(), 4);
        assert_eq!(consecutive_ones_entries_count(0b111011011, 3).unwrap(), 1);
        assert_eq!(consecutive_ones_entries_count(0b111011111, 3).unwrap(), 4);
        assert_eq!(consecutive_ones_entries_count(0b11110111, 2).unwrap(), 5);
        assert_eq!(consecutive_ones_entries_count(0b1111111111011110000001, 9).unwrap(), 2);
        assert_eq!(consecutive_ones_entries_count(u32::MAX, 32).unwrap(), 1);
        assert_eq!(consecutive_ones_entries_count(u32::MAX, 31).unwrap(), 2);
        assert_eq!(consecutive_ones_entries_count(u32::MAX, 0), None);
        assert_eq!(consecutive_ones_entries_count(0, 0), None);
        assert_eq!(consecutive_ones_entries_count(0, 1).unwrap(), 0);
    }

    fn general_test_swap_bits<F>(f: F) where
        F: Fn(u32, u32, u32) -> Option<u32> {
        assert_eq!(f(0b101, 2, 2), Some(0b101));
        assert_eq!(f(0b101, 228, 2), None);
        assert_eq!(f(0b101, 2, 282), None);
        assert_eq!(f(0b101, 0, 1), Some(0b110));
        assert_eq!(f(0b101, 1, 0), Some(0b110));
        assert_eq!(f(0, 0, 1), Some(0));
        assert_eq!(f(u32::MAX, 0, 31), Some(u32::MAX));
        assert_eq!(f(u32::MAX, 0, 32), None);
        assert_eq!(f(0b11001100101, 10, 1), Some(0b1001100111));
        assert_eq!(f(0b11001100101, 10, 5), Some(0b11001100101));
        assert_eq!(f(0b11001100101, 1, 31), Some(0b11001100101));
        assert_eq!(f(0b11001100101, 0, 31), Some(0b11001100100 | 1 << 31));
        const NUMBER: u32 = 0b110101;
        let mut number = NUMBER;
        number = f(number, 5, 1).unwrap();
        number = f(number, 5, 1).unwrap();
        assert_eq!(number, NUMBER);
    }

    #[test]
    fn test_swap_bits() {
        general_test_swap_bits(swap_bits);
    }

    #[test]
    fn test_swap_bits_xor() {
        general_test_swap_bits(swap_bits_xor);
    }
    
    #[test]
    fn test_remove_bit() {
        assert_eq!(remove_bit(11, 2), Some(7));
        assert_eq!(remove_bit(0b1110100, 3), Some(0b111100));
        assert_eq!(remove_bit(0b1011, 1), Some(0b101));
        assert_eq!(remove_bit(0, 0), Some(0));
        assert_eq!(remove_bit(228, 228), None);
        assert_eq!(1, (0..u32::BITS-1).fold(u32::MAX, |acc, _| remove_bit(acc, 0).unwrap()));
    }
}
