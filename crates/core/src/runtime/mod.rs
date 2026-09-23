use std::time::Duration;

use crate::{
    command::{Command, CommandResult},
    component::lla::Lla,
    entity::EntitySnapshot,
    event::Event,
    world::{World, WorldId},
};

pub struct Runtime {
    pub world: World,
    pub running: bool,
}

impl Runtime {
    pub fn new(world: World) -> Self {
        Self {
            world,
            running: false,
        }
    }

    pub fn execute(&mut self, command: Command) -> CommandResult {
        self.world.execute(command)
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn update(&mut self, dt: f64) {
        self.world.update(dt);
    }

    pub fn consume_events(&mut self) -> Vec<Event> {
        self.world.consume_event()
    }

    pub fn snapshots(&self) -> Vec<EntitySnapshot> {
        self.world.snapshots()
    }

    pub fn world_id(&self) -> WorldId {
        self.world.id()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn tick(&mut self, dt: f64) {
        if !self.running {
            return;
        }

        self.update(dt);
    }

    pub fn tick_rate() -> Duration {
        Duration::from_millis(16)
    }

    pub fn origin(&self) -> Lla {
        self.world.origin()
    }

    pub fn set_origin(&mut self, origin: Lla) {
        self.world.set_origin(origin)
    }

    pub fn entity_count(&self) -> usize {
        self.world.entities_count()
    }
}
