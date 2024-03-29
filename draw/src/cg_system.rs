use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use imgui::im_str;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState, MouseButton, MouseWheelDirection};

use anyhow::{bail, Context as AnyhowContext, Result};
use cstr::cstr;
use gl::types::*;
use glfw::{Context, OpenGlProfileHint, Window, WindowHint};
use rsmpeg::{
    avcodec::{AVCodec, AVCodecContext},
    avformat::AVFormatContextOutput,
    avutil::{ra, AVFrame},
    error::RsmpegError,
};
use std::{
    ffi::{CStr, CString},
    fs,
    mem::size_of,
    ptr::null,
};

mod frame_buffer;
mod image_manager;
mod shader;
mod vertex;

use super::perspective::Perspective;

use frame_buffer::FrameBuffer;
use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector2 = cgmath::Vector2<f32>;
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct CGExecutor {
    window_width: u32,
    window_height: u32,
    vertex_array: Vec<Vec<f32>>, // Vec<[[3, 3, 4, 2]; N]>
    camera: Point3,
    camera_target: Point3,
    camera_up: Vector3,
    material_specular: Vector3,
    material_shininess: f32,
    light_direction: Vector3,
    light_ambient: Vector3,
    light_diffuse: Vector3,
    light_specular: Vector3,
}

enum ShaderMode {
    General,
    Sphere,
    Bloom,
    RetroTV,
}

impl CGExecutor {
    pub fn new(
        window_width: u32,
        window_height: u32,
        vertex_array: Vec<Vec<f32>>,
        camera: Point3,
        camera_target: Point3,
        camera_up: Vector3,
        material_specular: Vector3,
        material_shininess: f32,
        light_direction: Vector3,
        light_ambient: Vector3,
        light_diffuse: Vector3,
        light_specular: Vector3,
        ) -> CGExecutor {

        CGExecutor {
            window_width,
            window_height,
            vertex_array,
            camera,
            camera_target,
            camera_up,
            material_specular,
            material_shininess,
            light_direction,
            light_ambient,
            light_diffuse,
            light_specular,
        }
    }

    pub fn execute(&mut self) -> Result<()> {

        fs::create_dir_all("././output/").unwrap();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 2);
            let (major, minor) = gl_attr.context_version();
            println!("OK: init OpenGL: version={}.{}", major, minor);
        }

        let window = video_subsystem
            .window("SDL2", self.window_width, self.window_height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);


        let mut shader_mode = ShaderMode::General;

        let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");
        let screen_shader = Shader::new("rsc/shader/screen_shader.vs", "rsc/shader/screen_shader.fs");
        let screen_shader_sphere = Shader::new("rsc/shader/screen_shader_sphere.vs", "rsc/shader/screen_shader_sphere.fs");
        let screen_shader_bloom = Shader::new("rsc/shader/screen_shader_bloom.vs", "rsc/shader/screen_shader_bloom.fs");
        let screen_shader_retro_tv = Shader::new("rsc/shader/screen_shader_retro_tv.vs", "rsc/shader/screen_shader_retro_tv.fs");
        let frame_buffer = FrameBuffer::new(self.window_width, self.window_height);

        let vertex_vec = Self::new_screen_vertex_vec(-1.0, -1.0, 1.0, 1.0, 20);

        let screen_vertex = Vertex::new(
            (vertex_vec.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertex_vec.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
            vec![gl::FLOAT, gl::FLOAT],
            vec![3, 2],
            5 * mem::size_of::<GLfloat>() as GLsizei,
            20 * 20 * 2 * 3,
        );

        let mut vertexes = vec![];

        for vertex_element in &self.vertex_array {
            let vertex = Vertex::new(
                (vertex_element.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertex_element.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
                vec![gl::FLOAT, gl::FLOAT, gl::FLOAT, gl::FLOAT],
                vec![3, 3, 4, 2],
                12 * mem::size_of::<GLfloat>() as GLsizei,
                (vertex_element.len() / 12) as i32,
            );
            vertexes.push(vertex);
        }

        // init imgui
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        // init imgui sdl2
        let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
            video_subsystem.gl_get_proc_address(s) as _
        });



        // Set current frame width and height to `iResolution`.
        unsafe { shader.set_2int(cstr!("iResolution"), self.window_width as i32, self.window_height as i32) }

        let mut encode_context = {
            let encoder =
                AVCodec::find_encoder_by_name(cstr!("png")).context("Failed to find encoder codec")?;
            let mut encode_context = AVCodecContext::new(&encoder);
            encode_context.set_bit_rate(400000);
            encode_context.set_width(self.window_width as i32);
            encode_context.set_height(self.window_height as i32);
            encode_context.set_time_base(ra(1, 30));
            encode_context.set_framerate(ra(30, 1));
            encode_context.set_gop_size(10);
            encode_context.set_max_b_frames(1);
            encode_context.set_pix_fmt(rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_RGB24);
            encode_context.open(None)?;
            encode_context
        };

        // Create a reusable frame buffer holder.
        let mut frame = AVFrame::new();
        frame.set_format(encode_context.pix_fmt);
        frame.set_width(encode_context.width);
        frame.set_height(encode_context.height);
        frame.alloc_buffer()?;

        let mut output_format_context = {
            let output_video_path = cstr!("././output/test.mp4");
            let mut output_format_context = AVFormatContextOutput::create(output_video_path, None)?;
            {
                let mut stream = output_format_context.new_stream();
                stream.set_codecpar(encode_context.extract_codecpar());
                stream.set_time_base(encode_context.time_base);
            }
            output_format_context.dump(0, output_video_path)?;
            output_format_context.write_header(&mut None)?;
            output_format_context
        };


        let mut depth_test: bool = true;
        let mut blend: bool = true;
        let mut wireframe: bool = false;
        let mut culling: bool = true;
        
        let start_time = std::time::Instant::now();

        let mut debug_window_mode = true;

        let mut is_mouse_pushed = false;
        let mut clicked_button = MouseButton::Unknown;
        let mut clicked_pos: [i32; 2] = [0, 0];

        let mut is_mouse_wheel_rolling = false;
        let mut mouse_wheel = MouseWheelDirection::Normal;

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut frame_no = 0;

        'running: loop {
            for event in event_pump.poll_iter() {
                imgui_sdl2_context.handle_event(&mut imgui_context, &event);
                if imgui_sdl2_context.ignore_event(&event) {
                    continue;
                }

                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        Self::encode_movie(&mut encode_context, None, &mut output_format_context)?;
                        output_format_context.write_trailer()?;
                        break 'running;
                    }
                        Event::KeyDown {
                        keycode: Some(Keycode::D),
                        repeat: false,
                        ..
                    } => {
                        debug_window_mode = !debug_window_mode;
                        println!("debug mode: {}", debug_window_mode);
                    }
                    Event::MouseButtonDown {
                        mouse_btn,
                        x,
                        y,
                        ..
                    } => {
                        is_mouse_pushed = true;
                        clicked_button = mouse_btn;
                        clicked_pos[0] = x;
                        clicked_pos[1] = y;
                    }
                    Event::MouseButtonUp{
                        ..
                    } => {
                        is_mouse_pushed = false;
                    }
                    Event::MouseWheel{
                        direction,
                        ..
                    } => {}
                    _ => {}
                }
            }

            unsafe {
                frame_buffer.bind_as_frame_buffer();

                if depth_test {
                    gl::Enable(gl::DEPTH_TEST);
                } else {
                    gl::Disable(gl::DEPTH_TEST);
                }

                if blend {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                } else {
                    gl::Disable(gl::BLEND);
                }

                if wireframe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                } else {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }

                if culling {
                    gl::Enable(gl::CULL_FACE);
                } else {
                    gl::Disable(gl::CULL_FACE);
                }

                gl::Viewport(0, 0, self.window_width as i32, self.window_height as i32);

                // clear screen
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // init matrice for model, view and projection
                let model_matrix = Matrix4::identity();
                let projection_matrix: Matrix4 = perspective(
                    cgmath::Deg(45.0f32),
                    self.window_width as f32 / self.window_height as f32,
                    0.1,
                    100.0,
                );

                imgui_sdl2_context.prepare_frame(
                    imgui_context.io_mut(),
                    &window,
                    &event_pump.mouse_state(),
                );
                let ui = imgui_context.frame();


                let mouse_pos = ui.io().mouse_pos;
                let mouse_displacement: [f32; 2] = [mouse_pos[0] as f32 - clicked_pos[0] as f32, mouse_pos[1] as f32 - clicked_pos[1] as f32];
                
                let view_matrix = match clicked_button{
                    MouseButton::Left => {
                        let v = Perspective::rotate(self.camera, self.camera_target, self.camera_up, 
                            Vector2::new(mouse_displacement[0] / 100.0, mouse_displacement[1] / 100.0));
                        if !is_mouse_pushed {
                            self.camera = v.camera; self.camera_target = v.target; self.camera_up = v.up;
                            clicked_button = MouseButton::Unknown;
                        }
                        Matrix4::look_at(v.camera, v.target, v.up)
                    },
                    MouseButton::Middle => {
                        let v = Perspective::translocate(self.camera, self.camera_target, self.camera_up,
                            Vector2::new(mouse_displacement[0] / 30.0, mouse_displacement[1] / 30.0));
                        if !is_mouse_pushed {
                            self.camera = v.camera; self.camera_target = v.target; self.camera_up = v.up;
                            clicked_button = MouseButton::Unknown;
                        }
                        Matrix4::look_at(v.camera, v.target, v.up)
                    },
                    MouseButton::Unknown => {
                        let wheel = ui.io().mouse_wheel;
                        if wheel != 0.0 {
                            let v = Perspective::translocate_forward(self.camera, self.camera_target, self.camera_up, wheel * 5.0);
                            self.camera = v.camera; self.camera_target = v.target; self.camera_up = v.up;
                            Matrix4::look_at(v.camera, v.target, v.up)
                        }
                        else {
                            Matrix4::look_at(self.camera, self.camera_target, self.camera_up)
                        }
                    },

                    _ => {
                        Matrix4::look_at(self.camera, self.camera_target, self.camera_up)
                    }
                };

                // shader use matrices
                shader.use_program();
                // shader.set_float(c_str!("iTime"), frame_no as f32 / 45.);
                shader.set_mat4(c_str!("uModel"), &model_matrix);
                shader.set_mat4(c_str!("uView"), &view_matrix);
                shader.set_mat4(c_str!("uProjection"), &projection_matrix);
                shader.set_vec3(c_str!("uViewPosition"), self.camera.x, self.camera.y, self.camera.z);
                shader.set_vector3(c_str!("uMaterial.specular"), &self.material_specular);
                shader.set_float(c_str!("uMaterial.shininess"), self.material_shininess);
                shader.set_vector3(c_str!("uLight.direction"), &self.light_direction);
                shader.set_vector3(c_str!("uLight.ambient"), &self.light_ambient);
                shader.set_vector3(c_str!("uLight.diffuse"), &self.light_diffuse);
                shader.set_vector3(c_str!("uLight.specular"), &self.light_specular);

                for vertex in &vertexes {
                    vertex.draw();
                }

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

                gl::Enable(gl::DEPTH_TEST);

                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

                gl::Enable(gl::CULL_FACE);

                // clear screen
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                frame_buffer.bind_as_texture();

                match shader_mode {
                    ShaderMode::General => {
                        screen_shader.use_program();
                    }
                    ShaderMode::Sphere => {
                        screen_shader_sphere.use_program();
                    }
                    ShaderMode::Bloom => {
                        screen_shader_bloom.use_program();
                    }
                    ShaderMode::RetroTV => {
                        screen_shader_retro_tv.use_program();
                        #[rustfmt::skip]
                        screen_shader_retro_tv
                            .set_float(c_str!("uScreenHeight"), self.window_height as f32);
                        let now_time = std::time::Instant::now();
                        screen_shader_retro_tv
                            .set_float(c_str!("uTime"), (now_time - start_time).as_secs_f32());
                    }
                }

                screen_vertex.draw();
                gl::BindTexture(gl::TEXTURE_2D, 0);


                
                imgui::Window::new(im_str!("Information"))
                    .size([300.0, 450.0], imgui::Condition::FirstUseEver)
                    .position([10.0, 10.0], imgui::Condition::FirstUseEver)
                    .build(&ui, || {
                        ui.text(im_str!("OpenGL Test App ver 1.0"));
                        ui.separator();
                        ui.text(im_str!("FPS: {:.1}", ui.io().framerate));
                        let display_size = ui.io().display_size;
                        ui.text(format!(
                            "Display Size: ({:.1}, {:.1})",
                            display_size[0], display_size[1]
                        ));
                        let mouse_pos = ui.io().mouse_pos;
                        ui.text(format!(
                            "Mouse Position: ({:.1}, {:.1})",
                            mouse_pos[0], mouse_pos[1]
                        ));

                        ui.separator();

                        ui.checkbox(im_str!("Depth Test"), &mut depth_test);
                        ui.checkbox(im_str!("Blend"), &mut blend);
                        ui.checkbox(im_str!("Wireframe"), &mut wireframe);
                        ui.checkbox(im_str!("Culling"), &mut culling);

                        ui.separator();

                        ui.text(im_str!("FBO Shader"));

                        if ui.button(im_str!("General"), [60.0, 20.0]) {
                            shader_mode = ShaderMode::General;
                        }
                        ui.same_line(80.0);
                        if ui.button(im_str!("Sphere"), [60.0, 20.0]) {
                            shader_mode = ShaderMode::Sphere;
                        }
                        ui.same_line(150.0);
                        if ui.button(im_str!("Bloom"), [60.0, 20.0]) {
                            shader_mode = ShaderMode::Bloom;
                        }
                        ui.same_line(220.0);
                        if ui.button(im_str!("RetroTV"), [60.0, 20.0]) {
                            shader_mode = ShaderMode::RetroTV;
                        }
                    });

                imgui_sdl2_context.prepare_render(&ui, &window);
                if debug_window_mode {
                    renderer.render(ui);
                }

                let buffer =
                {
                    let mut buffer = vec![0u8; (self.window_width as i32 * self.window_height as i32 * 3) as _];
                    let buffer_ptr = buffer.as_mut_ptr() as _;
                    unsafe { gl::ReadPixels(0, 0, self.window_width as i32, self.window_height as i32, gl::RGB, gl::UNSIGNED_BYTE, buffer_ptr) };
                    buffer
                };
                let data = frame.data[0];
                let linesize = frame.linesize[0] as usize;
                let width = self.window_width as usize;
                let height = self.window_height as usize;
                let rgb_data = unsafe { std::slice::from_raw_parts_mut(data, height * linesize * 3) };
                for y in 0..height {
                    for x in 0..width {
                        rgb_data[y * linesize + x * 3 + 0] = buffer[(y * width + x) * 3 + 0];
                        rgb_data[y * linesize + x * 3 + 1] = buffer[(y * width + x) * 3 + 1];
                        rgb_data[y * linesize + x * 3 + 2] = buffer[(y * width + x) * 3 + 2];
                    }
                }

                frame.set_pts(frame_no);
                Self::encode_movie(
                    &mut encode_context,
                    Some(&frame),
                    &mut output_format_context,
                )?;
                
                window.gl_swap_window();
                frame_no += 1;
            }

            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    fn new_screen_vertex_vec(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        division: i32,
    ) -> std::vec::Vec<f32> {
        let mut vertex_vec: std::vec::Vec<f32> = std::vec::Vec::new();

        for x in 0..division {
            for y in 0..division {
                let l = left + (right - left) / division as f32 * x as f32;
                let r = left + (right - left) / division as f32 * (x + 1) as f32;
                let t = top + (bottom - top) / division as f32 * y as f32;
                let b = top + (bottom - top) / division as f32 * (y + 1) as f32;

                let lc = 1.0 / division as f32 * x as f32;
                let rc = 1.0 / division as f32 * (x + 1) as f32;
                let tc = 1.0 / division as f32 * y as f32;
                let bc = 1.0 / division as f32 * (y + 1) as f32;

                vertex_vec.extend([l, t, 0.0, lc, tc].iter().cloned());
                vertex_vec.extend([r, t, 0.0, rc, tc].iter().cloned());
                vertex_vec.extend([l, b, 0.0, lc, bc].iter().cloned());
                vertex_vec.extend([l, b, 0.0, lc, bc].iter().cloned());
                vertex_vec.extend([r, t, 0.0, rc, tc].iter().cloned());
                vertex_vec.extend([r, b, 0.0, rc, bc].iter().cloned());
            }
        }

        vertex_vec
    }

    fn encode_movie(
        encode_context: &mut AVCodecContext,
        frame: Option<&AVFrame>,
        output_format_context: &mut AVFormatContextOutput,
    ) -> Result<()> {
        encode_context.send_frame(frame)?;
        loop {
            let mut packet = match encode_context.receive_packet() {
                Ok(packet) => packet,
                Err(RsmpegError::EncoderDrainError) | Err(RsmpegError::EncoderFlushedError) => break,
                Err(e) => return Err(e.into()),
            };
            packet.rescale_ts(
                encode_context.time_base,
                output_format_context.streams().get(0).unwrap().time_base,
            );
            output_format_context.write_frame(&mut packet)?;
        }
        Ok(())
    }
}
