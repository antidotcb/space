use bevy::prelude::{Added, EventWriter, Query};

use crate::space_core::{components::{
        boarding::Boarding,
        connected_player::ConnectedPlayer
    }, events::net::net_on_boarding::NetOnBoarding, resources::network_messages::ReliableServerMessage};

use super::on_setupui::INPUT_NAME_PATH;

pub fn on_boarding(
    query : Query<&ConnectedPlayer,Added<Boarding>>,
    mut net_on_boarding: EventWriter<NetOnBoarding>
) {

    for connected_player_component in query.iter() {

        net_on_boarding.send(NetOnBoarding {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::UIRequestInput(
                "setupUI".to_string(),
                INPUT_NAME_PATH.to_string()
            )
        });

    }

}
