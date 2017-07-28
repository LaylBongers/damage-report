use cgmath::{Vector3};

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub brushes: Vec<Brush>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            brushes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brush {
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<Face>,
}

impl Brush {
    pub fn cube() -> Self {
        // Laid out as a front and back plane, in CCW ordering, seen from the side of the faces
        let vertices = vec!(
            // Front-bottom
            Vector3::new(-1.0, -1.0, 1.0), // 0
            Vector3::new(1.0, -1.0, 1.0), // 1
            // Front-top
            Vector3::new(1.0, 1.0, 1.0), // 2
            Vector3::new(-1.0, 1.0, 1.0), // 3
            // Back-bottom
            Vector3::new(1.0, -1.0, -1.0), // 4
            Vector3::new(-1.0, -1.0, -1.0), // 5
            // Back-top
            Vector3::new(-1.0, 1.0, -1.0), // 6
            Vector3::new(1.0, 1.0, -1.0), // 7
        );

        let faces = vec!(
            // Side planes
            Face::square(0, 1, 2, 3),
            Face::square(1, 4, 7, 2),
            Face::square(4, 5, 6, 7),
            Face::square(5, 0, 3, 6),
            // Bottom and top planes
            Face::square(5, 4, 1, 0),
            Face::square(7, 6, 3, 2),
        );

        Brush {
            vertices,
            faces,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Face {
    pub indices: Vec<usize>,
}

impl Face {
    pub fn square(a: usize, b: usize, c: usize, d: usize) -> Self {
        Face {
            indices: vec!(a, b, c, d),
        }
    }

    pub fn normal(&self, brush: &Brush) -> Vector3<f32> {
        if self.indices.len() <= 2 {
            panic!("Invalid plane has less than 3 vertices");
        }

        let u = brush.vertices[self.indices[1]] - brush.vertices[self.indices[0]];
        let v = brush.vertices[self.indices[2]] - brush.vertices[self.indices[0]];

        u.cross(v)
    }
}
