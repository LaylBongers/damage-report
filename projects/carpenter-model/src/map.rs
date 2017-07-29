use cgmath::prelude::*;
use cgmath::{Vector3, Point3, Point2};
use collision::{Plane, Ray3, Intersect};

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
    pub vertices: Vec<Point3<f32>>,
    pub faces: Vec<Face>,
}

impl Brush {
    pub fn cube(position: Point3<f32>) -> Self {
        // Laid out as a front and back plane, in CCW ordering, seen from the side of the faces
        let vertices = vec!(
            // Front-bottom
            position + Vector3::new(-1.0, -1.0, 1.0), // 0
            position + Vector3::new(1.0, -1.0, 1.0), // 1
            // Front-top
            position + Vector3::new(1.0, 1.0, 1.0), // 2
            position + Vector3::new(-1.0, 1.0, 1.0), // 3
            // Back-bottom
            position + Vector3::new(1.0, -1.0, -1.0), // 4
            position + Vector3::new(-1.0, -1.0, -1.0), // 5
            // Back-top
            position + Vector3::new(-1.0, 1.0, -1.0), // 6
            position + Vector3::new(1.0, 1.0, -1.0), // 7
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

    pub fn check_intersection(&self, ray: Ray3<f32>, brush: &Brush) -> Option<PlaneIntersection> {
        // TODO: Check that the triangle is facing the ray before doing a ray hit
        let values = self.triangles_planes(brush);

        for (triangle, plane) in values {
            if let Some(intersection) = (plane, ray).intersection() {
                let axes = create_axes_for_plane(&plane);
                let origin = Point3::from_vec(plane.n * plane.d);

                // Convert the intersection and triangle points to 2D coordinates
                let p = project_3d_to_2d(intersection, axes, origin);
                let a = project_3d_to_2d(triangle[0], axes, origin);
                let b = project_3d_to_2d(triangle[1], axes, origin);
                let c = project_3d_to_2d(triangle[2], axes, origin);

                // Now that we have 2D coordinates, we need to check if it's within the triangle.
                // We use the Barycentric Technique for this, which I really just copied from
                // reference.

                // Compute vectors
                let v0 = c - a;
                let v1 = b - a;
                let v2 = p - a;

                // Compute dot products
                let dot00 = v0.dot(v0);
                let dot01 = v0.dot(v1);
                let dot02 = v0.dot(v2);
                let dot11 = v1.dot(v1);
                let dot12 = v1.dot(v2);

                // Compute barycentric coordinates
                let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
                let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
                let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

                // Check if point is in triangle
                let is_in = (u >= 0.0) && (v >= 0.0) && (u + v < 1.0);

                if is_in {
                    return Some(PlaneIntersection {
                        distance2: ray.origin.distance2(intersection),
                    })
                }
            }
        }

        None
    }

    pub fn triangles(&self, brush: &Brush) -> Vec<[Point3<f32>; 3]> {
        let mut triangles = Vec::new();

        // Fan-triangulage the face
        // TODO: Optionally support concave faces
        let fan_anchor = brush.vertices[self.indices[0]];
        let mut last_vertex = brush.vertices[self.indices[1]];
        for index in self.indices.iter().skip(2) {
            let vertex = brush.vertices[*index];

            triangles.push([
                fan_anchor,
                last_vertex,
                vertex,
            ]);

            last_vertex = vertex;
        }

        triangles
    }

    fn triangles_planes(&self, brush: &Brush) -> Vec<([Point3<f32>; 3], Plane<f32>)> {
        self.triangles(brush).into_iter().map(|vertices| {
            (vertices, Plane::from_points(vertices[0], vertices[1], vertices[2]).unwrap())
        }).collect()
    }
}

pub struct PlaneIntersection {
    pub distance2: f32,
}

/// Constructs arbitrary axes for a plane, used for 2D bounds checking
fn create_axes_for_plane(plane: &Plane<f32>) -> (Vector3<f32>, Vector3<f32>) {
    // Figure out if we should use an up vector to get a perpendicular or a X+1, it needs to be not
    // a parallel.
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = Vector3::new(1.0, 0.0, 0.0);
    let perp_seed = if plane.n == up { right } else { up };

    // Now use that seed vector to create an perpendicular, then use that to create another
    let x_axis = plane.n.cross(perp_seed);
    let y_axis = plane.n.cross(x_axis);

    (x_axis, y_axis)
}

fn project_3d_to_2d(
    point: Point3<f32>, axes: (Vector3<f32>, Vector3<f32>), origin: Point3<f32>
) -> Point2<f32> {
    let relative_point = point - origin;

    //let separation = plane.n.dot(intersection_relative);
    let x = axes.0.dot(relative_point);
    let y = axes.1.dot(relative_point);

    Point2::new(x, y)
}
