use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{dataTypes::{Data, self, pogootRequest, pogootResponse}, login::loginRequest};
pub struct util{


}
impl util{

    pub fn to_string_or_default<T>(thing:T, default:&str) -> String where T:for<'a> Deserialize<'a> + Serialize + Sized{
            let string = to_string(&thing);
            match string{
                Ok(x)=>x,
                _=>default.to_string()
            }
    }
    pub fn standard_error(msg:&str)->String{
        let error = pogootResponse::standard_error_message(msg);
        Self::to_string_or_default(error, msg)
    }
    pub fn verify_data_is_login(data:&Data)->bool{
        match data{
            Data::LoginData(_,_)=>true,
            _=>false
        }
    }
    pub fn verify_data_is_temp(data:&Data)->bool{
        match data{
            Data::TempData(_)=>true,
            _=>false
        }
    }

    pub fn verify_data_is_register(data:&Data)->bool{
        match data{
            Data::RegisterData(_,_)=>true,
            _=>false
        }
    }
    // pub fn parse_pogoot_request_to_login_request(req:pogootRequest)->loginRequest{

    // }
    pub fn unpack_login_data(data:Data)->Result<(String, String), pogootResponse>{
        if let Data::LoginData(username, password) = data{
            return Ok((username, password))
        }else if let Data::RegisterData(username, password) = data{
            return Ok((username, password))
        }else if let Data::TempData(username) = data{
            return Ok((username, String::with_capacity(0)))
        }
        
        return Err(pogootResponse::standard_error_message("Not login request"))
    }

}
