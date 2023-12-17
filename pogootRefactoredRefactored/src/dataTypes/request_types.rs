use serde::{Serialize, Deserialize};

///Payload types. All payloads must have a type present in this enum.
///TODO: Figure out better solution for payloads
#[derive(Deserialize, Serialize)]
pub enum PayloadType{
    ///Login datatype
    Login,
    ///Login register type
    Register,
    ///Game type. Intended for all game requests.
    Game,
    ///Note card type. Intended for all notecards
    Note
}
#[derive(Deserialize, Serialize, Default)]
pub enum PayloadData{
    #[default]
    None,
    ///Intended only for clients to send to server. The login request is composed of 2 strings in
    ///the order (Username, Password, Ip)
    LoginRequest(String, String, String),
    ///The Login Response is intened only for the server to send to the client. It is a response
    ///that contains a error/success message along with a Optional token
    LoginResponse(String, Option<String>),
    ///The Register Request is intended only for clients to send to the server. The register
    ///request is composed of (Username, Password, Ip)
    RegisterRequest(String, String, String),
    ///The register response contains (error/success message, Optinal Token)
    RegisterResponse(String, Option<String>)
}
impl PayloadData{
    pub fn grab_login_request_data(self)->Result<(String, String, String), ()>{
        if let Self::LoginRequest(username, password, ip)=self{
            return Ok((username, password, ip));
        }else{
            return Err(())
        }
    }
    pub fn grab_login_response_data(self)->Result<(String, Option<String>), ()>{
        if let Self::LoginResponse(message, token)=self{
            return Ok((message, token));
        }else{
            return Err(())
        }
    }
    pub fn grab_register_request_data(self)->Result<(String, String, String),()>{
        if let Self::RegisterRequest(username, password, ip) = self{
            return Ok((username, password, ip))
        }else{
            return Err(())
        }
    }
    pub fn grab_register_response_data(self)->Result<(String, Option<String>), ()>{
        if let Self::RegisterResponse(message, token)=self{
            return Ok((message, token));
        }else{
            return Err(())
        }
    }
}
///Payload trait. This is supposed to contain all the data
pub trait Payload{
    fn get_type(&self)->PayloadType;
    fn get_payload(&mut self)->PayloadData;
}
pub trait Request{
    fn get_payload(&mut self)->PayloadData;
    fn get_type(&self)->PayloadType;
}
pub trait Response{
    fn get_payload(&mut self)->PayloadData;
    fn get_type(&self)->PayloadType;
}
pub mod login{
    use serde::{Deserialize, Serialize};
    use super::{Request, Response, PayloadType, Payload, PayloadData};
    ///Login Response used to make a response to the Login Request. Intended only to be sent to the
    ///client
    #[derive(Deserialize, Serialize)]
    pub struct LoginResponse{
        pub payload:LoginResponsePayload,
    }
    ///Login Request used to make a request to the server to login. Intended only to be sent to the
    ///server
    #[derive(Deserialize, Serialize)]
    pub struct LoginRequestPayload{
        pub payload:PayloadData,
    }
    #[derive(Deserialize, Serialize)]
    pub struct LoginResponsePayload{
        pub payload:PayloadData,
    }

    impl Payload for LoginRequestPayload{
        fn get_type(&self)->PayloadType{
            PayloadType::Login
        }
        fn get_payload(&mut self)->PayloadData {
            std::mem::take(&mut self.payload)
        }
    }
    impl Payload for LoginResponsePayload{
        fn get_type(&self)->PayloadType{
            PayloadType::Login
        }
        fn get_payload(&mut self)->PayloadData {
            std::mem::take(&mut self.payload)
        }
    }
    #[derive(Deserialize, Serialize)]
    pub struct LoginRequest{
        pub payload:LoginRequestPayload,
    }
    // impl LoginRequest{

    // }
    // impl LoginResponse{

    // }
    impl Response for LoginResponse{
        fn get_payload(&mut self)->PayloadData {
            self.payload.get_payload()
        }
        fn get_type(&self)->PayloadType {
            self.payload.get_type()
        }
    }
    impl Request for LoginRequest{
        fn get_payload(&mut self)->PayloadData {
            self.payload.get_payload()
        }
        fn get_type(&self)->PayloadType {
            self.payload.get_type()
        }
    }
}
pub mod register{
    use serde::{Deserialize, Serialize};
    use super::{Request, Response, PayloadType, PayloadData, Payload};
    #[derive(Deserialize, Serialize)]
    pub struct RegisterResponse{
        pub payload:RegisterResponsePayload
    }
    #[derive(Deserialize, Serialize)]
    pub struct RegisterRequest{
        pub payload:RegisterRequestPayload
    }
    #[derive(Deserialize, Serialize)]
    pub struct RegisterRequestPayload{
        payload:PayloadData
    }
    #[derive(Deserialize, Serialize)]
    pub struct RegisterResponsePayload{
        payload:PayloadData
    }
    impl Payload for RegisterRequestPayload{
        fn get_type(&self)->PayloadType {
            PayloadType::Register
        }
        fn get_payload(&mut self)->PayloadData {
            std::mem::take(&mut self.payload)
        }
    }
    impl Payload for RegisterResponsePayload{
        fn get_payload(&mut self)->PayloadData {
            std::mem::take(&mut self.payload)
        }
        fn get_type(&self)->PayloadType {
            PayloadType::Register
        }
    }
    impl Response for RegisterResponse{
        fn get_type(&self)->PayloadType {
            self.payload.get_type()
        }
        fn get_payload(&mut self)->PayloadData {
            self.payload.get_payload()
        }
    }
    impl Request for RegisterRequest{
        fn get_payload(&mut self)->PayloadData {
            self.payload.get_payload()
        }
        fn get_type(&self)->PayloadType {
            self.payload.get_type()
        }
    }
}

pub mod game{
    use serde::{Deserialize, Serialize};
    use super::{Request, Response};
}

pub mod notecard{
    use serde::{Deserialize, Serialize};
    use super::{Request, Response};
}
