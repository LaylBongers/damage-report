use slog::{Logger};
use cgmath::{Vector2, Point2};
use tiled::{Map as TMap, ObjectShape};

use model::{TilesLayer, Object};

pub struct Map {
    layers: Vec<TilesLayer>,
    objects: Vec<Object>,
}

impl Map {
    pub fn new(map: &TMap, log: &Logger) -> Self {
        let size = Vector2::new(map.width, map.height);

        // Create tile layers for the layers in the map
        let mut layers = Vec::new();
        for layer in &map.layers {
            layers.push(TilesLayer::new(&layer, size));
        }

        // Go over all the objects in object groups to start loading in entities
        let mut objects = Vec::new();
        for object_group in &map.object_groups {
            for object in &object_group.objects {
                // We need to get the width and height of the object, because some objects have
                // surface area, rather than just being a sprite at a point
                let (width, height) = match object.shape {
                    ObjectShape::Rect { width, height } => (width, height),
                    _ => {
                        warn!(log, "Object found in map with non-rect object shape, not supported");
                        continue
                    }
                };

                // The type defines what we should do with it
                match object.obj_type.as_str() {
                    // Blank should be just collision, easy to quickly place
                    "" => {
                        objects.push(Object::new(
                            Point2::new(object.x, object.y),
                            Vector2::new(width, height),
                        ))
                    },
                    // We couldn't identify this one, so print a warning
                    unknown => warn!(log, "Object found in map with unknown type \"{}\"", unknown),
                }
            }
        }

        Map {
            layers,
            objects,
        }
    }

    pub fn layers(&self) -> &Vec<TilesLayer> {
        &self.layers
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }
}
