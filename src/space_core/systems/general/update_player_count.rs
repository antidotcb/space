use bevy::prelude::{EventWriter, Query};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_update_player_count::NetUpdatePlayerCount, resources::network_messages::{ReliableServerMessage, ServerConfigMessage}};

pub fn update_player_count(
    connected_players : Query<&ConnectedPlayer>,
    mut events : EventWriter<NetUpdatePlayerCount>,
) {

    let mut connected_players_amount : u16 = 0;

    for connected_player_component in connected_players.iter() {
        if connected_player_component.connected {
            connected_players_amount+=1;
        }
    }

    for connected_player_component in connected_players.iter() {

        if !connected_player_component.connected {
            continue;
        }

        events.send(NetUpdatePlayerCount{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConnectedPlayers(connected_players_amount)),
        });

    }

}
