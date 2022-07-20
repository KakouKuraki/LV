use image_manager;
use image_manager::ImageManager;

type Vector2f = cgmath::Vector2<f32>;
type Vector3f = cgmath::Vector3<f32>;
type Vector4f = cgmath::Vector4<f32>;
type Vector2i = cgmath::Vector2<i32>;

pub struct RootTimeline {
    pub tl_2d: Timeline2D,
    pub tl_3d: Timeline3D,
}

impl RootTimeline {
    pub fn gen(self, frame_no: i32){

    }
}



pub struct Timeline2D {
    width: i32,
    height: i32,
    background: ImageManager,
    layers: Vec<Layer2D>,
}

pub struct Timeline3D {
    layers: Vec<Layer3D>,
}

pub struct Layer2D {
    objects: Vec<Object2D>,
}

pub struct Layer3D {
    objects: Vec<Object3D>,
}

pub struct Object2D {
    kind: Object2DKind,
    frame: Vector2i,
    position_start: Vector2f,
    position_end: Vector2f,
    rotation_start: Vector2f,
    rotation_end: Vector2f,
    scale_start: Vector2f,
    scale_end: Vector2f,
    center_start: Vector2f,
    center_end: Vector2f,
    color: Vector4f,
}
pub struct Object3D {
    kind: Object3DKind,
    frame: Vector2i,
    position_start: Vector3f,
    position_end: Vector3f,
    rotation_start: Vector3f,
    rotation_end: Vector3f,
    scale_start: Vector3f,
    scale_end: Vector3f,
    center_start: Vector3f,
    center_end: Vector3f,
    color: Vector4f,
}

pub enum Object2DKind {
    Circle,
    Square,
    Triangle,
    Image,
    Str,
}

pub enum Object3DKind {
    Sphere,
    Cube,
    Tetrahedron,
    Image,
    Str,
}
