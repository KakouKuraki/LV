use std::marker::PhantomData;

use cgmath::Transform;
use cgmath::Rad;
use cgmath::InnerSpace;

type Vector3 = cgmath::Vector3<f32>;
type Matrix3 = cgmath::Matrix3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Vector4 = cgmath::Vector4<f32>;
type Vector2 = cgmath::Vector2<f32>;


pub struct Node {
    coordinate: Vector3,
    normal: Vector3,
    color: Vector4,
    texture: Vector2,
}

impl Node {
    fn new(coordinate: Vector3, normal: Vector3, color: Vector4, texture: Vector2) -> Self {
        Node {
            coordinate,
            normal,
            color,
            texture,
        }
    }

    fn encode(self) -> Vec<f32> {
        vec![
            self.coordinate.x, self.coordinate.y, self.coordinate.z,
            self.normal.x, self.normal.y, self.normal.z,
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.texture.x, self.texture.y
        ]
    }

    fn translocate(&mut self, diff: Vector3) {
        self.coordinate += diff;
    }


    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32) {
        let diff = Vector3::new(0.0, center_y, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_x);
        let rot_x = Matrix4::from_angle_x(rad);
        self.coordinate = rot_x.transform_vector(self.coordinate);
        self.normal = rot_x.transform_vector(self.normal);
        self.translocate(diff);
    }

    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32) {
        let diff = Vector3::new(center_x, 0.0, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_y);
        let rot_y = Matrix4::from_angle_y(rad);
        self.coordinate = rot_y.transform_vector(self.coordinate);
        self.normal = rot_y.transform_vector(self.normal);
        self.translocate(diff);
    }

    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32) {
        let diff = Vector3::new(center_x, center_y, 0.0);
        self.translocate(-diff);
        let rad = Rad(theta_z);
        let rot_z = Matrix4::from_angle_z(rad);
        self.coordinate = rot_z.transform_vector(self.coordinate);
        self.normal = rot_z.transform_vector(self.normal);
        self.translocate(diff);
    }
}   //Node


pub trait SphereTrait{}
pub struct Sphere;
impl SphereTrait for Sphere{}

pub trait CubeTrait{}
pub struct Cube;
impl CubeTrait for Cube{}

pub trait TetrahedronTrait{}
pub struct Tetrahedron;
impl TetrahedronTrait for Tetrahedron{}

pub trait CylinderTrait{}
pub struct Cylinder;
impl CylinderTrait for Cylinder{}

pub trait CircleTrait{}
pub struct Circle;
impl CircleTrait for Circle{}

pub trait SquareTrait{}
pub struct Square;
impl SquareTrait for Square{}

pub trait TriangleTrait{}
pub struct Triangle;
impl TriangleTrait for Triangle{}

pub struct Object<T>{
    phantom: PhantomData<T>,
    pub center: Vector3,
    pub scale: Vector3,
    pub color: Vector4,
    pub nodes: Vec<Node>,
}

impl<T> Object<T>{
    pub fn new() -> Self {
        Self {
            phantom: PhantomData, 
            center: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 0.0, 0.0),
            color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            nodes: Vec::new(),
        }
    }

    pub fn rescale_x(&mut self, scale: f32) {
        self.scale.x = scale;
    }

    pub fn rescale_y(&mut self, scale: f32) {
        self.scale.y = scale;
    }

    pub fn rescale_z(&mut self, scale: f32) {
        self.scale.z = scale;
    }

    pub fn recolor(&mut self, color: Vector4) {
        self.color = color;
    }

    pub fn translocate(&mut self, diff: Vector3) {
        for node in &mut self.nodes {
            node.translocate(diff);
        }
    }

    pub fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32) {
        for node in &mut self.nodes {
            node.rotate_x(theta_x, center_y, center_z);
        }
    }

    pub fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32) {
        for node in &mut self.nodes {
            node.rotate_y(theta_y, center_x, center_z);
        }
    }

    pub fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32) {
        for node in &mut self.nodes {
            node.rotate_z(theta_z, center_x, center_y);
        }
    }

    pub fn encode(self) -> Vec<f32> {
        let mut ret = vec![];
        for node in self.nodes {
            for val in node.encode() {
                ret.push(val);
            }
        }
        ret
    }
}   //Object


impl<T: SphereTrait> Object<T> {
    pub fn generate_sphere_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);
        for slice in 0..32 {
            for stack in 0..32 {
                // 1
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 2
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 3
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 4
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 5
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 6
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));
            }
        }
    }
}   //Sphere

impl<T: CubeTrait> Object<T> {
    pub fn generate_cube_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);

        self.scale *= 2.0;
        let offset = Vector3::new(-0.5, -0.5, -0.5);

        let normal_vecs:Vec<[f32;3]> 
        = vec![
            [-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0], 
            [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]
        ];

        for normal_vec in normal_vecs{
            for i in 0..3{
                let mut cube_vertex: [f32;3] = [0.0, 0.0, 0.0];
                if normal_vec[i] == -1.0{
                    //1
                    cube_vertex[i] = 0.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //2
                    cube_vertex[(i+2)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //3
                    cube_vertex[(i+1)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //4
                    for t in 0..3{ cube_vertex[t] = 0.0; }
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //5
                    cube_vertex[(i+2)%3] = 1.0;
                    cube_vertex[(i+1)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //6
                    cube_vertex[(i+2)%3] = 0.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    break;
                }
                if normal_vec[i] == 1.0{
                    //1
                    cube_vertex[i] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //2
                    cube_vertex[(i+1)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //3
                    cube_vertex[(i+2)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //4
                    for t in 0..3{ cube_vertex[t] = 0.0; }
                    cube_vertex[i] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //5
                    cube_vertex[(i+2)%3] = 1.0;
                    cube_vertex[(i+1)%3] = 1.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    //6
                    cube_vertex[(i+1)%3] = 0.0;
                    self.nodes.push(Node::new(array_to_coordinate(self.center, cube_vertex, offset, self.scale),
                        array_to_Vector3(normal_vec), self.color, texture));
                    break;
                }
            };

        }
        
    }

}   //Cube

impl <T: TetrahedronTrait> Object<T> {
    pub fn generate_tetrahedron_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);
        
        let theta = ((-1.0 / 3.0) as f32).acos();
        let mountain_top = Vector3::new(0.0, 0.0, self.scale.z);
        for i in 0..3{
            let phi = i as f32 * 2.0 * -std::f32::consts::FRAC_PI_3;
            let bottom_triangle_vertex1 = PoolarVector::new(self.center, self.scale, theta, phi).to_xyz();
            let phi = (i + 1) as f32 * 2.0 * -std::f32::consts::FRAC_PI_3;
            let bottom_triangle_vertex2 = PoolarVector::new(self.center, self.scale, theta, phi).to_xyz();
            let normal_vec = (mountain_top - bottom_triangle_vertex1).cross(bottom_triangle_vertex2 - mountain_top).normalize();
            self.nodes.push(Node::new(bottom_triangle_vertex1, normal_vec, self.color, texture));
            self.nodes.push(Node::new(mountain_top, normal_vec, self.color, texture));
            self.nodes.push(Node::new(bottom_triangle_vertex2, normal_vec, self.color, texture));
        }
        for i in 0..3{
            let phi = i as f32 * 2.0 * -std::f32::consts::FRAC_PI_3;
            let bottom_triangle_vertex = PoolarVector::new(self.center, self.scale, theta, phi).to_xyz();
            self.nodes.push(Node::new(bottom_triangle_vertex, Vector3::new(0.0, 0.0, -1.0), self.color, texture))
        }
    }
}   //tetrahedron

impl <T: CylinderTrait> Object<T> {
    pub fn generate_cylinder_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);

        for stack in 0..32{
            let bottom_normal_vec = Vector3::new(0.0, 0.0, 1.0);
            let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
            let mut sector_vertex1 = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi).to_xyz();
            let phi = ((stack + 1) as f32 / 32.0) * std::f32::consts::PI*2.0;
            let mut sector_vertex2 = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi).to_xyz();
            let lateral_normal_vec = ((sector_vertex1 + sector_vertex2) / 2.0).normalize();

            sector_vertex1.z += self.scale.z / 2.0;
            sector_vertex2.z += self.scale.z / 2.0;
            self.nodes.push(Node::new(sector_vertex1, bottom_normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex2, bottom_normal_vec, self.color, texture));
            self.nodes.push(Node::new(Vector3::new(self.center.x, self.center.y, self.center.z + self.scale.z / 2.0), bottom_normal_vec, self.color, texture));

            self.nodes.push(Node::new(sector_vertex1, lateral_normal_vec, self.color, texture));
            sector_vertex1.z -= self.scale.z;
            self.nodes.push(Node::new(sector_vertex1, lateral_normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex2, lateral_normal_vec, self.color, texture));

            self.nodes.push(Node::new(sector_vertex2, lateral_normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex1, lateral_normal_vec, self.color, texture));
            sector_vertex2.z -= self.scale.z;
            self.nodes.push(Node::new(sector_vertex2, lateral_normal_vec, self.color, texture));
            
            let bottom_normal_vec = Vector3::new(0.0, 0.0, -1.0);
            self.nodes.push(Node::new(sector_vertex2, bottom_normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex1, bottom_normal_vec, self.color, texture));
            self.nodes.push(Node::new(Vector3::new(self.center.x, self.center.y, self.center.z - self.scale.z / 2.0), bottom_normal_vec, self.color, texture));
        }
    }
}

impl <T: CircleTrait> Object<T> {
    pub fn generate_circle_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);

        for stack in 0..32{
            let normal_vec = Vector3::new(0.0, 0.0, -1.0);
            let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
            let sector_vertex1 = PoolarVector::new(self.center, self.scale , std::f32::consts::FRAC_PI_2, phi);
            let phi = ((stack + 1) as f32 / 32.0) * std::f32::consts::PI*2.0;
            let sector_vertex2 = PoolarVector::new(self.center, self.scale , std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(sector_vertex1.to_xyz(), normal_vec, self.color, texture));
            self.nodes.push(Node::new(self.center, normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex2.to_xyz(), normal_vec, self.color, texture));
            let normal_vec = Vector3::new(0.0, 0.0, 1.0);
            self.nodes.push(Node::new(sector_vertex2.to_xyz(), normal_vec, self.color, texture));
            self.nodes.push(Node::new(self.center, normal_vec, self.color, texture));
            self.nodes.push(Node::new(sector_vertex1.to_xyz(), normal_vec, self.color, texture));
        }
    }
}   //Circle

impl <T: TriangleTrait> Object<T> {
    pub fn generate_triangle_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);

        let normal_vec = Vector3::new(0.0, 0.0, 1.0);
        for i in 0..3{
            let phi = i as f32 * std::f32::consts::FRAC_PI_3 * 2.0;
            let vertex = PoolarVector::new(self.center, self.scale , std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }

        let normal_vec = Vector3::new(0.0, 0.0, -1.0);
        for i in 0..3{
            let phi = i as f32 * -std::f32::consts::FRAC_PI_3 * 2.0;
            let vertex = PoolarVector::new(self.center, self.scale , std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }
    }
}   //Triangle

impl <T: SquareTrait> Object<T> {
    pub fn generate_square_nodes(&mut self) {
        let texture = Vector2::new(0.0, 0.0);

        let normal_vec = Vector3::new(0.0, 0.0, 1.0);
        for i in 0..3{
            let phi = i as f32 * std::f32::consts::FRAC_PI_2;
            let vertex = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }
        for i in 0..3{
            let phi = (i + 2) as f32 * std::f32::consts::FRAC_PI_2;
            let vertex = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }

        let normal_vec = Vector3::new(0.0, 0.0, -1.0);
        for i in 0..3{
            let phi = i as f32 * -std::f32::consts::FRAC_PI_2;
            let vertex = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }
        for i in 0..3{
            let phi = (i + 2) as f32 * -std::f32::consts::FRAC_PI_2;
            let vertex = PoolarVector::new(self.center, self.scale, std::f32::consts::FRAC_PI_2, phi);
            self.nodes.push(Node::new(vertex.to_xyz(), normal_vec, self.color, texture));
        }
    }
}   //Square



struct PoolarVector {
    begin: Vector3,
    r: Vector3,
    theta: f32,
    phi: f32,
}

impl  PoolarVector{
    fn new(begin: Vector3, r: Vector3, theta: f32, phi: f32) -> PoolarVector {
        PoolarVector {
            begin,
            r,
            theta,
            phi,
        }
    }
    
    fn to_xyz(&self) -> Vector3 {
        let x = self.r.x*self.theta.sin()*self.phi.cos()+self.begin.x;
        let y = self.r.y*self.theta.sin()*self.phi.sin()+self.begin.y;
        let z = self.r.z*self.theta.cos()+self.begin.z;
        Vector3::new(x, y, z)
    }
    
    fn to_normal_xyz(&self) -> Vector3 {
        let r_norm = self.r.magnitude();
        let nx = (self.r.x/r_norm)*self.theta.sin()*self.phi.cos();
        let ny = (self.r.y/r_norm)*self.theta.sin()*self.phi.sin();
        let nz = (self.r.z/r_norm)*self.theta.cos();
        Vector3::new(nx, ny, nz)
    }
}


fn array_to_Vector3(arr: [f32;3]) -> Vector3{
    Vector3::new(arr[0], arr[1], arr[2])
}

fn array_to_coordinate(begin: Vector3, p: [f32;3], offset: Vector3, scale: Vector3) -> Vector3 {
    let x = (p[0] + offset.x) * scale.x;
    let y = (p[1] + offset.y) * scale.y;
    let z = (p[2] + offset.z) * scale.z;
    begin + Vector3::new(x, y, z)
}