// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Trencadis Display Server
//!
//! This is an experimental display server for rendering user interfaces
//! built with the Trencadis frameworks. It uses the [WebRender][webrender]
//! library from [Mozilla][mozilla]'s [Servo][servo] project.
//!
//! The display server is responsible for taking a concrete, laid out,
//! set of widgets and associated style information and rendering it
//! on screen.
//!
//! For testing purposes, this is currently just an executable, although,
//! in the future, it should be a library that can be embedded into other
//! applications or run as a separate program.
//!
//! [webrender]: https://github.com/servo/webrender
//! [mozilla]: https://mozilla.org/
//! [servo]: https://servo.org/

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate palette;
extern crate petgraph;
extern crate webrender;
extern crate webrender_traits;

use gleam::gl;
use std::env;
use std::path::PathBuf;
use webrender_traits::{ClipRegion, ColorF, Epoch, TransformStyle};
use webrender_traits::{DeviceUintSize, LayoutPoint, LayoutRect, LayoutSize};
use webrender_traits::PipelineId;

mod frames;

use frames::Frame;

struct Notifier {
    window_proxy: glutin::WindowProxy,
}

impl Notifier {
    fn new(window_proxy: glutin::WindowProxy) -> Notifier {
        Notifier { window_proxy: window_proxy }
    }
}

impl webrender_traits::RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let res_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    let window = glutin::WindowBuilder::new()
        .with_title("WebRender Sample")
        .with_multitouch()
        .with_gl(glutin::GlRequest::GlThenGles {
                     opengl_version: (3, 2),
                     opengles_version: (3, 0),
                 })
        .build()
        .unwrap();

    unsafe {
        window.make_current().ok();
    }

    let gl = match gl::GlType::default() {
        gl::GlType::Gl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        gl::GlType::Gles => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
    };

    println!("OpenGL version {}", gl.get_string(gl::VERSION));
    println!("Shader resource path: {:?}", res_path);

    let (width, height) = window.get_inner_size_pixels().unwrap();

    let opts = webrender::RendererOptions {
        resource_override_path: res_path,
        debug: true,
        precache_shaders: true,
        device_pixel_ratio: window.hidpi_factor(),
        ..Default::default()
    };

    let size = DeviceUintSize::new(width, height);
    let (mut renderer, sender) = webrender::renderer::Renderer::new(gl, opts, size).unwrap();
    let api = sender.create_api();

    let notifier = Box::new(Notifier::new(window.create_window_proxy()));
    renderer.set_render_notifier(notifier);

    // Set up root frame and some other stuff as a test scene.
    let bounds = LayoutRect::new(LayoutPoint::zero(),
                                 LayoutSize::new(width as f32, height as f32));

    let mut root_frame = Frame::default();

    let button_size = LayoutSize::new(50.0, 100.0);

    let mut button_a = Frame::default();
    button_a.push_rect(LayoutRect::new(LayoutPoint::new(10.0, 10.0), button_size),
                       ClipRegion::simple(&bounds),
                       ColorF::new(1.0, 0.0, 0.0, 1.0));
    root_frame.push_child(button_a);

    let mut button_b = Frame::default();
    button_b.push_rect(LayoutRect::new(LayoutPoint::new(90.0, 10.0), button_size),
                       ClipRegion::simple(&bounds),
                       ColorF::new(1.0, 0.0, 0.0, 0.5));
    root_frame.push_child(button_b);

    // Now build and render it.
    let pipeline_id = PipelineId(0, 0);
    let mut builder = webrender_traits::DisplayListBuilder::new(pipeline_id);
    builder.push_stacking_context(webrender_traits::ScrollPolicy::Scrollable,
                                  bounds,
                                  0,
                                  None,
                                  TransformStyle::Flat,
                                  None,
                                  webrender_traits::MixBlendMode::Normal,
                                  Vec::new());
    root_frame.build(&mut builder);
    builder.pop_stacking_context();

    let epoch = Epoch(0);
    let root_background_color = ColorF::new(1.0, 1.0, 1.0, 1.0);
    api.set_display_list(Some(root_background_color),
                         epoch,
                         LayoutSize::new(width as f32, height as f32),
                         builder.finalize(),
                         true);
    api.set_root_pipeline(pipeline_id);
    api.generate_frame(None);

    'outer: for event in window.wait_events() {
        let mut events = Vec::new();
        events.push(event);

        for event in window.poll_events() {
            events.push(event);
        }

        for event in events {
            match event {
                glutin::Event::Closed |
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Q)) => break 'outer,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                             _,
                                             Some(glutin::VirtualKeyCode::P)) => {
                    let enable_profiler = !renderer.get_profiler_enabled();
                    renderer.set_profiler_enabled(enable_profiler);
                    api.generate_frame(None);
                }
                _ => (),
            }
        }

        renderer.update();
        renderer.render(DeviceUintSize::new(width, height));
        window.swap_buffers().ok();
    }
}
