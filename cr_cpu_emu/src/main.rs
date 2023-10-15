use std::path::PathBuf;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cr_cpu_common::constants::{VRAM_HEIGHT, VRAM_WIDTH};

#[allow(unused_imports)]
use cr_cpu_common::prelude::*;

const SCREEN_WIDTH: u32 = VRAM_WIDTH;
const SCREEN_HEIGHT: u32 = VRAM_HEIGHT;

/// The state of the emulator including its emulation methods and functions
struct EmuState {
    // TODO: add a cpu to be emulated, and read its vram, then render it to the screen.
    cpu: Cpu,
}

impl EmuState {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::from_binary(PathBuf::from("code.bin")).unwrap()
        }
    }

    pub fn update(&mut self) {

        // self.cpu.execute_cycles(1);

    }

    pub fn draw(&self, frame: &mut [u8]) {

        let mut vram_iter = self.cpu.get_vram().chunks(3).enumerate();

        for (i,pixel) in frame.chunks_exact_mut(4).enumerate() {
            let (index,vram_rgb) = vram_iter.next().unwrap_or_default();
            let rgba = {
                [*vram_rgb.get(0).unwrap(), *vram_rgb.get(1).unwrap(), *vram_rgb.get(2).unwrap(), 255]
            };
            pixel.copy_from_slice(&rgba);
        }
    }
}

fn log_error<E: std::error::Error + 'static>(fn_name: &str, err: E) {
    error!("{}: {}", fn_name,err);
}

fn main() {
    // let mut cpu = Cpu::new();
    // cpu.add_to_end(&IAdd(192));
    // cpu.add_to_end(&ISub(64));
    // cpu.add_to_end(&Dump);
    // cpu.add_to_end(&Cmp(ACC, OR));
    // cpu.add_to_end(&JGT(1));
    // cpu.add_to_end(&Dump);
    //
    // cpu.execute_until_unknown();
    // env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64,SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("CR_CPU_SIM")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_tex = SurfaceTexture::new(window_size.width,window_size.height,&window);
        Pixels::new(SCREEN_WIDTH,SCREEN_HEIGHT,surface_tex).unwrap()
    };

    let mut state = EmuState::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // draw pixels into buffer
            state.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                *control_flow = ControlFlow::ExitWithCode(1);
                log_error("pixels.render", err);
                return;
            }
        }
        if input.update(&event) {
            // close window ??
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::ExitWithCode(0);
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
               state.cpu.execute_cycles(1);
            }
            // TODO: pass input to the cpu if/when we need to.
        }

        state.update();
        window.request_redraw();

    });


}
