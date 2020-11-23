use crate::net::message::Message;
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RootMessageType {
    HOST_GAME = 0x00,
    JOIN_GAME,
    START_GAME,
    REMOVE_GAME,
    REMOVE_PLAYER,
    GAME_DATA,
    JOINED_GAME,
    END_GAME,
    GET_GAME_LIST,
    ALTER_GAME,
    KICK_PLAYER,
    WAIT_FOR_HOST,
    REDIRECT,
    RESELECT_SERVER,
    GET_GAME_LIST_V2,
}

impl TryFrom<u8> for RootMessageType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == RootMessageType::HOST_GAME as u8 => Ok(RootMessageType::HOST_GAME),
            x if x == RootMessageType::JOIN_GAME as u8 => Ok(RootMessageType::JOIN_GAME),
            x if x == RootMessageType::START_GAME as u8 => Ok(RootMessageType::START_GAME),
            x if x == RootMessageType::REMOVE_GAME as u8 => Ok(RootMessageType::REMOVE_GAME),
            x if x == RootMessageType::REMOVE_PLAYER as u8 => Ok(RootMessageType::REMOVE_PLAYER),
            x if x == RootMessageType::GAME_DATA as u8 => Ok(RootMessageType::GAME_DATA),
            x if x == RootMessageType::JOINED_GAME as u8 => Ok(RootMessageType::JOINED_GAME),
            x if x == RootMessageType::END_GAME as u8 => Ok(RootMessageType::END_GAME),
            x if x == RootMessageType::GET_GAME_LIST as u8 => Ok(RootMessageType::GET_GAME_LIST),
            x if x == RootMessageType::ALTER_GAME as u8 => Ok(RootMessageType::ALTER_GAME),
            x if x == RootMessageType::KICK_PLAYER as u8 => Ok(RootMessageType::KICK_PLAYER),
            x if x == RootMessageType::WAIT_FOR_HOST as u8 => Ok(RootMessageType::WAIT_FOR_HOST),
            x if x == RootMessageType::REDIRECT as u8 => Ok(RootMessageType::REDIRECT),
            x if x == RootMessageType::RESELECT_SERVER as u8 => {
                Ok(RootMessageType::RESELECT_SERVER)
            }
            x if x == RootMessageType::GET_GAME_LIST_V2 as u8 => {
                Ok(RootMessageType::GET_GAME_LIST_V2)
            }
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
pub struct GameOptionsData {
    pub version: u8,
    pub max_players: u8,
    pub keywords: u32,
    pub maps: u8,
    pub player_speed_mod: f32,
    pub cewmate_vision_mod: f32,
    pub impostor_vision_mod: f32,
    pub kill_cooldown: f32,
    pub n_common_tasks: u8,
    pub n_long_tasks: u8,
    pub n_short_tasks: u8,
    pub n_emergency_meetings: u32,
    pub n_imposters: u8,
    pub kill_distance: u8,
    pub discussion_time: u32,
    pub voting_time: u32,
    pub is_defaults: bool,
    pub emergency_cooldown: u8,
    pub confirm_ejects: bool,
    pub visual_tasks: bool,
    pub anonymous_votes: bool,
    pub task_bar_updates: u8,
}

impl GameOptionsData {
    pub fn parse(m: &Message) -> GameOptionsData {
        GameOptionsData {
            version: m.data[1],
            max_players: m.data[2],
            keywords: u32::from_le_bytes([m.data[3], m.data[4], m.data[5], m.data[6]]),
            maps: m.data[7],
            player_speed_mod: f32::from_le_bytes([m.data[8], m.data[9], m.data[10], m.data[11]]),
            cewmate_vision_mod: f32::from_le_bytes([
                m.data[12], m.data[13], m.data[14], m.data[15],
            ]),
            impostor_vision_mod: f32::from_le_bytes([
                m.data[16], m.data[17], m.data[18], m.data[19],
            ]),
            kill_cooldown: f32::from_le_bytes([m.data[20], m.data[21], m.data[22], m.data[23]]),
            n_common_tasks: m.data[24],
            ..GameOptionsData::default()
        }
    }
}
