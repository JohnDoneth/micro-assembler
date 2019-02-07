# Micro Assembler

Project to ease work while working on [this homework assignment](https://cis.gvsu.edu/~kurmasz/Teaching/Courses/W19/CIS451/Homework/MicrocodeForPHMultiCycleCPU/) for course CIS 451 at GVSU.

[Downloads](https://github.com/JohnDoneth/micro-assembler/releases/latest)

```man
Micro Assembler 0.2.0
John Doneth <doneth7@gmail.com>
Tool to aid in compiling microcode for CIS 451 homework

USAGE:
    micro-assembler [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dispatch-output <dispatch>      Sets the file to output dispatch to [default: dispatch1]
    -i, --input <input>                   Sets the input file to use [default: input.yaml]
    -m, --microcode-output <microcode>    Sets the file to output microcode to [default: microcode]
    -v <verbosity>                        Sets the level of verbosity [default: warn]  [possible values: disabled, info,
                                          warn, debug, error, trace]
```

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
... omitted ...
0x5d 0x0
0x5e 0x0
0x5f 0x0
0x60 0x3 <-- Points to the address 0x3 in the microcode for the 0x60 opcode "add".
0x61 0x0
0x62 0x0
0x63 0x0
... omitted ...
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
