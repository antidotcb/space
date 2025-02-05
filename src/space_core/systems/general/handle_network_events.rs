use bevy::{ecs::{system::{Commands, Res, ResMut}}, prelude::{EventReader, EventWriter, Query, info}};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::space_core::{components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, player_input::PlayerInput}, events::net::net_on_new_player_connection::NetOnNewPlayerConnection, functions::entity::{on_new_player_connection::on_new_player_connection, on_player_disconnect::on_player_disconnect}, resources::{authid_i::AuthidI, client_health_ui_cache::ClientHealthUICache, gridmap_data::GridmapData, handle_to_entity::HandleToEntity, server_id::ServerId, tick_rate::TickRate, used_names::UsedNames}};

pub fn handle_network_events(
    mut net: ResMut<NetworkResource>,
    tick_rate : Res<TickRate>,
    mut auth_id_i : ResMut<AuthidI>,
    server_id : Res<ServerId>,
    mut handle_to_entity : ResMut<HandleToEntity>,
    mut commands: Commands,
    mut reader: EventReader<NetworkEvent>,
    mut net_on_new_player_connection : EventWriter<NetOnNewPlayerConnection>,
    mut connected_players : Query<(&mut PersistentPlayerData, &mut ConnectedPlayer, &mut PlayerInput)>,
    mut used_names : ResMut<UsedNames>,
    mut client_health_ui_cache : ResMut<ClientHealthUICache>,
    gridmap_data : Res<GridmapData>,
) {

    for event in reader.iter() {
        
        match event {
            NetworkEvent::Packet(_handle, _packet) => {
                
            },
            NetworkEvent::Connected(handle) => {
                
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs

                match net.connections.get_mut(handle) {
                    Some(connection) => {
                        match connection.remote_address() {
                            Some(remote_address) => {
                                info!(
                                    "Incoming connection on [{}] from [{}]",
                                    handle,
                                    remote_address
                                );
                            }
                            None => {
                                panic!("handle_network_events.rs NetworkEvent::Connected: new connection with a strange remote_address [{}]", handle);
                            }
                        }
                    }
                    None => {
                        panic!("handle_network_events.rs NetworkEvent::Connected: got packet for non-existing connection [{}]", handle);
                    }
                }

                on_new_player_connection(
                    &mut net_on_new_player_connection,
                    handle,
                    &tick_rate,
                    &mut auth_id_i, 
                    &server_id,
                    &mut handle_to_entity,
                    &mut commands,
                    &mut used_names,
                    &gridmap_data,
                );


            }
            
            NetworkEvent::Disconnected(handle) => {
                on_player_disconnect(
                    *handle,
                    &mut handle_to_entity,
                    &mut connected_players,
                    &mut used_names,
                    &mut client_health_ui_cache,
                );
            }
            NetworkEvent::Error(_handle, _err) => {
                //warn!("NetworkEvent error [{}] : {:?}", _handle, _err);
            }
        }
    }
    
}
