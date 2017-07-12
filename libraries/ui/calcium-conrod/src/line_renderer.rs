use cgmath::{Vector2, Zero};
use lyon::path::{Path};
use lyon::path_builder::{BaseBuilder};
use lyon::path_builder::math::{point, TypedPoint2D};
use lyon::path_iterator::{PathIterator};
use lyon::tessellation::{StrokeTessellator, VertexBuffers, StrokeOptions, StrokeVertex};
use lyon::tessellation::geometry_builder::{simple_builder};

use conrod::{Color, Point};

use calcium_rendering::{Types};
use calcium_rendering_simple2d::{RenderBatch, DrawVertex};

use util;

pub fn push_lines<T: Types>(
    batch: &mut RenderBatch<T>, color: Color, thickness: f64, points: &[Point],
    half_size: Vector2<f32>,
) {
    // Can't build lines from a point
    if points.len() < 2 { return; }

    // Build up the line path
    let mut path_builder = Path::builder();
    path_builder.move_to(point(
        points[0][0] as f32 + half_size.x,
        -points[0][1] as f32 + half_size.y,
    ));
    for p in points.iter().skip(1) {
        path_builder.line_to(point(
            p[0] as f32 + half_size.x,
            -p[1] as f32 + half_size.y,
        ));
    }
    let path = path_builder.build();

    // Turn the lines into triangles
    // TODO: Avoid the intermediate geometry data by creating our own geometry
    //  builder that directly outputs the vertices we need
    let mut tessellator = StrokeTessellator::new();
    let mut geometry: VertexBuffers<StrokeVertex> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut geometry);
        tessellator.tessellate(
            path.path_iter().flattened(0.05),
            &StrokeOptions::default()
                // Not sure if this is correct behavior but without it the
                //  triangles were too small to show up
                .with_line_width(thickness as f32 * 2.0),
            &mut vertex_builder
        );
    }

    // Finally, add the geometry to the batch
    for i in geometry.indices {
        let v = geometry.vertices[i as usize];

        batch.vertices.push(DrawVertex::new(
            vec_lyon_to_cgmath(v.position),
            Vector2::zero(),
            util::color_conrod_to_calcium(color),
        ));
    }
}

fn vec_lyon_to_cgmath<U>(value: TypedPoint2D<f32, U>) -> Vector2<f32> {
    Vector2::new(value.x, value.y)
}
