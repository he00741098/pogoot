use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Permissions{
    can_upload_notecards:bool,
    can_access_notecards:bool,
    can_pogoot:bool,
    can_access_website:bool,
}
