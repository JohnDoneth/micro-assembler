
use yaml_rust::{YamlLoader};
use std::collections::HashMap;

#[derive(Default, Copy, Clone)]
struct DispatchEntry {
    address: u32,
}

struct Dispatch {
    entries: [DispatchEntry; 128],
}

impl Default for Dispatch {
    fn default() -> Dispatch {
        Dispatch {
            entries: [DispatchEntry::default(); 128]
        }
    }
}

use std::fs::File;
use std::io::Write;
use std::io;
use std::path::Path;

impl Dispatch {

    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        
        for (index, entry) in self.entries.iter().enumerate() {
            write!(file, "{} {:x}", index, entry.address)?;
        }

        Ok(())  
    }

}

/*
22 - 23 	PCSource
21 	PCWrite
20 	PCWriteCond
    
16-19 	ALUop
    
15 	ALUSrcA
13 - 14 	ALUSrcB
12 	IRWrite
    
11 	IorD
10 	MemRead
9 	MemWrite
8 	MemToReg
    
7 	RegDest
6 	RegWrite
5 	Halt
4 	Error
    
3 	unused
2 	unused
1 	dispatch
0 	next
*/
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct Microcode {
    pc_source: u8, // 2 bits
    pc_write: bool,
    pc_write_cond: bool,

    alu_op: u8, // 4 bits

    alu_src_a: bool,
    alu_src_b: u8, // 2 bits

    ir_write: bool,
    i_or_d: bool,
    mem_read: bool,
    mem_write: bool,
    mem_to_reg: bool,

    reg_dest: bool,
    reg_write: bool,

    halt: bool,
    error: bool,

    dispatch: bool,
    next: bool,
}


impl From<HashMap<String, String>> for Microcode {

    fn from(map: HashMap<String, String>) -> Microcode {
        Microcode::default()
    }

}

fn set_bit(value: &mut u32, n: usize, b: bool) {
    if b {
        (*value) |= 1 << n;
    } else {
        (*value) &= !(1 << n);
    }
}

fn set_bit_range(value: &mut u32, start: usize, end: usize, new_value: u8) {
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

fn is_bit_set(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn extract_bit_range(value: u32, start: usize, end: usize) -> u8 {
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

impl From<u32> for Microcode {
    fn from(src: u32) -> Microcode {
        Microcode {
            pc_source: extract_bit_range(src, 22, 23), // 2 bits
            pc_write: is_bit_set(src, 21),             // 1 bit
            pc_write_cond: is_bit_set(src, 20),

            alu_op: extract_bit_range(src, 16, 19), // 4 bits

            alu_src_a: is_bit_set(src, 15),
            alu_src_b: extract_bit_range(src, 13, 14), // 2 bits

            ir_write: is_bit_set(src, 12),
            i_or_d: is_bit_set(src, 11),
            mem_read: is_bit_set(src, 10),
            mem_write: is_bit_set(src, 9),
            mem_to_reg: is_bit_set(src, 8),

            reg_dest: is_bit_set(src, 7),
            reg_write: is_bit_set(src, 6),

            halt: is_bit_set(src, 5),
            error: is_bit_set(src, 4),

            // 3 - unused
            // 2 - unused

            dispatch: is_bit_set(src, 1),
            next: is_bit_set(src, 0),
        }
    }
}

impl Into<u32> for Microcode {
    fn into(self) -> u32 {
        let mut value: u32 = 0;

        set_bit_range(&mut value, 22, 23, self.pc_source);
        set_bit(&mut value, 21, self.pc_write);
        set_bit(&mut value, 20, self.pc_write_cond);
        set_bit_range(&mut value, 16, 19, self.alu_op);
        set_bit(&mut value, 15, self.alu_src_a);
        set_bit_range(&mut value, 13, 14, self.alu_src_b);
        set_bit(&mut value, 12, self.ir_write);
        set_bit(&mut value, 11, self.i_or_d);
        set_bit(&mut value, 10, self.mem_read);
        set_bit(&mut value, 9, self.mem_write);
        set_bit(&mut value, 8, self.mem_to_reg);
        set_bit(&mut value, 7, self.reg_dest);
        set_bit(&mut value, 6, self.reg_write);
        set_bit(&mut value, 5, self.halt);
        set_bit(&mut value, 4, self.error);
        set_bit(&mut value, 1, self.dispatch);
        set_bit(&mut value, 0, self.next);

        value
    }
}

fn main() {

    let string = std::fs::read_to_string("input.yaml").unwrap();

    let input = YamlLoader::load_from_str(&string).unwrap();

    println!("{:#?}", input);

    //println!("{:#?}", Microcode::from(0b001000100011010000000001));
}

#[test]
fn test_output_pc_source() {
    let mcode = Microcode {
        pc_source: 3,
        ..Default::default()
    };

    let bit_repr: u32 = mcode.into();

    println!("{:024b}", bit_repr);

    assert!(is_bit_set(bit_repr, 22));
    assert!(is_bit_set(bit_repr, 23));
}

// 0 223401 # 001000100011010000000001b
#[test]
fn example1() {
    let original = 0b001000100011010000000001;
    let mcode: Microcode = original.into();
    
    let byte_repr: u32 = mcode.into();
    
    assert_eq!(byte_repr, original);
}

// 1 026002 # 000000100110000000000010b
#[test]
fn example2() {
    let original = 0b100110000000000010;
    let mcode1: Microcode = original.into();
    
    let byte_repr: u32 = mcode1.clone().into();
    let mcode2: Microcode = byte_repr.into();

    assert_eq!(byte_repr, original);
}
