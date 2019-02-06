# Micro Assembler

Project to ease work while working on [this homework assignment](https://cis.gvsu.edu/~kurmasz/Teaching/Courses/W19/CIS451/Homework/MicrocodeForPHMultiCycleCPU/) for course CIS 451 at GVSU.

This project is able to compile the following yaml to microcode with an associated dispatch file with comments.

```yaml
add:
  # The first micro instruction
  - pc-source: 0
    pc-write: true
    alu-op: 2

  # The second micro instruction
  - pc-source: 1

  # The third micro instruction
  - pc-source: 2
    mem-write: true

addi:
  # The first micro instruction
  - pc-source: 0
    pc-write: true
    alu-op: 2

  # The second micro instruction
  - pc-source: 1

  # The third micro instruction
  - pc-source: 2
    mem-write: true

and:
andi:
beq:
# etc ...
```

The above yaml file produces the following dispatch and microcode files.

Dispatch

```hex
... omited ...
0x5d 0x0
0x5e 0x0
0x5f 0x0
0x60 0x3 <-- Points to the address 0x3 in the microcode for the 0x60 opcode "add".
0x61 0x0
0x62 0x0
0x63 0x0
... omited ...
```

Microcode

```hex
0x0 0x220000 # addi segment
0x1 0x400000
0x2 0x800200
0x3 0x220000 # add segment
0x4 0x400000
0x5 0x800200
```
