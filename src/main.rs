use std::{ fs::File, env, str::FromStr, io::prelude::*, collections::HashMap };

fn main() {
    let args: Vec<String> = env::args().collect(); // Get user input cli args
    println!("{:?}", args); // Print our args
    let mut file = File::open(&*args[1]).expect("err file"); // Open user specified file, should add arg sanitizing
    let mut buffer = Vec::new(); // Open a Vec for saving binary file's bytes
    let _res = file.read_to_end(&mut buffer); // Read the file to our buffer as a Vec<u8>
    let instruction_buffer: &mut [u32] = bytemuck::cast_slice_mut(buffer.as_mut()); // Since instructions are interpreted as 4 bytes, cast the Vec<u8> to 32 bits instead (Vec<u32>)
    let mut instruction_names: HashMap<u32, String> =  HashMap::new(); // Hashmap for name + result
    let mut instructions: HashMap<u32, Vec<u32>> =  HashMap::new(); // Hashmap for mask + result vec

/*****************************************************************************
 *  The ADD instruction
 *  Mask:       0x7F800000(0b01111111100000000000000000000000)
 *  Result:     0x11000000(0b00010001000000000000000000000000)
 *  match if instruction(?) & mask(0x7F800000) == result(0x11000000)        */
    instruction_names.insert(0x11000000, "ADD".to_owned());
    instructions.entry(0x7F800000)
        .or_default() // Fallback
        .push(0x11000000);
/*****************************************************************************
 *  The ADD (extended register) instruction
 *  Mask:       0x7FE00000(0b01111111111000000000000000000000)
 *  Result:     0x0B000000(0b00001011000000000000000000000000)
 *  match if instruction(?) & mask(0x7FE00000) == result(0x0B000000)        */
    instruction_names.insert(0x0B000000, "ADD (extended register)".to_owned());
    instructions.entry(0x7FE00000)
        .or_default() // Fallback
        .push(0x0B000000);
/*****************************************************************************
 *  The ADRP instruction
 *  Mask:       0x9F000000(0b10011111000000000000000000000000)
 *  Result:     0x90000000(0b10010000000000000000000000000000)
 *  match if instruction(?) & mask(0x9F000000) == result(0x90000000)        */
    instruction_names.insert(0x90000000, "ADRP".to_owned());
    instructions.entry(0x9F000000)
        .or_default() // Fallback
        .push(0x90000000);
/*****************************************************************************
 *  The ADR instruction
 *  Mask:       0x9F000000(0b10011111000000000000000000000000)
 *  Result:     0x10000000(0b00010000000000000000000000000000)
 *  match if instruction(?) & mask(0x9F000000) == result(0x10000000)        */
    instruction_names.insert(0x10000000, "ADR".to_owned());
    instructions.entry(0x9F000000)
        .or_default() // Fallback
        .push(0x10000000);
/*****************************************************************************
 *  The BL instruction
 *  Mask:       0xFC000000(0b11111100000000000000000000000000)
 *  Result:     0x94000000(0b10010100000000000000000000000000)
 *  match if instruction(?) & mask(0xFC000000) == result(0x94000000)        */
    instruction_names.insert(0x94000000, "BL".to_owned());
    instructions.entry(0xFC000000)
        .or_default() // Fallback
        .push(0x94000000);
/*****************************************************************************
 *  The B instruction
 *  Mask:       0xFC000000(0b11111100000000000000000000000000)
 *  Result:     0x94000000(0b10010100000000000000000000000000)
 *  match if instruction(?) & mask(0xFC000000) == result(0x94000000)        */
    instruction_names.insert(0x14000000, "B".to_owned());
    instructions.entry(0xFC000000)
        .or_default() // Fallback
        .push(0x14000000);
/*****************************************************************************
 *  The B.cond instruction
 *  Mask:       0xFF000010(0b11111111000000000000000000010000)
 *  Result:     0x54000000(0b01010100000000000000000000000000)
 *  match if instruction(?) & mask(0xFF000010) == result(0x54000000)        */
    instruction_names.insert(0x54000000, "B.cond".to_owned());
    instructions.entry(0xFF000010)
        .or_default() // Fallback
        .push(0x54000000);
/*****************************************************************************
 *  The BR instruction
 *  Mask:       0xFFFFFC1F(0b11111111111111111111110000011111)
 *  Result:     0xD61F0000(0b11010110000111110000000000000000)
 *  match if instruction(?) & mask(0xFFFFFC1F) == result(0xD61F0000)        */
    instruction_names.insert(0xD61F0000, "BR".to_owned());
    instructions.entry(0xFFFFFC1F)
        .or_default() // Fallback
        .push(0xD61F0000);
/*****************************************************************************
 *  The BLR instruction
 *  Mask:       0xFFFFFC1F(0b11111111111111111111110000011111)
 *  Result:     0xD63F0000(0b11010110001111110000000000000000)
 *  match if instruction(?) & mask(0xFFFFFC1F) == result(0xD63F0000)        */
    instruction_names.insert(0xD63F0000, "BLR".to_owned());
    instructions.entry(0xFFFFFC1F)
        .or_default() // Fallback
        .push(0xD63F0000);
/*****************************************************************************
 *  The CMP instruction
 *  Mask:       0x7F80001F(0b01111111100000000000000000011111)
 *  Result:     0x6B00001F(0b01101011000000000000000000011111)
 *  match if instruction(?) & mask(0x7F80001F) == result(0x6B00001F)        */
    instruction_names.insert(0x6B00001F, "CMP".to_owned());
    instructions.entry(0x7F80001F)
        .or_default() // Fallback
        .push(0x6B00001F);
/*****************************************************************************
 *  The LDP instruction
 *  Mask:       0x79C00000(0b01111001110000000000000000000000)
 *  Result:     0x28C00000(0b00101000110000000000000000000000)
 *  match if instruction(?) & mask(0x79C00000) == result(0x28C00000)        */
    instruction_names.insert(0x28C00000, "LDP".to_owned());
    instructions.entry(0x79C00000)
        .or_default() // Fallback
        .push(0x28C00000);
/*****************************************************************************
 *  The LDR (immediate) instruction
 *  Mask:       0xBFE00C00(0b10111111111000000000110000000000)
 *  Result:     0x18000400(0b00011000000000000000010000000000)
 *  match if instruction(?) & mask(0xBFE00C00) == result(0x18000400)        */
    instruction_names.insert(0x18000400, "LDR (immediate)".to_owned());
    instructions.entry(0xBFE00C00)
        .or_default() // Fallback
        .push(0x18000400);
/*****************************************************************************
 *  The LDR (literal) instruction
 *  Mask:       0xBF000000(0b10111111000000000000000000000000)
 *  Result:     0x18000000(0b00011000000000000000000000000000)
 *  match if instruction(?) & mask(0xBF000000) == result(0x18000000)        */
    instruction_names.insert(0x18000000, "LDR (literal)".to_owned());
    instructions.entry(0xBF000000)
        .or_default() // Fallback
        .push(0x18000000);
/*****************************************************************************
 *  The MOV (register) instruction
 *  Mask:       0x7FE0FFE0(0b01111111111000001111111111100000)
 *  Result:     0x2A0003E0(0b00101010000000000000001111100000)
 *  match if instruction(?) & mask(0x7FE0FFE0) == result(0x2A0003E0)        */
    instruction_names.insert(0x2A0003E0, "MOV (register)".to_owned());
    instructions.entry(0x7FE0FFE0)
        .or_default() // Fallback
        .push(0x2A0003E0);
/*****************************************************************************
 *  The MOV (wide immediate) instruction
 *  Mask:       0x7F800000(0b01111111100000000000000000000000)
 *  Result:     0x52800000(0b01010010100000000000000000000000)
 *  match if instruction(?) & mask(0x7F800000) == result(0x52800000)        */
    instruction_names.insert(0x52800000, "MOV (wide immediate)".to_owned());
    instructions.entry(0x7F800000)
        .or_default() // Fallback
        .push(0x52800000);
/*****************************************************************************
 *  The MSR (immediate) instruction
 *  Mask:       0xFFF8F01F(0b11111111111110001111000000011111)
 *  Result:     0xD500401F(0b11010101000000000100000000011111)
 *  match if instruction(?) & mask(0xFFF8F01F) == result(0xD500401F)        */
    instruction_names.insert(0xD500401F, "MSR (immediate)".to_owned());
    instructions.entry(0xFFF8F01F)
        .or_default() // Fallback
        .push(0xD500401F);
/*****************************************************************************
 *  The MSR (register) instruction
 *  Mask:       0xFFF80000(0b11111111111110000000000000000000)
 *  Result:     0xD5180000(0b11010101000110000000000000000000)
 *  match if instruction(?) & mask(0xFFF80000) == result(0xD5180000)        */
    instruction_names.insert(0xD5180000, "MSR (register)".to_owned());
    instructions.entry(0xFFF80000)
        .or_default() // Fallback
        .push(0xD5180000);
/*****************************************************************************
 *  The NOP instruction
 *  Mask:       0xFFFFFFFF(0b11111111111111111111111111111111)
 *  Result:     0xD503201F(0b11010101000000110010000000011111)
 *  match if instruction(?) & mask(0xFFFFFFFF) == result(0xD503201F)        */
    instruction_names.insert(0xD503201F, "NOP".to_owned());
    instructions.entry(0xFFFFFFFF)
        .or_default() // Fallback
        .push(0xD503201F);
/*****************************************************************************
 *  The RET instruction
 *  Mask:       0xFFFFFC1F(0b11111111111111111111110000011111)
 *  Result:     0xD65F0000(0b11010110010111110000000000000000)
 *  match if instruction(?) & mask(0xFFFFFC1F) == result(0xD65F0000)        */
    instruction_names.insert(0xD65F0000, "RET".to_owned());
    instructions.entry(0xFFFFFC1F)
        .or_default() // Fallback
        .push(0xD65F0000);
/*****************************************************************************
 *  The STP instruction
 *  Mask:       0x7FC00000(0b01111111110000000000000000000000)
 *  Result:     0x28800000(0b00101000100000000000000000000000)
 *  match if instruction(?) & mask(0x7FC00000) == result(0x28800000)        */
    instruction_names.insert(0x28800000, "STP".to_owned());
    instructions.entry(0x7FC00000)
        .or_default() // Fallback
        .push(0x28800000);
/*****************************************************************************
 *  The SUB (immediate) instruction
 *  Mask:       0x7F800000(0b01111111100000000000000000000000)
 *  Result:     0x4B000000(0b01001011000000000000000000000000)
 *  match if instruction(?) & mask(0x7F800000) == result(0x4B000000)        */
    instruction_names.insert(0x4B000000, "SUB (immediate)".to_owned());
    instructions.entry(0x7F800000)
        .or_default() // Fallback
        .push(0x4B000000);
/*****************************************************************************
 *  The SUBS (immediate) instruction
 *  Mask:       0x7F800000(0b01111111100000000000000000000000)
 *  Result:     0x71000000(0b01110001000000000000000000000000)
 *  match if instruction(?) & mask(0x7F800000) == result(0x71000000)        */
    instruction_names.insert(0x71000000, "SUBS (immediate)".to_owned());
    instructions.entry(0x7F800000)
        .or_default() // Fallback
        .push(0x71000000);
/*****************************************************************************
*****************************************************************************/

    let mut idx: i32 = 0; // Index for tracking instruction count

    for insn in instruction_buffer { // Loop instructions in specified binary
        if idx >= i32::from_str(&*args[2]).unwrap() { // Make sure to break if index is at specified limit
            break;
        }
        let mut br: bool = false; // Boolean for keeping track of the instruction being matched.
        for (key, val) in &instructions { // Loop our instructions hashmap
            for val1 in val { // Loop the hashmap val's Vec
                if *insn & key == *val1 { // Match instruction with our saved masks to our saved result
                    let name = &instruction_names.get(&val1).unwrap(); // Get the instruction String name
                    let cnt = 23 - &name.chars().count(); // Count for spacing(will crash if negative so this could be improved)
                    println!("Found {}:{} insn: {:#08X} mask: {:#08X} insn & mask: {:#08X}", &name, String::from_utf8(vec![b' '; cnt]).unwrap(), *insn, key, val1); // Print everything baby
                    br = true; // Instruction was matched so mark as true
                }
            }
            if br {
                break; // Break since we already matched the instruction otherwise keep looping
            }
        }
        if !br {
            println!("Found UNKNOWN"); // Because br was never set to true the instruction was never matched
        }
        idx += 1; // Add to our index
    }
}
