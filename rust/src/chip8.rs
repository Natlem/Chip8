struct Chip8 {
    opcode      : u16,
    memory      : [u8; 4096],
    regV        : [u8; 16],
    indexReg    : u16,
    progCount   : u16,
    gfx         : [u8; 64 * 32],
    delay_timer : u8,
    sound_timer : u8,
    stack       : [u16; 16],
    sp          : u16,
    key         : [u8; 16],
}

