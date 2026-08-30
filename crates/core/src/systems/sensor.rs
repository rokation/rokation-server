use crate::{geometry::mbr::Mbr, sensor::detection::Detection, world::World};

pub struct SensorSystem;

impl SensorSystem {
    pub fn update(world: &World) -> Vec<Detection> {
        let mut detections = Vec::new();

        for sensor_entity in world.entities.values() {
            let Some(sensor) = &sensor_entity.sensor else {
                continue;
            };

            let position = sensor_entity.transform.position;
            let query = Mbr::from_radius(position, sensor.range);

            let candidates = world.spatial_index.search(&query);

            for target_id in candidates {
                if target_id == sensor_entity.id {
                    continue;
                }

                let Some(target) = world.entities.get(&target_id) else {
                    continue;
                };

                let distance = position.distance(&target.transform.position);

                if distance <= sensor.range {
                    detections.push(Detection {
                        sensor_id: sensor_entity.id,
                        target_id,
                        distance,
                    })
                }
            }
        }

        detections
    }
}
