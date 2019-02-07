use std::collections::HashMap;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use yaml_rust::Yaml;

use log::*;

use crate::bits::*;
use crate::constants::*;

#[derive(Default, Copy, Clone, Debug)]
struct DispatchEntry {
    address: usize,
}

pub struct Dispatch {
    entries: [DispatchEntry; 128],
}

impl Default for Dispatch {
    fn default() -> Dispatch {
        Dispatch {
            entries: [DispatchEntry::default(); 128],
        }
    }
}

impl Dispatch {
    fn add_index(&mut self, key: &str, addr: usize) {
        if key != "default" {
            let index: &usize = OPCODE_MAP.get(key).expect("Invalid index");

            debug!("Indexing {} @ {} with address {:x}", key, index, addr);

            self.entries[*index].address = addr;
        }
    }
}

impl Dispatch {
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;

        for (index, entry) in self.entries.iter().enumerate() {
            writeln!(file, "{:x} {:x}", index, entry.address)?;
        }

        Ok(())
    }
}

/// Microcode
/// ---------
/// Bytes
/// 22 - 23 	PCSource
/// 21 	PCWrite
/// 20 	PCWriteCond
/// 16-19 	ALUop
/// 15 	ALUSrcA
/// 13 - 14 	ALUSrcB
/// 12 	IRWrite
/// 11 	IorD
/// 10 	MemRead
/// 9 	MemWrite
/// 8 	MemToReg
/// 7 	RegDest
/// 6 	RegWrite
/// 5 	Halt
/// 4 	Error
/// 3 	unused
/// 2 	unused
/// 1 	dispatch
/// 0 	next
///
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Microcode {
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

/*
impl From<HashMap<String, String>> for Microcode {
    fn from(map: HashMap<String, String>) -> Microcode {
        Microcode::default()
    }
}
*/

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

        // 3 - unused
        // 2 - unused

        set_bit(&mut value, 1, self.dispatch);
        set_bit(&mut value, 0, self.next);

        value
    }
}

impl From<&Yaml> for Microcode {
    fn from(hash: &Yaml) -> Self {
        let hash = hash.clone().into_hash().unwrap();

        let mut microcode = Microcode::default();

        for key in hash.keys() {
            if let Yaml::String(ref key_str) = key {
                if !VALID_BITS.contains(&key_str.as_str()) {
                    warn!("Undefined setting {}", key_str);
                }
            } else {
                warn!("Unexpected type. Expected yaml String instead of {:?}", key);
            }
        }

        for (key, value) in hash {
            if let Yaml::String(ref key_str) = key {
                set_flag_bits(key_str, "pc-source", &value, &mut microcode.pc_source, 2);
                set_flag_bits(key_str, "alu-op", &value, &mut microcode.alu_op, 4);
                set_flag_bits(key_str, "alu-src-b", &value, &mut microcode.alu_src_b, 2);

                set_flag_if_true(key_str, "pc-write", &value, &mut microcode.pc_write);
                set_flag_if_true(key_str, "pc-write-cond", &value, &mut microcode.pc_write_cond);

                set_flag_if_true(key_str, "alu-src-a", &value, &mut microcode.alu_src_a);
                set_flag_if_true(key_str, "ir-write", &value, &mut microcode.ir_write);
                set_flag_if_true(key_str, "i-or-d", &value, &mut microcode.i_or_d);
                set_flag_if_true(key_str, "mem-read", &value, &mut microcode.mem_read);
                set_flag_if_true(key_str, "mem-write", &value, &mut microcode.mem_write);
                set_flag_if_true(key_str, "mem-to-reg", &value, &mut microcode.mem_to_reg);
                set_flag_if_true(key_str, "reg-dest", &value, &mut microcode.reg_dest);
                set_flag_if_true(key_str, "reg-write", &value, &mut microcode.reg_write);

                set_flag_if_true(key_str, "halt", &value, &mut microcode.halt);
                set_flag_if_true(key_str, "error", &value, &mut microcode.error);
            }
        }

        microcode
    }
}

pub fn write_microcode<P: AsRef<Path>>(
    path: P,
    mut input: HashMap<String, Vec<Microcode>>,
) -> io::Result<()> {
    let mut file = File::create(path)?;

    let mut addr = 0usize;

    for (_key, microcode) in input.iter_mut() {
        // Add the "next" directive to all microcode instructions
        for code in microcode.iter_mut() {
            code.next = true;
        }

        // Remove the "next" directive from the last microcode instruction
        if let Some(ref mut last) = microcode.iter_mut().last() {
            last.next = false;
            last.dispatch = true;
        }
    }


    if let Some(default) = input.remove("default") {
            
        for (index, code) in default.iter().cloned().enumerate() {
            
            debug!("{:#?}", code);
            
            let byte_repr: u32 = code.into();

            write!(file, "{:x} {:x}", addr, byte_repr)?;

            if index == 0 {
                writeln!(file, " # default segment")?;
            } else {
                writeln!(file)?;
            }

            addr += 1;
        }
    }

    for (key, microcode) in input {

        // Output the microcode
        for (index, code) in microcode.iter().cloned().enumerate() {
            
            debug!("{:#?}", code);
            
            let byte_repr: u32 = code.into();

            write!(file, "{:x} {:x}", addr, byte_repr)?;

            if index == 0 {
                writeln!(file, " # {} segment", key)?;
            } else {
                writeln!(file)?;
            }

            addr += 1;
        }
    }

    Ok(())
}

pub fn generate_dispatch(input: HashMap<String, Vec<Microcode>>) -> Dispatch {
    let mut dispatch = Dispatch::default();

    let mut input = input.clone();

    let mut addr = 0usize;

    if let Some(default) = input.remove("default") {
        for _ in default {
            addr += 1;
        }
    }

    for (key, microcode) in input {
        dispatch.add_index(&key, addr);

        for _ in microcode {
            addr += 1;
        }
    }

    dispatch
}

pub fn collapse_instructions(
    instructions: HashMap<String, Vec<Yaml>>,
) -> HashMap<String, Vec<Microcode>> {
    instructions
        .iter()
        .map(|(key, value)| {
            let microcode: Vec<Microcode> = value.iter().map(Microcode::from).collect();

            (key.clone(), microcode)
        })
        .collect()
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
