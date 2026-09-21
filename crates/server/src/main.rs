use rokation_core::{command::Command, entity::EntityKind, runtime::Runtime, world::World};

fn main() {
    let world = World::new();
    let mut runtime = Runtime::new(world);

    runtime.start();

    let entity_id = runtime.execute(Command::SpawnEntity {
        kind: EntityKind::Drone,
    });

    println!("runtime started: {}", runtime.is_running());
    println!("world: {}", runtime.world_id());
    println!("spawn result: {:?}", entity_id);
}
