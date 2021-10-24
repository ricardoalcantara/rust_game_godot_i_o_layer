extern crate gdnative;

use game::Game;
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
        // if let Some(n) = self.node_ref {
        //     unsafe { n.assume_safe().set_position(Vector2::zero()); }
        // }

        self.game.ready().unwrap();

        godot_print!("hello, world.");
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        self.game.update(delta).unwrap();

        let n = unsafe { self.node_ref.unwrap().assume_safe() };
        let pos = self.game.get_first_entity_position();
        n.set_position(pos);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
