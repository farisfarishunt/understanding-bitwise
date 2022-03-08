use std::io::Write;
use std::collections::LinkedList;

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

pub fn invert_bit(number: u32, index: u32) -> Option<u32> {
    manipulate_bit(index, ||
        number ^ 1 << index
    )
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
    
    #[test]
    fn test_reset_bit() {
        assert_eq!(reset_bit(11, 1), Some(9));
        assert_eq!(reset_bit(9, 32), None);
        assert_eq!(reset_bit(0, 0), Some(0));
        assert_eq!(reset_bit(1, 0), Some(0));
        assert_eq!(reset_bit(263600, 18), Some(0b10110110000));
        assert_eq!(reset_bit(0b11010, 5), Some(0b11010));
    }
    
    #[test]
    fn test_set_reset_bit() {
        for i in (0..u32::BITS).step_by(2) {
            assert_eq!(reset_bit(set_bit(0b10101010101010101010101010101010, i).unwrap(), i).unwrap(), 0b10101010101010101010101010101010);
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
}
