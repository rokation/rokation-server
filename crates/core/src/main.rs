use rokation_core::{
    component::sensor::Sensor,
    entity::EntityKind,
    geometry::vector::{Point3, Vec3},
    simulation::Simulation,
    world::World,
};

fn main() {
    let mut world = World::new();
    let sensor_id = world.spawn(EntityKind::Sensor);
    let drone_id = world.spawn(EntityKind::Drone);

    {
        let drone1 = world.get_mut(sensor_id).expect("sensor not found");
        drone1.transform.position = Point3::new(0.0, 0.0, 0.0);
        drone1.sensor = Some(Sensor { range: 50.0 });
    }
    {
        let drone2 = world.get_mut(drone_id).expect("drone not found");
        drone2.transform.position = Point3::new(100.0, 0.0, 0.0);
        drone2.velocity = Vec3::new(-30.0, 0.0, 0.0);
    }

    let mut simulation = Simulation::new(world);

    for tick in 0..5 {
        println!("----- tick {tick} -----");
        let events = simulation.update(1.0);

        for event in events {
            let json = serde_json::to_string(&event).unwrap();
            println!("{json}");
        }
    }
}
