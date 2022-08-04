mod cg_system;
mod objects;
mod perspective;

use cg_system::CGExecutor;
use std::f32::consts::PI;
use objects::{Object, Sphere, Cube, Tetrahedron, Cylinder, Circle, Triangle, Square};
use perspective::Perspective;

type Vector3 = cgmath::Vector3<f32>;
type Vector4 = cgmath::Vector4<f32>;
type Vector2 = cgmath::Vector2<f32>;
type Point3 = cgmath::Point3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

fn main() {
    let window_width: u32 = 1920;
    let window_height: u32 = 1080;
    let mut vertex_array = vec![];

    let object_center = Vector3::new(0.0, 0.0, 0.0);
    let color = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let mut test_object = Object::<Cylinder>::new();
    test_object.rescale_x(0.05);
    test_object.rescale_y(0.05);
    test_object.rescale_z(50.0);
    test_object.recolor(color);
    test_object.generate_cylinder_nodes();
    test_object.translocate(object_center);
    vertex_array.push(test_object.encode());

    let mut test_object = Object::<Cylinder>::new();
    test_object.rescale_x(0.05);
    test_object.rescale_y(0.05);
    test_object.rescale_z(50.0);
    test_object.recolor(color);
    test_object.generate_cylinder_nodes();
    test_object.translocate(object_center);
    test_object.rotate_x(PI/2.0 as f32, object_center.y, object_center.z);
    vertex_array.push(test_object.encode());

    let mut test_object = Object::<Cylinder>::new();
    test_object.rescale_x(0.05);
    test_object.rescale_y(0.05);
    test_object.rescale_z(50.0);
    test_object.recolor(color);
    test_object.generate_cylinder_nodes();
    test_object.translocate(object_center);
    test_object.rotate_y(PI/2.0 as f32, object_center.x, object_center.z);
    vertex_array.push(test_object.encode());


    for object_index in 0..7 {
        let object_center = Vector3::new(object_index as f32 * 4.0 - 12.0, object_index as f32 * 4.0 - 16.0, object_index as f32 * 4.0 - 12.0);
        let object_radius = 1.0;
        let color = Vector4::new(object_index as f32 / 5.0, 0.0, 0.5, 1.0);
        let mut test_object = Object::<Tetrahedron>::new();
        test_object.rescale_x(object_radius*2.0);
        test_object.rescale_y(object_radius*2.0);
        test_object.rescale_z(object_radius*2.0);
        test_object.recolor(color);
        test_object.generate_tetrahedron_nodes();
        test_object.translocate(object_center);
        test_object.rotate_x(PI/2.0*(object_index) as f32, object_center.y, object_center.z);
        test_object.rotate_y(PI/3.0*(object_index) as f32, object_center.x, object_center.z);
        test_object.rotate_z(PI/4.0*(object_index) as f32, object_center.x, object_center.y);
        vertex_array.push(test_object.encode());
    }

    let camera = Point3::new(20.0, -20.0, 20.0);
    let camera_target = Point3::new(0.0, 0.0, 0.0);
    let camera_up = Vector3::new(0.0, 0.0, 1.0);

    let material_specular = Vector3::new(0.2, 0.2, 0.2);
    let material_shininess = 0.1 as f32;
    let light_direction = Vector3::new(1.0, 1.0, 0.0);
    let light_ambient = Vector3::new(0.3, 0.3, 0.3);
    let light_diffuse = Vector3::new(0.5, 0.5, 0.5);
    let light_specular = Vector3::new(0.2, 0.2, 0.2);

    let mut executor = CGExecutor::new(
        window_width,
        window_height,
        vertex_array.clone(),
        camera,
        camera_target,
        camera_up,
        material_specular,
        material_shininess,
        light_direction,
        light_ambient,
        light_diffuse,
        light_specular
    );
    executor.execute();
}