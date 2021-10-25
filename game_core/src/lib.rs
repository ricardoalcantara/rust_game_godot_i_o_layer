extern crate gdnative;

use game::{Events, Game, InputType};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld {
    node_ref: Option<Ref<Node2D>>,
    game: Game,
}

#[methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld {
            node_ref: None,
            game: Game::new(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let x = unsafe { owner.get_node_as::<Node2D>("Sprite").unwrap().claim() };
        self.node_ref = Some(x);

        self.game.ready().unwrap();

        godot_print!("hello, world.");
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        let inputs = HelloWorld::get_inputs();
        self.game.add_actions(inputs);
        self.game.update(delta).unwrap();

        for event in self.game.get_events() {
            match event {
                Events::PlayerUpdate(pos) => {
                    let n = unsafe { self.node_ref.unwrap().assume_safe() };
                    n.set_position(pos);
                }
            }
        }
    }

    fn get_inputs() -> Vec<InputType> {
        let mut inputs = Vec::new();
        let input = Input::godot_singleton();
        if Input::is_action_pressed(input, "move_left") {
            inputs.push(InputType::ActionPressed("move_left"))
        }
        if Input::is_action_pressed(input, "move_right") {
            inputs.push(InputType::ActionPressed("move_right"))
        }
        if Input::is_action_pressed(input, "move_up") {
            inputs.push(InputType::ActionPressed("move_up"))
        }
        if Input::is_action_pressed(input, "move_down") {
            inputs.push(InputType::ActionPressed("move_down"))
        }

        inputs
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
