# Micro Assembler

![](https://img.shields.io/travis/JohnDoneth/micro-assembler/master.svg?style=flat-square) ![](https://img.shields.io/appveyor/ci/JohnDoneth/micro-assembler/master.svg?style=flat-square) ![](https://img.shields.io/github/license/JohnDoneth/micro-assembler.svg?style=flat-square) ![https://github.com/JohnDoneth/micro-assembler/releases/latest](https://img.shields.io/badge/Supported%20Platforms-Windows%2C%20Linux%2C%20Mac%20OS%20X-blue.svg?style=flat-square)



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
5 0
6 0
7 0
8 3 <-- Points to the address 0x3 in the microcode for the 0x08 opcode "addi".
9 0
a 0
b 0
... omitted ...
```

Microcode

```hex
0 220001 # add segment
1 400001
2 800202
3 220001 # addi segment
4 400001
5 800202
```

---

Tips:

There's a special "instruction" called "default" that will always be placed at 0x0 in the microcode file, it might come in handy.

```yaml
default:
  -  # Some instructions here
  -  # ...
  -  # ...
```
