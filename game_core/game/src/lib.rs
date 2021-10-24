use hecs::World;
use math::Vector2;
use physics::systems::system_update_physics;

pub struct Player;

pub struct Game {
    world: World,
    total_delta: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            total_delta: 0.0,
        }
    }

    pub fn ready(&mut self) -> std::result::Result<(), ()> {
        self.world.spawn((Player, Vector2::new(300.0, 200.0)));

        Ok(())
    }

    pub fn update(&mut self, delta: f32) -> std::result::Result<(), ()> {
        self.total_delta += delta;

        system_update_physics(&mut self.world);

        for (_, position) in &mut self.world.query::<&mut Vector2>() {
            if self.total_delta.sin() > 0.0 {
                *position += Vector2::new(10.0, 0.0) * delta * 10.0;
            } else {
                *position += Vector2::new(-10.0, 0.0) * delta * 10.0;
            }
        }
        Ok(())
    }

    pub fn get_first_entity_position(&self) -> Vector2 {
        self.world
            .query::<&Vector2>()
            .iter()
            .next()
            .unwrap()
            .1
            .clone()
    }
}
