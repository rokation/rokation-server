use crate::world::World;

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(world: &mut World, dt: f64) {
        for entity in world.entities_mut() {
            entity.transform.position += entity.velocity * dt;
        }
    }
}
