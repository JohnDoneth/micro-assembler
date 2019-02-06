pub fn set_bit(value: &mut u32, n: usize, b: bool) {
    if b {
        (*value) |= 1 << n;
    } else {
        (*value) &= !(1 << n);
    }
}

pub fn set_bit_range(value: &mut u32, start: usize, end: usize, new_value: u8) {
    let mask = (1 << (1 + end - start)) - 1;

    //println!("mask    {:024b}", mask);

    //println!("value   {:024b}", new_value);

    let new_value = (new_value as u32) & mask;

    let new_value = new_value << end;
    let new_value = new_value >> (end - start);

    //println!("s mask  {:024b}", (mask << end) >> (end - start));

    //println!("shifted {:024b}", new_value);

    //println!("before  {:024b}", value);
    (*value) |= new_value;

    //println!("       {:024b}", 0b110000000000000000000000);
    //println!("after   {:024b}\n", value);
}

pub fn is_bit_set(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn extract_bit_range(value: u32, start: usize, end: usize) -> u8 {
    assert!(start < end);

    let k = end - start + 1;

    let mask = (1 << k) - 1;

    let mask = mask << start + 1;
    let mask = mask >> 1;

    //println!("mask  {:024b}", mask);

    //println!("value {:024b}", value);

    let res = (mask & value) as u32;

    //println!("res   {:024b}\n", res);

    let shifted_res = res >> start;

    //println!("sres  {:024b}\n", shifted_res);

    return shifted_res as u8;
}

#[test]
fn test_extract_bit_range() {
    let bits = 0b0110 as u32;
    let extracted = extract_bit_range(bits, 2, 3);
    assert!(extracted == 0b01);

    let bits = 0b0110 as u32;
    let extracted = extract_bit_range(bits, 0, 3);
    assert!(extracted == 0b0110);

    let bits = 0b0110 as u32;
    let extracted = extract_bit_range(bits, 0, 1);
    assert!(extracted == 0b10);

    let bits = 0b0110 as u32;
    let extracted = extract_bit_range(bits, 1, 2);
    assert!(extracted == 0b11);
}
