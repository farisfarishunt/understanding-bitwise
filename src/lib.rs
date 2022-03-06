use std::io::Write;
use std::collections::LinkedList;

#[derive(PartialEq, Debug)]
pub struct Overflow;

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
    let acc = |number| count += number & 1;
    process_binary_ones(number, acc);
    count
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
        
        let mut str = bin_rep_str(228);
        assert_eq!(str, String::from("11100100"));

        str = bin_rep_str(u32::MAX);
        assert_eq!(str, String::from("11111111111111111111111111111111"));
        
        str = bin_rep_str(u32::MIN);
        assert_eq!(str, String::from("0"));
        
        str = bin_rep_str(1);
        assert_eq!(str, String::from("1"));
    }
    
    #[test]
    fn test_binary_ones_count() {
        let mut count = binary_ones_count(228);
        assert_eq!(count, 4);
        
        count = binary_ones_count(u32::MAX);
        assert_eq!(count, 32);
        
        count = binary_ones_count(u32::MIN);
        assert_eq!(count, 0);
        
        count = binary_ones_count(1);
        assert_eq!(count, 1);
    }
}
