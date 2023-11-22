use tokio::sync::mpsc::Sender;

use crate::login::loginRequest;

///store state
pub struct state_storage{
    pub login_channel:Sender<loginRequest>
}

///secrets for database connections
pub struct secrets{

}
