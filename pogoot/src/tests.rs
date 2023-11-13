use crate::requestTypes::*;
use serde_json::to_string;
use tracing::info;
#[test]
fn serializeTestingForDamienBecauseHeIsNotBigBrainEnoughToJustRunTheCodeInHisBrain(){
    let testUsername = "Hi".to_string();
    let testRequest = pogootRequest { requestType:request::CreateGame, data:Data::Username(testUsername) };
    let magicResult = to_string(&testRequest).unwrap();
    println!("MagicalResult: {}", magicResult);
    info!("MagicalResult: {}", magicResult);

}
