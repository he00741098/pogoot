use crate::requestTypes::*;
use serde_json::to_string;

#[test]
fn test_pogoot_request_serialization(){
    let test_username = "Hi".to_string();
    let test_request = pogootRequest { requestType:request::CreateGame, data:Data::Username(test_username) };
    let test_request2 = pogootRequest { requestType:request::SubscribeToGame("69".to_string()), data:Data::Username("test_username".to_owned()) };
    let magic_result = to_string(&test_request).unwrap();
    let magic_result2 = to_string(&test_request2).unwrap();
    println!("MagicalResult: {}", magic_result);
    println!("MagicalResult2: {}", magic_result2);
    // info!("MagicalResult: {}", magic_result);
}
