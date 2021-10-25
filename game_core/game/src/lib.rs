use std::mem::replace;

use hecs::{With, World};
use math::{Transform2, Vector2};
use physics::{components::Actor, systems::system_update_physics};

pub struct Player;

pub enum InputType {
    ActionPressed(&'static str),
    ActionReleased(&'static str),
    ActionJustPressed(&'static str),
    ActionJustReleased(&'static str),
}

pub struct Game {
    world: World,
    total_delta: f32,
    inputs: Option<Vec<InputType>>,
    events: Vec<Events>,
}

pub enum Events {
    PlayerUpdate(Vector2),
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            total_delta: 0.0,
            inputs: None,
            events: Vec::new(),
        }
    }

    pub fn ready(&mut self) -> std::result::Result<(), ()> {
        let pp = Vector2::new(300.0, 200.0);
        self.world
            .spawn((Player, Actor::default(), Transform2::new(pp)));
        self.events.push(Events::PlayerUpdate(pp));
        Ok(())
    }

    pub fn update(&mut self, delta: f32) -> std::result::Result<(), ()> {
        self.total_delta += delta;

        let inputs = std::mem::replace(&mut self.inputs, None);

        if let Some(inputs) = inputs {
            let mut direction = Vector2::zero();

            for input in inputs {
                match input {
                    InputType::ActionPressed(action) => {
                        if action == "move_left" {
                            direction.x -= 1.0;
                        }
                        if action == "move_right" {
                            direction.x += 1.0;
                        }
                        if action == "move_up" {
                            direction.y -= 1.0;
                        }
                        if action == "move_down" {
                            direction.y += 1.0;
                        }
                    }
                    _ => (),
                }
            }

            if direction.square_length() > 0.0 {
                direction = direction.normalize() * 100.0 * delta;

                for (_, actor) in &mut self.world.query::<With<Player, &mut Actor>>() {
                    actor.move_x = direction.x;
                    actor.move_y = direction.y;
                }
            }
        }

        system_update_physics(&mut self.world);

        // too much events
        for (_, transform) in &mut self.world.query::<With<Player, &Transform2>>() {
            self.events.push(Events::PlayerUpdate(transform.position));
        }

        Ok(())
    }

    pub fn add_actions(&mut self, inputs: Vec<InputType>) {
        self.inputs = Some(inputs);
    }

    pub fn get_events(&mut self) -> Vec<Events> {
        std::mem::replace(&mut self.events, Vec::new())
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
