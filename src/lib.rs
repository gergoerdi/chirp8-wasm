mod panic_hook;
mod screen;

use wasm_bindgen::prelude::*;

use chirp8_engine::prelude::*;
use chirp8_engine::cpu::CPU;
use chirp8_engine::quirks::*;
use chirp8_engine::peripherals::*;
use chirp8_engine::font::*;

use screen::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type FrameBuf = [ScreenRow; SCREEN_HEIGHT as usize];

struct WasmPeripherals {
    framebuf: FrameBuf,
    keys: u16,
    ram: [u8; 0x1000],
}

impl WasmPeripherals {
    pub fn new() -> Self {
        WasmPeripherals {
            framebuf: [0; SCREEN_HEIGHT as usize],
            keys: 0,
            ram: [0; 0x1000],
        }
    }
}

impl Peripherals for WasmPeripherals {
    fn set_pixel_row(&mut self, y: ScreenY, row: ScreenRow) {
        self.framebuf[y as usize] = row;
    }

    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow {
        self.framebuf[y as usize]
    }

    fn get_keys(&self) -> u16 {
        self.keys
    }

    fn set_sound(&mut self, val: Byte) {
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        self.ram[addr as usize]
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        self.ram[addr as usize] = val;
    }
}

#[wasm_bindgen]
pub struct Ctx {
    cpu: CPU,
    virt: WasmPeripherals,
}

#[wasm_bindgen]
pub fn setup() -> Ctx {
    panic_hook::set_panic_hook();

    let bytes = include_bytes!("../hidden.ch8");
    let quirks = Quirks {
        shift_vy: false,
        reset_vf: false,
        increment_ptr: false,
        video_wait: true,
        clip_sprites: true,
    };

    let mut cpu = CPU::new(quirks);
    let mut virt = WasmPeripherals::new();

    for (addr, b) in FONT_HEX.iter().enumerate() {
        virt.write_ram(addr as Addr, *b);
    }

    let mut ptr = 0x0200;
    for b in bytes {
        virt.write_ram(ptr, *b);
        ptr += 1;
    }

    Ctx { cpu, virt }
}

#[wasm_bindgen]
pub fn render_image(ctx: &Ctx, pixbuf: &mut [u32]) {
    let framebuf = &ctx.virt.framebuf;

    render_framebuf(framebuf, pixbuf);
}

#[wasm_bindgen]
pub fn step(ctx: &mut Ctx, keys: u16) {
    let ref mut cpu = &mut ctx.cpu;
    let ref mut virt = &mut ctx.virt;

    virt.keys = keys;
    for _ in 0..1000 {
        cpu.step(*virt)
    };
    cpu.tick_frame();
}
