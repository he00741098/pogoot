use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum request{
    ///Create new Game, Data required for question upload
    CreateGame,
    ///Start game takes a game ID and starts the Game
    StartGame(String),
    ///Subscribe to game takes a game ID and subscribes to the game. Data Required for Username
    SubscribeToGame(String),
    ///Answer takes in a number between 0..answers.len(), no data
    Answer(usize),
    ///Resub if discconnected
    ReSubscribeToGame(String),

}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Data{
    None,
    QuestionUpload(questionList),
    Username(String),
    UsernameAndToken(String, String),
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct pogootRequest{
    pub requestType:request,
    pub data:Data
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
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct censoredQuestion{
    pub question:String,
    pub answers:Vec<String>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum responses{
    errorResponse(String),
    successResponse(String),
    gameCreatedResponse(String),
    gameCreationErrorResponse(String),
    gameNotFoundError,
    sendReconToken,
    reconToken(String),
    reconnectorToken(String),
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct gameData{
    pub totalQuestions:usize,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum commanderCommand{
    next,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum commanderGamePlayResults{
    Leaderboard(Vec<(String, i32)>),
    GameOver(Vec<(String, i32)>)
}