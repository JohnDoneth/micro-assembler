use lazy_static::lazy_static;
use std::collections::HashMap;
use sugar::*;

lazy_static! {
    pub static ref OPCODE_MAP: HashMap<&'static str, usize> = {
        hashmap! {
            "add"  => 0x20 + 64,
            "addi" => 0x08,
            "and"  => 0x24 + 64,
            "andi" => 0x0C,
            "beq"  => 0x04,
            "bne"  => 0x05,
            "halt" => 0x20,
            "j"    => 0x02,
            "jal"  => 0x03,
            "jr"   => 0x08 + 64,
            "lw"   => 0x23,
            "lui"  => 0x0F,
            "nor"  => 0x27 + 64,
            "or"   => 0x25 + 64,
            "ori"  => 0x0D,
            "slt"  => 0x2A + 64,
            "slti" => 0x0A,
            "sw"   => 0x2B,
            "sub"  => 0x22 + 64,
        }
    };
}

pub const VALID_BITS: &[&str] = &[
    "pc-source",
    "pc-write",
    "pc-write-cond",
    "alu-op",
    "alu-src-a",
    "alu-src-b",
    "ir-write",
    "i-or-d",
    "mem-read",
    "mem-write",
    "mem-to-reg",
    "reg-dest",
    "reg-write",
    "halt",
    "error",
    "slt",
    "slti",
];

pub const VALID_OPERATIONS: &[&str] = &[
    "add", "addi", "and", "andi", "beq", "bne", "halt", "j", "jal", "jr", "lw", "lui", "nor", "or",
    "ori", "slt", "slti", "sw", "sub",
];
