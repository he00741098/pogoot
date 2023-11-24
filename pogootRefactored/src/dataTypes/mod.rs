use std::time::Duration;

use axum::extract::ws::WebSocket;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
pub mod database;
pub mod config;
pub mod state_storage;
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct pogootRequest{
    pub request:requestType,
    pub data:Data

}

impl pogootRequest{
    pub fn is_answer(&self)->bool{
        match self.request{
            requestType::Answer=>{
                match self.data{
                    Data::AnswerData(_, _)=>{true},
                    _=>false,
                }
            },
            _=>{
                false
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct pogootResponse{
    pub response:responseType,
    pub data:Data
}
impl pogootResponse{
    pub fn standard_error_message(message:&str)->Self{
        pogootResponse { response: responseType::errorResponse, data: Data::StandardErrorData(format!("{}", message)) }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Data{
    ///Data for create game, requires a valid list of questions
    CreateGameData(questionList),
    ///Start Game data, not required.
    StartGameData,
    ///Subscribe Game Data, requires GID
    SubscribeToGameData(String),
    ///Answer Data, should be pushed through websocket, contains a number in 0..answers.len(),
    ///question Id, answer Id
    ///Username, Token
    AnswerData(usize, usize),
    ///username, token, question id, answer id
    InternalAnswerData(String, String, usize, usize),
    ///ReSub Data, contains a Token and a GID
    ReSubscribeData(String, String),
    ///Login Request Data - Username, Password
    LoginData(String, String),
    ///Register Request Data - Username, Password
    RegisterData(String, String),
    ///Contains error message
    StandardErrorData(String),
    ///Contains game Id and Game Password
    GameCreationSuccessData(String, String),
    ///Contains error message
    GameCreationErrorData(String),
    ///Contains error message
    GameNotFoundErrorData(String),
    ///Data for temporary logins
    TempData(String),
    AnonData(String),
    ///Post the token to the server for initial verification
    VerifyToken(String),
    ///For use with the log in phase of the websocket
    SocketVerified,
    ///New point count, Person in front, Point count difference
    gameUpdateData(usize, String, usize),
    QuestionData(censoredQuestion),
    LeaderBoardUpdate(Vec<(String, String, usize)>),
    GamePlayerTimeUpdate(usize, Duration),
    NextGameData,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum requestType{
    ///Create new Game, Data required for question upload
    CreateGame,
    ///Start game, data requires a game ID and starts the Game
    StartGame,
    ///Subscribe to game, data takes a game ID and subscribes to the game. Data Required for Username
    SubscribeToGame,
    ///Answer takes in a number between 0..answers.len(), no data
    Answer,
    InternalAnswer,
    ///Resub if discconnected
    ReSubscribeToGame,
    ///Login
    Login,
    ///Register
    Register,
    ///Temporary Login for no Login Games
    Temp,
    ///Anonomous login for anon games
    Anon,
    ///Token verify request
    VerifyToken,
    NextQuestion,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum responseType{
    ///Standard error response, Standard error data contains a string
    errorResponse,
    ///standard success response, standard success data contains a string
    successResponse,
    ///game creation response, success, Game Creation data contains response token
    gameCreatedSuccessResponse,
    ///Game Creation Error, game creation response data, contains string
    gameCreationErrorResponse,
    ///Game not found Error, contains string
    gameNotFoundErrorResponse,
    ///Login response, data contains boolean
    loginResponse,
    ///Register response, data contains boolean
    registerResponse,
    ///Update the player on their current score and stuff
    gameUpdateResponse,
    questionResponse,
    GameTimeUpdateResponse,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct questionList{
    pub questions:Vec<Question>
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Question{
    pub question:String,
    pub answers:Vec<(bool, String)>
}

impl Question{
    pub fn censored(self)->censoredQuestion{
        censoredQuestion { question: self.question, answers: self.answers.iter().map(|x|x.1.clone()).collect::<Vec<String>>() }
    }
    pub fn new(question:String, answers:Vec<(bool, String)>)->Self{
        Question{question, answers}
    }
    pub fn isValid(&self)->bool{
        //TODO
        if self.answers.iter().filter(|x|x.0).collect::<Vec<&(bool, String)>>().len()>0{
            true
        }else{
            false
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct censoredQuestion{
    pub question:String,
    pub answers:Vec<String>
}

#[derive(Clone, Debug)]
pub struct GameUpdate{
    pub update_version:usize,
    pub data:gameUpdateHelper
}

#[derive(Clone, Debug)]
pub enum gameUpdateHelper{
    newQuestion(censoredQuestion, usize),
    ///username, token, score
    playerListUpdate(Vec<(String, String, usize)>)
}
impl GameUpdate{
    ///Takes a number and tells you if the game update is recent
    pub fn is_new(&self, last_req_ver:usize)->bool{
        if last_req_ver>=self.update_version{
            return false;
        }
        true
    }
    pub fn is_player_list(&self)->bool{
        match self.data{
            gameUpdateHelper::playerListUpdate(_)=>true,
            _=>false
        }
    }
    pub fn get_player_list(self)->Result<Vec<(String, String, usize)>, pogootResponse>{
        if self.is_player_list(){
            if let gameUpdateHelper::playerListUpdate(list) = self.data{
                return Ok(list);
            }else{
                return Err(pogootResponse::standard_error_message("Data is not a list"));
            }
        }else{
            return Err(pogootResponse::standard_error_message("Not a player list update"));
        }
    }
    pub fn get_question(self)->Result<censoredQuestion, pogootResponse>{
        if !self.is_player_list(){
            if let gameUpdateHelper::newQuestion(question, question_id) = self.data{
                return Ok(question);
            }else{
                return Err(pogootResponse::standard_error_message("Not question"));
            }
        }else{
            return Err(pogootResponse::standard_error_message("Is not question"));
        }
    }
}


#[test]
fn pogoot_request_json(){
    println!("Create Game Json");
    let mut questions = vec![];
    for i in 0..10{
        let temp_question = Question{question:format!("What is this question: {}", i), answers:vec![(false, "Pog".to_string()),(false, "JFK".to_string()), (false, "Plog".to_string()), (true, i.to_string())]};
        questions.push(temp_question);
    }
    let create_request = pogootRequest{
        request:requestType::CreateGame,
        data:Data::CreateGameData(questionList { questions })
    };
    let create_request = serde_json::to_string(&create_request).unwrap();
    println!("Create request: {:?}", create_request);
    
    println!("Start game request");
    let start_request = pogootRequest{request:requestType::StartGame, data:Data::StartGameData};
    let start_request = serde_json::to_string(&start_request).unwrap();
    println!("Start game request: {:?}", start_request);
    
    println!("Next game request");
    let next_request = pogootRequest{request:requestType::NextQuestion, data:Data::NextGameData};
    let next_request = serde_json::to_string(&next_request).unwrap();
    println!("Next request: {:?}", next_request);


    println!("Verify Token request");
    let verify_request = pogootRequest{request:requestType::VerifyToken, data:Data::VerifyToken("token".to_string())};
    let verify_request = serde_json::to_string(&verify_request).unwrap();
    println!("Verify token request: {:?}", verify_request);

    println!("Login request");
    let login_request = pogootRequest{request:requestType::Temp, data:Data::TempData("Poggers".to_string())};
    let login_request = serde_json::to_string(&login_request).unwrap();
    println!("Login token request: {:?}", login_request);

}
