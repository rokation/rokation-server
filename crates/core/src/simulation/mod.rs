use crate::{
    event::{bus::EventBus, simulation::SimulationEvent},
    systems::{movement::MovementSystem, sensor::SensorSystem},
    world::World,
};

pub struct Simulation {
    pub world: World,
    event_bus: EventBus,
    tick: u64,
    elapsed: f64,
}

impl Simulation {
    pub fn new(world: World) -> Self {
        Self {
            world,
            event_bus: EventBus::new(),
            tick: 0,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) -> Vec<SimulationEvent> {
        self.tick += 1;
        self.elapsed += dt;

        MovementSystem::update(&mut self.world, dt);
        self.world.sync_spatial_index();

        let detections = SensorSystem::update(&mut self.world);
        for detection in detections {
            self.event_bus
                .publish(SimulationEvent::Detection(detection));
        }

        self.event_bus.drain()
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }
}
