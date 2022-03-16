#![allow(clippy::precedence)]

use std::io::Write;
use std::collections::LinkedList;
use std::ops::{Shl, Shr, BitXor};
use std::mem;

/// Unit. Used when something is overflowed. Meant to be used as *E* parameter of *Err* type inside the *Result* type
#[derive(PartialEq, Debug)]
pub struct Overflow;

/// Returns the result of raising two to a power or error if resulting value is not in available integer range
/// # Arguments
/// * `power` - number, the power to which two will be raised
/// # Examples
/// ```
/// # use understanding_bitwise::{Overflow, power_of_two};
/// assert_eq!(power_of_two(3), Ok(8));
/// assert_eq!(power_of_two(45), Err(Overflow));
/// ```
pub fn power_of_two(power: u32) -> Result<u32, Overflow> {
    if power < u32::BITS {
        Ok(1 << power)
    } else {
        Err(Overflow)
    }
}

/// Helper function, base function. Used to process binary, performs a shift until high order bit is met
/// # Arguments
/// * `number` - number to work with
/// * `f` - function that is called every iteration of the processing
fn process_binary_until_hob<F>(number: u32, mut f: F)
where
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

/// Writes a binary representation to the *Write* object
/// # Arguments
/// * `number` - number to work with
/// * `bw` - *Write* object that gets a binary representation of the number
/// # Examples
/// ```
/// # use understanding_bitwise::write_binary_representation;
/// let mut vec = Vec::<u8>::with_capacity(u32::BITS as usize);
/// write_binary_representation(0b101, &mut vec);
/// let str = std::str::from_utf8(vec.as_slice()).unwrap().to_owned();
/// assert_eq!(str, "101");
/// ```
pub fn write_binary_representation<W: Write>(number: u32, bw: &mut W) {
    // ASCII/UTF-8 code of char 0. The next number is code of char 1.
    const ZERO_CHAR_NUM: u32 = 48;
    let mut binary_number_list = LinkedList::<u8>::new();
    let write = |number| {
        binary_number_list.push_front(u8::try_from(ZERO_CHAR_NUM + (number & 1)).unwrap());
    };
    process_binary_until_hob(number, write);
    bw.write_all(binary_number_list.into_iter().collect::<Vec<u8>>().as_slice()).unwrap();
}

/// Returns the count of ones in binary representation of the number
/// # Arguments
/// * `number` - number to work with
/// # Examples
/// ```
/// # use understanding_bitwise::binary_ones_count;
/// assert_eq!(binary_ones_count(0b101), 2);
/// ```
pub fn binary_ones_count(number: u32) -> u32 {
    let mut count = 0u32;
    process_binary_until_hob(number, |number| count += number & 1);
    count
}

/// Returns the count of ones in binary representation of the number.
/// Uses subtraction method
/// # Arguments
/// * `number` - number to work with
/// # Examples
/// ```
/// # use understanding_bitwise::binary_ones_count_sub_method;
/// assert_eq!(binary_ones_count_sub_method(0b101), 2);
/// ```
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

/// Returns true if number can't have hob
/// # Arguments
/// * `number` - number to work with
fn no_hob(number: u32) -> bool {
    number == 0
}

/// Returns [*highest order bit*](https://commoncog.com/blog/highest-order-bit/) or None if number can't have hob
/// # Arguments
/// * `number` - number to work with
/// # Examples
/// ```
/// # use understanding_bitwise::hob;
/// assert_eq!(hob(0), None);
/// assert_eq!(hob(0b100), Some(2));
/// ```
pub fn hob(number: u32) -> Option<u32> {
    if no_hob(number) {
        return None;
    }

    let mut index = 0;
    process_binary_until_hob(number, |_| index += 1);
    Some(index - 1)
}

/// Returns [*highest order bit*](https://commoncog.com/blog/highest-order-bit/) or None if number can't have hob. Uses threshold method.
/// # Arguments
/// * `number` - number to work with
/// # Examples
/// ```
/// # use understanding_bitwise::hob_thr;
/// assert_eq!(hob_thr(0), None);
/// assert_eq!(hob_thr(0b100), Some(2));
/// ```
pub fn hob_thr(number: u32) -> Option<u32> {
    if no_hob(number) {
        return None;
    }

    let mut index = u32::BITS - 1;
    let mut threshold = 1 << u32::BITS - 1;
    while number < threshold {
        threshold >>= 1;
        index -= 1;
    }
    Some(index)
}

/// Returns [*highest order bit*](https://commoncog.com/blog/highest-order-bit/) or None if number can't have hob. Uses comparison for equality with a power of two method.
/// # Arguments
/// * `number` - number to work with
/// # Examples
/// ```
/// # use understanding_bitwise::hob_comp_pot;
/// assert_eq!(hob_comp_pot(0), None);
/// assert_eq!(hob_comp_pot(0b100), Some(2));
/// ```
pub fn hob_comp_pot(number: u32) -> Option<u32> {
    if no_hob(number) {
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

/// Helper function, base function. Used to check if the bit index is in a valid range
/// # Arguments
/// * `index` - index of the bit to be manipulated with
/// * `f` - function that performing manipulations with a bit
fn manipulate_bit<F>(index: u32, f: F) -> Option<u32>
where
    F: Fn() -> u32 {
    if index >= u32::BITS {
        return None;
    }

    Some(f())
}

/// Returns a copy of the original number with the specific bit set to 1
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the specific bit
/// # Examples
/// ```
/// # use understanding_bitwise::set_bit;
/// assert_eq!(set_bit(0b101, 1), Some(0b111));
/// assert_eq!(set_bit(0b100, 2), Some(0b100));
/// assert_eq!(set_bit(0b100, 45), None);
/// ```
pub fn set_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, || number | 1 << index)
}

/// Returns a copy of the original number with the specific bit set to 0
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the specific bit
/// # Examples
/// ```
/// # use understanding_bitwise::unset_bit;
/// assert_eq!(unset_bit(0b101, 2), Some(0b1));
/// assert_eq!(unset_bit(0b100, 1), Some(0b100));
/// assert_eq!(unset_bit(0b100, 45), None);
/// ```
pub fn unset_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        (number | 1 << index) - (1 << index)
    )
}

/// Returns a copy of the original number with the specific bit set to 0. Uses xor method.
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the specific bit
/// # Examples
/// ```
/// # use understanding_bitwise::unset_bit_xor;
/// assert_eq!(unset_bit_xor(0b101, 2), Some(0b1));
/// assert_eq!(unset_bit_xor(0b100, 1), Some(0b100));
/// assert_eq!(unset_bit_xor(0b100, 45), None);
/// ```
pub fn unset_bit_xor(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number & (number ^ 1 << index)
    )
}

/// Returns a copy of the original number with the specific bit set to 0. Uses bitwise not method.
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the specific bit
/// # Examples
/// ```
/// # use understanding_bitwise::unset_bit_bitwise_not;
/// assert_eq!(unset_bit_bitwise_not(0b101, 2), Some(0b1));
/// assert_eq!(unset_bit_bitwise_not(0b100, 1), Some(0b100));
/// assert_eq!(unset_bit_bitwise_not(0b100, 45), None);
/// ```
pub fn unset_bit_bitwise_not(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number & ! (1 << index)
    )
}

/// Returns a copy of the original number with the specific bit inverted. If the bit was 0 it becomes 1 and vice versa
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the specific bit
/// # Examples
/// ```
/// # use understanding_bitwise::invert_bit;
/// assert_eq!(invert_bit(0b101, 2), Some(0b1));
/// assert_eq!(invert_bit(0b100, 1), Some(0b110));
/// assert_eq!(invert_bit(0b100, 45), None);
/// ```
pub fn invert_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number ^ 1 << index
    )
}

/// Helper function, base function. Circular shifts (left and right) are similar and have same body (but different operations performed in places)
/// # Arguments
/// * `byte` - number to work with
/// * `count` - number of positions to be shifted by
/// * `f1` - function to be performed at the first part of expression
/// * `f2` - function to be performed at the second part of expression
fn circular_sh_base<F1, F2>(byte: u8, count: u32, f1: F1, f2: F2) -> u8
where
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

/// Returns left [*circularly shifted*](https://en.wikipedia.org/wiki/Circular_shift) number
/// # Arguments
/// * `byte` - number to work with
/// * `count` - number of positions to be shifted by
/// # Examples
/// ```
/// # use understanding_bitwise::circular_shl;
/// assert_eq!(circular_shl(0b10000011, 2), 0b00001110);
/// ```
pub fn circular_shl(byte: u8, count: u32) -> u8 {
    circular_sh_base(byte, count, u8::shl, u8::shr)
}

/// Returns right [*circularly shifted*](https://en.wikipedia.org/wiki/Circular_shift) number
/// # Arguments
/// * `byte` - number to work with
/// * `count` - number of positions to be shifted by
/// # Examples
/// ```
/// # use understanding_bitwise::circular_shr;
/// assert_eq!(circular_shr(0b10000011, 2), 0b11100000);
/// ```
pub fn circular_shr(byte: u8, count: u32) -> u8 {
    circular_sh_base(byte, count, u8::shr, u8::shl)
}

/// Returns the number that represents a sequence of consecutive ones
/// # Arguments
/// * `consecutive_ones_count` - count of consecutive ones in a sequence
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

/// Returns number of entries matching the consecutive ones sequence in the number
/// # Arguments
/// * `number` - number to work with
/// * `consecutive_ones_count` - count of consecutive ones in a sequence
/// # Examples
/// ```
/// # use understanding_bitwise::consecutive_ones_entries_count;
/// assert_eq!(consecutive_ones_entries_count(0b1001110, 2), Some(2));
/// assert_eq!(consecutive_ones_entries_count(0b1001110, 0), None);
/// assert_eq!(consecutive_ones_entries_count(0b1001110, 45), None);
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

/// Helper function, base function. Checks indexes equality and that indexes are in valid limits. If ok, performs the swap.
/// # Arguments
/// * `number` - number to work with
/// * `index1` - index of the bit to be swapped
/// * `index2` - index of the bit to be swapped
/// * `f` - the swap bit function
fn swap_bits_base<F>(number: u32, index1: u32, index2: u32, f: F) -> Option<u32>
where
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

/// Returns the number with the specific bits swapped.
/// # Arguments
/// * `number` - number to work with
/// * `index1` - index of the bit to be swapped
/// * `index2` - index of the bit to be swapped
/// # Examples
/// ```
/// # use understanding_bitwise::swap_bits;
/// assert_eq!(swap_bits(0b100011, 1, 4), Some(0b110001));
/// assert_eq!(swap_bits(0b100011, 300, 4), None);
/// ```
pub fn swap_bits(number: u32, index1: u32, index2: u32) -> Option<u32> {
    swap_bits_base(number, index1, index2, || {
        let mut min_index = index1;
        let mut max_index = index2;
        if min_index > max_index {
            mem::swap(&mut min_index, &mut max_index);
        }
        let distance = max_index - min_index;
        let min_index_number = 1 << min_index;
        let max_index_number = 1 << max_index;
        number & (number ^ min_index_number ^ max_index_number) | number >> distance & min_index_number | number << distance & max_index_number
    })
}

/// Returns the number with the specific bits swapped. Uses xor method
/// # Arguments
/// * `number` - number to work with
/// * `index1` - index of the bit to be swapped
/// * `index2` - index of the bit to be swapped
/// # Examples
/// ```
/// # use understanding_bitwise::swap_bits_xor;
/// assert_eq!(swap_bits_xor(0b100011, 1, 4), Some(0b110001));
/// assert_eq!(swap_bits_xor(0b100011, 300, 4), None);
/// ```
pub fn swap_bits_xor(number: u32, index1: u32, index2: u32) -> Option<u32> {
    swap_bits_base(number, index1, index2, || {
        let bit1 = (number >> index1) & 1;
        let bit2 = (number >> index2) & 1;
        let mut swapper = bit1 ^ bit2;
        swapper = swapper << index1 | swapper << index2;
        number ^ swapper
    })
}

/// Returns the number with specific bit removed. Bits before removed bit are left untouched. Bits after removed bit are shifted to the right by 1.
/// # Arguments
/// * `number` - number to work with
/// * `index` - index of the bit to be removed
/// # Examples
/// ```
/// # use understanding_bitwise::remove_bit;
/// assert_eq!(remove_bit(0b100011, 1), Some(0b10001));
/// assert_eq!(remove_bit(0b100011, 300), None);
/// ```
pub fn remove_bit(number: u32, index: u32) -> Option<u32> {
    if ! (0..u32::BITS).contains(&index) {
        return None;
    }

    let mut remover = number >> index + 1 ^ number >> index;
    remover <<= index;
    Some(number ^ remover)
}

/// Finds element that doesn't have duplicate. Other elements must have number of entries divisible by two. There must be only one unique element.
/// # Arguments
/// * `vals` - IntoIterator instance. You can pass a read-only reference of a collection
/// # Examples
/// ```
/// # use understanding_bitwise::find_unique;
/// assert_eq!(find_unique(&[45, 32, 777, 10, 45, 10, 32]), Some(777));
/// assert_eq!(find_unique(&[0u32; 0]), None);
/// ```
pub fn find_unique<'a, I, B: 'a>(vals: I) -> Option<B>
where
    I: IntoIterator<Item = &'a B>,
    B: BitXor<Output = B> + Copy {
    vals.into_iter().fold(None, |acc, &val| {
        acc.map_or_else(|| Some(val), |acc| Some(acc ^ val))
    })
}

/// This module contains tests
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

    fn general_test_binary_ones_count<F>(f: F)
    where
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

    fn general_test_hob<F>(f: F)
    where
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

        index = f(1 << u32::BITS - 1);
        assert_eq!(index.unwrap(), u32::BITS - 1);
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

    fn general_test_unset_bit<F>(f: F)
    where
        F: Fn(u32, u32) -> Option<u32> {
        assert_eq!(f(11, 1), Some(9));
        assert_eq!(f(9, 32), None);
        assert_eq!(f(0, 0), Some(0));
        assert_eq!(f(1, 0), Some(0));
        assert_eq!(f(263600, 18), Some(0b10110110000));
        assert_eq!(f(0b11010, 5), Some(0b11010));
    }

    #[test]
    fn test_unset_bit() {
        general_test_unset_bit(unset_bit);
    }

    #[test]
    fn test_unset_bit_xor() {
        general_test_unset_bit(unset_bit_xor);
    }

    #[test]
    fn test_unset_bit_bitwise_not() {
        general_test_unset_bit(unset_bit_bitwise_not);
    }

    #[test]
    fn test_set_unset_bit() {
        for unset_bit_f in [unset_bit, unset_bit_xor] {
            for i in (0..u32::BITS).step_by(2) {
                assert_eq!(unset_bit_f(set_bit(0b10101010101010101010101010101010, i).unwrap(), i).unwrap(), 0b10101010101010101010101010101010);
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

    fn general_test_swap_bits<F>(f: F)
    where
        F: Fn(u32, u32, u32) -> Option<u32> {
        assert_eq!(f(0b101, 2, 2), Some(0b101));
        assert_eq!(f(0b101, 228, 2), None);
        assert_eq!(f(0b101, 2, 282), None);
        assert_eq!(f(0b101, 282, 282), None);
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

    #[test]
    fn test_find_unique() {
        assert_eq!(find_unique(&[1, 0, 2, 2, 0, 228, 1]), Some(228));
        assert_eq!(find_unique(&vec![0b110, 0b10, 0b111, 0b110, 0b10, 0b111, 0b101]), Some(0b101));
        assert_eq!(find_unique(&[0u32; 0]), None);
        assert_eq!(find_unique(&vec![0u32; 0]), None);
        assert_eq!(find_unique(&Vec::<u32>::new()), None);
    }
}
