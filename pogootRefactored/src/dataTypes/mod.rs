use serde::{Deserialize, Serialize};

pub mod database;
pub mod config;
pub mod state_storage;
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct pogootRequest{
    pub request:requestType,
    pub data:Data

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
    ///Start Game Data, requires a GID (Game Id)
    StartGameData(String),
    ///Subscribe Game Data, requires GID
    SubscribeToGameData(String),
    ///Answer Data, should be pushed through websocket, contains a number in 0..answers.len()
    AnswerData(usize),
    ///ReSub Data, contains a Token and a GID
    ReSubscribeData(String, String),
    ///Login Request Data - Username, Password
    LoginData(String, String),
    ///Register Request Data - Username, Password
    RegisterData(String, String),
    ///Contains error message
    StandardErrorData(String),
    ///Contains error message
    GameCreationSuccessData(String),
    ///Contains error message
    GameCreationErrorData(String),
    ///Contains error message
    GameNotFoundErrorData(String),
    ///Data for temporary logins
    TempData(String),
    AnonData(String)
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
    registerResponse
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
