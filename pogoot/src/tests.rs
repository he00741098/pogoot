use crate::requestTypes::*;
use serde_json::to_string;
use tracing::info;
#[test]
fn serializeTestingForDamienBecauseHeIsNotBigBrainEnoughToJustRunTheCodeInHisBrain(){
    let testUsername = "Hi".to_string();
    let questions = vec![Question{question:"Why".to_string(), answers:vec![(false,"because duh".to_string()),(false, "duh duh".to_string()), (true, "bruh_du".to_string())]},Question{question:"Why".to_string(), answers:vec![(false,"because duh".to_string()),(false, "duh duh".to_string()), (true, "bruh_du".to_string())]}];

    let testRequest = pogootRequest { requestType:request::CreateGame, data:Data::QuestionUpload(questionList { questions }) };
    let magicResult = to_string(&testRequest).unwrap();
    println!("MagicalResult: {}", magicResult);
    info!("MagicalResult: {}", magicResult);

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