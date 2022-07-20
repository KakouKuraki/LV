use objects;

type Vector3 = cgmath::Vector3<f32>;
type Vector4 = cgmath::Vector4<f32>;

pub enum ObjectKind {
    Sphere,
    Cube,
    Tetrahedron,
    Circle,
    Square,
    Triangle,
    Image,
    Str,
}

pub struct Object {
    kind: ObjectKind,
    start: i32,
    end: i32,
    x_start: f32,
    x_end: f32,
    y_start: f32,
    y_end: f32,
    z_start: f32,
    z_end: f32,
    rx_start: f32,
    rx_end: f32,
    ry_start: f32,
    ry_end: f32,
    rz_start: f32,
    rz_end: f32,
    sx_start: f32,
    sx_end: f32,
    sy_start: f32,
    sy_end: f32,
    sz_start: f32,
    sz_end: f32,
    cx_start: f32,
    cx_end: f32,
    cy_start: f32,
    cy_end: f32,
    cz_start: f32,
    cz_end: f32,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

// objects must be ordered by time!
pub struct Layer {
    objects: Vec<Object>,
}

pub struct TimeLine {
    layers: Vec<Layer>,
}

impl Object {
    pub fn gen(self, frame_num: i32) -> impl objects::Object {
        let move_ratio = (frame_num - self.start) as f32 / (self.end - self.start) as f32;
        let x = (self.x_end - self.x_start) * move_ratio + self.x_start;
        let y = (self.y_end - self.y_start) * move_ratio + self.y_start;
        let z = (self.z_end - self.z_start) * move_ratio + self.z_start;
        let rx = (self.rx_end - self.rx_start) * move_ratio + self.rx_start;
        let ry = (self.ry_end - self.ry_start) * move_ratio + self.ry_start;
        let rz = (self.rz_end - self.rz_start) * move_ratio + self.rz_start;
        let sx = (self.sx_end - self.sx_start) * move_ratio + self.sx_start;
        let sy = (self.sy_end - self.sy_start) * move_ratio + self.sy_start;
        let sz = (self.sz_end - self.sz_start) * move_ratio + self.sz_start;
        let cx = (self.cx_end - self.cx_start) * move_ratio + self.cx_start;
        let cy = (self.cy_end - self.cy_start) * move_ratio + self.cy_start;
        let cz = (self.cz_end - self.cz_start) * move_ratio + self.cz_start;
        match self.kind {
            Sphere => {
                let sphere = objects::Sphere::new();
                sphere.rescale_x(sx);
                sphere.rescale_x(sy);
                sphere.rescale_x(sz);
                sphere.recolor(Vector4::new(self.red, self.green, self.blue, self.alpha));
                sphere.generate_nodes();
                sphere.translocate(Vector3::new(x, y, z));
                sphere.rotate_x(rx, cy, cz);
                sphere.rotate_y(ry, cx, cz);
                sphere.rotate_z(rz, cx, cy);
                sphere
            },
        }
    }
}

impl Layer {
    pub fn gen(self, frame_num: i32) -> Option(impl objects::Object) {
        let existance = self.is_exist(frame_num);
        if existance.0 {
            return Some(self.objects[existance.1].gen(frame_num));
        }
        None
    }

    fn is_exist(self, frame_num: i32) -> (bool, i32) {
        let mut object_index: i32 = 0;
        for object in self.objects {
            if object.end > frame_num {
                break;
            }
            object_index += 1;
        }
        (self.objects[object_index] < frame_num, object_index)
    }
}

impl TimeLine {
    pub fn gen(self, frame_num: i32) -> Vec<impl objects::Object> {
        let mut objects = vec![];
        for layer in self.layers {
            let may_object = layer.gen(frame_num);
            if let Some(object) = may_object {
                objects.push(object.gen(frame_num));
            }
        }
        objects
    }
}
