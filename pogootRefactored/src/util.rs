use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{dataTypes::{Data, self, pogootRequest, pogootResponse, requestType, responseType}, login::loginRequest};
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

    pub fn parse_msg_to_pogoot(msg:Message)->Result<pogootRequest,pogootResponse>{
        if let Message::Text(msg) = msg{
            let from_str = serde_json::from_str(&msg);
            if from_str.is_err(){Err(pogootResponse::standard_error_message("Parse to request failed"))}else{
                Ok(from_str.unwrap())
            }
        }else{
            Err(pogootResponse::standard_error_message("Message not text"))
        }
    }
    pub fn websocket_message_wrap(response:pogootResponse)->Message{
        Message::Text(Self::to_string_or_default(response.clone(), &*format!("{:?}", response)))
    }
    pub fn unpack_token_verify(request:pogootRequest)->Result<String, pogootResponse>{
        match request.request{
            requestType::VerifyToken=>{
                match request.data{
                    Data::VerifyToken(token)=>{Ok(token)},
                    _=>{Err(pogootResponse::standard_error_message("Not Verify Token Data"))}
                }
            },
            _=>{Err(pogootResponse::standard_error_message("Request is not VerifyToken"))}
        }
    }
    pub fn sort_player_list(player_list:&mut Vec<(String, String, usize)>){
        player_list.sort_by(|a,b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    }
    pub fn get_relevant_data(player_list:&mut Vec<(String, String, usize)>, username:&str, token:&str)->pogootResponse{
        Self::sort_player_list(player_list);
        for i in 0..player_list.len(){
            if player_list[i].1==username && player_list[i].0==token{
                let mut in_front = "".to_string();
                let mut in_front_points = 0;
                if i>0{
                    in_front = player_list[i-1].1.clone();
                    in_front_points = player_list[i-1].2;
                }
                let cur_point = player_list[i].2;
                return pogootResponse{response:responseType::gameUpdateResponse, data:Data::gameUpdateData(cur_point, in_front, in_front_points)};
            }
        }
        pogootResponse::standard_error_message("Player Not Found In Player List")
    }
    pub fn map_answer_to_internal(request:pogootRequest, username:&str, token:&str)->Result<pogootRequest, pogootResponse>{
        if let Data::AnswerData(id, answer) = request.data{
            return Ok(pogootRequest{request:requestType::InternalAnswer, data:Data::InternalAnswerData(username.to_string(), token.to_string(), id, answer)});
        }
        Err(pogootResponse::standard_error_message("Could not parse to internal answer"))
    }
}

#[test]
fn test_sort_player_list(){
    let mut playerlist = vec![("tokener".to_string(), "poggers".to_string(), 800),("tokener2".to_string(), "poggers2".to_string(), 900),("tokener3".to_string(), "poggers3".to_string(), 700)];
    util::sort_player_list(&mut playerlist);
    println!("Playerlist: {:?}", playerlist);
}
