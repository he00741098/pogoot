use crate::requestTypes::*;
use serde_json::to_string;
use tracing::info;

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
#[test]
fn joinGameTest(){
    let gameJoin = pogootRequest{requestType:request::SubscribeToGame("721174".to_string()), data:Data::Username("Poggooooo".to_string())};
    let magicResult = to_string(&gameJoin).unwrap();
    println!("MagicalResult2: {}", magicResult);
    info!("MagicalResult2: {}", magicResult);
}

//Expected orderings of requests
//1. pogoot request - create game, subscribe to game, etc
//2.