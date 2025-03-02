use x86_64::instructions::port::Port;

/// Switches the display to 320×200 graphics mode (mode 13h).
/// Note: The actual register programming is hardware‐dependent;
/// this stub represents where such code would go.
pub fn switch_to_graphics_mode() {
    unsafe {
        // Write Miscellaneous Output register.
        {
            let mut misc_out = Port::<u8>::new(0x3c2);
            misc_out.write(0x63);
        }

        // Set Sequencer Registers.
        {
            let seq_regs = [0x03, 0x01, 0x0F, 0x00, 0x0E];
            let mut seq_index = Port::<u8>::new(0x3c4);
            let mut seq_data  = Port::<u8>::new(0x3c5);
            for (i, &val) in seq_regs.iter().enumerate() {
                seq_index.write(i as u8);
                seq_data.write(val);
            }
        }

        // Set CRTC (Cathode Ray Tube Controller) Registers.
        {
            let crtc_regs: [u8; 25] = [
                0x5F, 0x4F, 0x50, 0x82, 0x54, 0x80, 0xBF, 0x1F,
                0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x9C, 0x0E, 0x8F, 0x28, 0x40, 0x96, 0xB9, 0xA3, 0xFF,
            ];
            let mut crtc_index = Port::<u8>::new(0x3d4);
            let mut crtc_data  = Port::<u8>::new(0x3d5);
            for (i, &val) in crtc_regs.iter().enumerate() {
                crtc_index.write(i as u8);
                crtc_data.write(val);
            }
        }

        // Set Graphics Controller Registers.
        {
            let gr_regs = [0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x05, 0x0F, 0xFF];
            let mut gr_index = Port::<u8>::new(0x3ce);
            let mut gr_data  = Port::<u8>::new(0x3cf);
            for (i, &val) in gr_regs.iter().enumerate() {
                gr_index.write(i as u8);
                gr_data.write(val);
            }
        }

        // Set Attribute Controller Registers.
        {
            let att_regs = [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x14, 0x07,
                0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
                0x0C, 0x0D, 0x0E, 0x0F, 0x41,
            ];
            for (i, &val) in att_regs.iter().enumerate() {
                // Reset flip-flop by reading from input status register.
                let mut _att_flip = Port::<u8>::new(0x3da);
                let _ = _att_flip.read();
                let mut att = Port::<u8>::new(0x3c0);
                att.write(i as u8);
                att.write(val);
            }
            // Re-enable display output.
            {
                let mut _att_flip = Port::<u8>::new(0x3da);
                let _ = _att_flip.read();
                let mut att = Port::<u8>::new(0x3c0);
                att.write(0x20);
            }
        }

        // Clear the framebuffer to remove residual data.
        let fb_ptr = 0xA0000 as *mut u8;
        let fb_size = 320 * 200;
        for i in 0..fb_size {
            fb_ptr.add(i).write_volatile(0);
        }
    }
}

/// Draws a centered 1px-high, 160px-long white line.
// Framebuffer address for mode 13h is assumed at 0xA0000.
pub fn draw_centered_line() {
    let fb_ptr = 0xA0000 as *mut u8;
    let screen_width = 320;
    let screen_height = 200;
    // Calculate center row and horizontal starting point.
    let y = screen_height / 2;
    let line_length = 160;
    let x_start = (screen_width - line_length) / 2;
    
    for i in 0..line_length {
        let offset = y * screen_width + x_start + i;
        unsafe {
            // Write white pixel (color index 0x0F) to the framebuffer.
            *fb_ptr.add(offset) = 0x3F // True white in Mode 13h
        }
    }
}

/// Draws a line with white color (color index 0x0F) using Bresenham's algorithm.
pub fn draw_line(start: (usize, usize), end: (usize, usize)) {
    let fb_ptr = 0xA0000 as *mut u8;
    let (mut x0, mut y0) = (start.0 as isize, start.1 as isize);
    let (x1, y1) = (end.0 as isize, end.1 as isize);
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;

    loop {
        // Plot the pixel if within the 320x200 bounds.
        if x0 >= 0 && x0 < 320 && y0 >= 0 && y0 < 200 {
            let offset = (y0 as usize) * 320 + (x0 as usize);
            unsafe { fb_ptr.add(offset).write_volatile(0x3F); } // True white in Mode 13h
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
