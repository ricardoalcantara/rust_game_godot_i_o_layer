use crate::components::{Actor, Collision, Solid};
use hecs::*;
use math::{Rectangle2, Transform2, Vector2};

fn collide_at(rect_a: &Rectangle2, other_rects: &Vec<Rectangle2>) -> bool {
    for other_rect in other_rects {
        if rect_a.intersects(&other_rect) {
            return true;
        }
    }

    false
}

fn query_all_solids_collision(world: &mut World) -> Vec<Rectangle2> {
    let mut solids_collision: Vec<Rectangle2> = Vec::new();

    for (_entity, (collision, transform)) in
        &mut world.query::<With<Solid, (&Collision, &Transform2)>>()
    {
        let rect_collision = Rectangle2::new(
            collision.rectangle.origin + transform.position,
            collision.rectangle.size,
        );

        solids_collision.push(rect_collision);
    }

    solids_collision
}

// https://maddythorson.medium.com/celeste-and-towerfall-physics-d24bd2ae0fc5
pub fn system_update_physics(world: &mut World) {
    let solids_collision = query_all_solids_collision(world);

    for (_entity, (actor, transform, collition)) in
        &mut world.query::<(&mut Actor, &mut Transform2, Option<&Collision>)>()
    {
        // move x
        actor.remainder_x += actor.move_x;
        actor.move_x = 0.0;
        let mut move_x = actor.remainder_x.round() as i32;

        if move_x != 0 {
            actor.remainder_x -= move_x as f32;
            let sign = move_x.signum();

            while move_x != 0 {
                if let Some(collision) = &collition {
                    let next_position = transform.position + Vector2::new(sign as f32, 0.0);
                    let rect_a = Rectangle2::new(
                        collision.rectangle.origin + next_position,
                        collision.rectangle.size,
                    );
                    if !collide_at(&rect_a, &solids_collision) {
                        transform.position.x += sign as f32;
                        move_x -= sign;
                    } else {
                        // onCollide();
                        break;
                    }
                } else {
                    transform.position.x += sign as f32;
                    move_x -= sign;
                }
            }
        }

        // move y
        actor.remainder_y += actor.move_y;
        actor.move_y = 0.0;
        let mut move_y = actor.remainder_y.round() as i32;

        if move_y != 0 {
            actor.remainder_y -= move_y as f32;
            let sign = move_y.signum();

            while move_y != 0 {
                if let Some(collision) = &collition {
                    let next_position = transform.position + Vector2::new(0.0, sign as f32);
                    let rect_a = Rectangle2::new(
                        collision.rectangle.origin + next_position,
                        collision.rectangle.size,
                    );
                    if !collide_at(&rect_a, &solids_collision) {
                        transform.position.y += sign as f32;
                        move_y -= sign;
                    } else {
                        // onCollide();
                        break;
                    }
                } else {
                    transform.position.y += sign as f32;
                    move_y -= sign;
                }
            }
        }
    }

    for (_entity, (solid, transform2d)) in &mut world.query::<(&mut Solid, &mut Transform2)>() {
        solid.remainder_x += solid.move_x;
        solid.move_x = 0.0;
        solid.remainder_y += solid.move_y;
        solid.move_y = 0.0;

        let move_x = solid.remainder_x.round() as i32;
        let move_y = solid.remainder_y.round() as i32;

        if move_x != 0 || move_y != 0 {
            //Loop through every Actor in the Level, add it to
            //a list if actor.IsRiding(this) is true
            // List riding = GetAllRidingActors();

            if let Ok(collision) = &mut world.get_mut::<&mut Collision>(_entity) {
                collision.collidable = false;
            }

            if move_x != 0 {
                solid.remainder_x -= move_x as f32;
                transform2d.position.x += move_x as f32;

                // overlapCheck(actor)
                if false {
                    //Push move_x dir
                } else if false { // is_riding
                     // carry move_x dir
                }
                /*
                if (overlapCheck(actor))
                {
                    //Push right
                    actor.MoveX(this.Right — actor.Left, actor.Squish);
                }
                else if (riding.Contains(actor))
                {
                    //Carry right
                    actor.MoveX(moveX, null);
                }
                */
            }

            if move_y != 0 {
                solid.remainder_y -= move_y as f32;
                transform2d.position.y += move_y as f32;

                // overlapCheck(actor)
                if false {
                    //Push move_y dir
                } else if false { // is_riding
                     // carry move_y dir
                }
            }

            if let Ok(collision) = &mut world.get_mut::<&mut Collision>(_entity) {
                collision.collidable = true;
            }
        }
    }
}
