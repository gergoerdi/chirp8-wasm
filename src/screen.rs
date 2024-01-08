use chirp8_engine::prelude::*;

const SCALE : usize = 8;

const COLOR_ON       : u32 = 0xff_00_00_00;
const COLOR_ON_GRID  : u32 = 0xff_20_38_20;
const COLOR_OFF      : u32 = 0xff_73_bd_71;
const COLOR_OFF_GRID : u32 = 0xff_63_ad_61;

const PIX_WIDTH  : usize = SCALE * SCREEN_WIDTH as usize;
const PIX_HEIGHT : usize = SCALE * SCREEN_HEIGHT as usize;

pub fn render_framebuf(framebuf: &[ScreenRow; SCREEN_HEIGHT as usize], pixbuf: &mut [u32]) {
    for (y, row) in framebuf.iter().enumerate() {
        let mut row = *row;

        for x in 0..SCREEN_WIDTH as usize {
            let pixel = row & (1 << 63) != 0;
            row <<= 1;

            for i in 0..SCALE {
                for j in 0.. SCALE {
                    let grid_y = (i == 0) | (i == SCALE - 1);
                    let grid_x = (j == 0) | (j == SCALE - 1);

                    let ptr = ((y as usize * SCALE + i) * PIX_WIDTH) + (x as usize * SCALE + j);
                    pixbuf[ptr] =
                        if grid_x || grid_y {
                            if pixel {COLOR_ON_GRID} else {COLOR_OFF_GRID}
                        } else {
                            if pixel {COLOR_ON} else {COLOR_OFF}
                        }
                }
            }
        }
    };
}
