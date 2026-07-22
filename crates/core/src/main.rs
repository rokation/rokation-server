use chrono::Local;
use rokation_core::{error::Result, foundation::position::Position, world::world::World};

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = World::new();

    // 새로운 엔티티 생성 (현재 타입 X)
    let e1_id = world.spawn();
    let e2_id = world.spawn();

    world.insert(e1_id, Position::new(10.0, 10.0, 10.0))?;
    world.insert(e2_id, Position::new(10.0, 10.0, 10.0))?;

    let result = world.get::<Position>(e1_id);
    result.is_some().then(|| println!("{:?}", result));

    println!(
        "[{}] The World is started now! - Entity count = {}",
        Local::now(),
        world.len()
    );

    Ok(())
}
