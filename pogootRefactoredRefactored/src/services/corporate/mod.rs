use super::{notecard::storage_controller::NotecardStorageManager, database::Database};

pub struct Coordinator{

}
impl Coordinator{

    pub fn start_all_services(){
        //TODO: deal with user management
        //TODO: Complete all of the database stuff
        //TODO: all notecards to be transfered
        // let notecard_storage_manager =NotecardStorageManager{};
        
        //initialization sequence
        //
        //Init the database
        let database = Database::new(Database::try_to_get_secrets());
        //Init the login/user management service
        //start listening for requests
    }
}
