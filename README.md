# RARM
**A W.I.P ARM Disassembler written in rust :)**

##### Test Output:
```
❯ cargo r test.bin 40
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rarm test.bin 40`
["target/debug/rarm", "test.bin", "40"]
Found ADRP:                    insn: 0x90000000 mask: 0x9F000000 insn & mask: 0x90000000
Found ADD:                     insn: 0x91000000 mask: 0x7F800000 insn & mask: 0x11000000
Found LDR (immediate):         insn: 0x580017C1 mask: 0xBFE00C00 insn & mask: 0x18000400
Found BL:                      insn: 0x94006D7D mask: 0xFC000000 insn & mask: 0x94000000
Found CMP:                     insn: 0xEB00003F mask: 0x7F80001F insn & mask: 0x6B00001F
Found B.cond:                  insn: 0x54000260 mask: 0xFF000010 insn & mask: 0x54000000
Found LDR (literal):           insn: 0x5800019E mask: 0xBF000000 insn & mask: 0x18000000
Found LDR (immediate):         insn: 0x58001762 mask: 0xBFE00C00 insn & mask: 0x18000400
Found SUB (immediate):         insn: 0xCB010042 mask: 0x7F800000 insn & mask: 0x4B000000
Found MOV (register):          insn: 0xAA0003E5 mask: 0x7FE0FFE0 insn & mask: 0x2A0003E0
Found MOV (register):          insn: 0xAA0203E6 mask: 0x7FE0FFE0 insn & mask: 0x2A0003E0
Found MOV (register):          insn: 0xAA0103E7 mask: 0x7FE0FFE0 insn & mask: 0x2A0003E0
Found LDP:                     insn: 0xA8C11003 mask: 0x79C00000 insn & mask: 0x28C00000
Found STP:                     insn: 0xA8811023 mask: 0x7FC00000 insn & mask: 0x28800000
Found SUBS (immediate):        insn: 0xF1004042 mask: 0x7F800000 insn & mask: 0x71000000
Found B.cond:                  insn: 0x54FFFFA1 mask: 0xFF000010 insn & mask: 0x54000000
Found RET:                     insn: 0xD65F03C0 mask: 0xFFFFFC1F insn & mask: 0xD65F0000
Found NOP:                     insn: 0xD503201F mask: 0xFFFFFFFF insn & mask: 0xD503201F
Found UNKNOWN
Found UNKNOWN
Found STP:                     insn: 0xA8817CBF mask: 0x7FC00000 insn & mask: 0x28800000
Found SUBS (immediate):        insn: 0xF10040C6 mask: 0x7F800000 insn & mask: 0x71000000
Found B.cond:                  insn: 0x54FFFFC1 mask: 0xFF000010 insn & mask: 0x54000000
Found BR:                      insn: 0xD61F00E0 mask: 0xFFFFFC1F insn & mask: 0xD61F0000
Found MSR (immediate):         insn: 0xD5034FDF mask: 0xFFF8F01F insn & mask: 0xD500401F
Found ADRP:                    insn: 0xB000000A mask: 0x9F000000 insn & mask: 0x90000000
Found ADD:                     insn: 0x9100014A mask: 0x7F800000 insn & mask: 0x11000000
Found MSR (register):          insn: 0xD518C00A mask: 0xFFF80000 insn & mask: 0xD5180000
Found LDR (immediate):         insn: 0x5800168A mask: 0xBFE00C00 insn & mask: 0x18000400
Found LDR (immediate):         insn: 0x580016AB mask: 0xBFE00C00 insn & mask: 0x18000400
Found ADD (extended register): insn: 0x8B0A016B mask: 0x7FE00000 insn & mask: 0xB000000
Found MOV (wide immediate):    insn: 0xD280000C mask: 0x7F800000 insn & mask: 0x52800000
Found STP:                     insn: 0xA881314C mask: 0x7FC00000 insn & mask: 0x28800000
Found CMP:                     insn: 0xEB0B015F mask: 0x7F80001F insn & mask: 0x6B00001F
Found B.cond:                  insn: 0x54FFFFC1 mask: 0xFF000010 insn & mask: 0x54000000
Found LDR (immediate):         insn: 0x580016AA mask: 0xBFE00C00 insn & mask: 0x18000400
Found LDR (immediate):         insn: 0x580016CB mask: 0xBFE00C00 insn & mask: 0x18000400
Found ADD (extended register): insn: 0x8B0A016B mask: 0x7FE00000 insn & mask: 0xB000000
Found MOV (wide immediate):    insn: 0xD280000C mask: 0x7F800000 insn & mask: 0x52800000
Found STP:                     insn: 0xA881314C mask: 0x7FC00000 insn & mask: 0x28800000
```