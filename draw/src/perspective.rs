use cgmath::InnerSpace;
use cgmath::Rad;
use cgmath::Deg;
use cgmath::Matrix;

type Vector2 = cgmath::Vector2<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Vector4 = cgmath::Vector4<f32>;
type Point3 = cgmath::Point3<f32>;
type Matrix3 = cgmath::Matrix3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Perspective {
    pub camera: Point3,
    pub target: Point3,
    pub up: Vector3,
}

impl Perspective {
    pub fn rotate(
        camera: Point3,
        target: Point3,
        up: Vector3,
        rad: Vector2,
    ) -> Perspective 
    {
        let view_matrix = Matrix4::look_at(camera, target, up);
        let s = view_matrix.row(0).truncate();
        let u = view_matrix.row(1).truncate().normalize();
        let f = camera - target;

        let horizontal_rotate_mat = Matrix3::from_axis_angle(u, Rad(-rad.x));
        let vertical_rotate_mat = Matrix3::from_axis_angle(s, Rad(-rad.y));
        let rotation_mat = vertical_rotate_mat * horizontal_rotate_mat;
        let camera_pos = rotation_mat * Vector3::new(f.x, f.y, f.z);

        // println!("camera: {}, {}, {}", camera_pos.x, camera_pos.y, camera_pos.z);
        Perspective {camera: Point3::new(camera_pos.x + target.x, camera_pos.y + target.y, camera_pos.z + target.z),
             target: target, up: rotation_mat * up}
    }

    pub fn translocate(
        camera: Point3,
        target: Point3,
        up: Vector3,
        mouse_move: Vector2,
    )-> Perspective
    {
        let view_matrix = Matrix4::look_at(camera, target, up);
        let transition 
        = Vector3::new(view_matrix.x.x, view_matrix.y.x, view_matrix.z.x) * -mouse_move.x 
        + Vector3::new(view_matrix.x.y, view_matrix.y.y, view_matrix.z.y) * mouse_move.y;
        let next_camera_pos = camera + transition;
        
        Perspective {camera: next_camera_pos, target: target + transition , up: up}
    }

    pub fn translocate_forward(       
        camera: Point3,
        target: Point3,
        up: Vector3,
        mouse_move: f32,
    )-> Perspective
    {
        let forward = Vector3::new(target.x - camera.x, target.y - camera.y, target.z - camera.z).normalize() * mouse_move;

        Perspective {camera: camera + forward, target: target + forward, up: up}
    }
}
