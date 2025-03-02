// Set graphics mode: use 0x101 for 640x480, 256 colors or 0x13 for standard mode.
pub const GRAPHICS_MODE: u16 = 0x13; // 0x101; 

pub const SCREEN_WIDTH: usize = match GRAPHICS_MODE {
    0x13 => 320,
    0x101 => 640,
    _ => 0,
};

pub const SCREEN_HEIGHT: usize = match GRAPHICS_MODE {
    0x13 => 200,
    0x101 => 480,
    _ => 0,
};

// For mode 13h, TRUE_WHITE is 0x3F; for 256-color mode, choose 0xFF (adjust as needed).
pub const TRUE_WHITE: u8 = match GRAPHICS_MODE {
    0x13 => 0x3F,
    0x101 => 0xFF,
    _ => 0,
};

pub const FRAMEBUFFER_ADDRESS: usize = match GRAPHICS_MODE {
    0x13 => 0xA0000,  // Mode 13h uses VGA memory
    0x101 => 0xA0000, // Temporarily use mode 13h framebuffer address to avoid crash
    _ => 0,
};