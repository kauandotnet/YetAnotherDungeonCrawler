use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
#[read_component(Player)]
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut intend_to_move = <(Entity, &WantsToMove)>::query();
    intend_to_move
        .iter(ecs)
        .for_each(|(entity, wants_to_move)| {
            if map.can_enter_tile(wants_to_move.destination) {
                commands.add_component(wants_to_move.entity, wants_to_move.destination);
                if ecs
                    .entry_ref(wants_to_move.entity)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    camera.on_player_move(wants_to_move.destination);
                }
            }
            commands.remove(*entity);
        });
}
