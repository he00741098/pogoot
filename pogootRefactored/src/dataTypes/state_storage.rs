use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::mpsc::Sender;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use crate::login::loginRequest;
use crate::pogoot;

use super::{pogootRequest, GameUpdate};

///store state
pub struct state_storage{
    pub login_channel:Sender<loginRequest>,
    //Games, Sender leads to merger thread which then will interact with the game thread
    pub games:Arc<RwLock<HashMap<String, (Sender<pogootRequest>, broadcast::Sender<GameUpdate>)>>>,
    pub commander_portals:Arc<RwLock<HashMap<String, (String,Sender<pogoot::GameCommand>)>>>
}

///secrets for database connections
pub struct secrets{

}
