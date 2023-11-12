use std::pin::pin;
use std::time::{SystemTime, Duration};
use axum::{Router, routing::get, response::IntoResponse};
use futures::{poll, TryStreamExt};
use futures_util::Future;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::error::RecvError;
use tracing::info;
use axum::response::Response;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::extract::State;
use std::sync::Arc;
use axum::extract::ws::Message;
use futures::future;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use nanoid::nanoid;
use async_std::sync::RwLock;
use serde_json::to_string;

#[tokio::main]
async fn main() {
    let state = Arc::new(Database{
        thead_addresses:RwLock::new(HashMap::new())    

    });
    let app = Router::new()
    .route("/ws", get(handler))
    .with_state(state)
    ;

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn joinRoom()->impl IntoResponse{}


pub async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<Database>>) -> Response {
    info!("Handling Websocket Upgrade");
    ws.on_upgrade(|socket| handle_socket(socket, state))
}


async fn handle_socket(mut socket: WebSocket, state:Arc<Database>) {
    //DEAL WITH STUFF HERE BEFORE SPLITTING THE SOCKET
    let mut starter:Option<tokio::sync::oneshot::Sender<bool>> = None;
    let mut commander:Option<tokio::sync::oneshot::Sender<WebSocket>> = None;
    while let Some(msg) = socket.recv().await {
        
        let msg = if let Ok(msg) = msg  {
            match msg{
                Message::Text(x)=>{
                    let request:Result<pogootRequest,_> = serde_json::from_str(&*x);
                    if request.is_ok(){
                        //will proccess Start, Create, and Subscrbe - Answer will only be valid
                        //later
                        let request = request.unwrap();
                        let requestType = request.requestType;
                        let data = request.data;
                        if let request::StartGame(x) = requestType{
                            //TODO:Spawn Game thread
                            if let (Some(b),Some(c)) = (starter, commander){
                                starter = None;
                                commander=None;
                                let commanderResult = c.send(socket);
                                let infoThing = if let Ok(_)=commanderResult{
                                    info!("Successfully sent websocket to game thread");

                                let result = b.send(true);
                                
                                match result{
                                    Ok(_)=>match to_string(&responses::successResponse("Succesfully started Game".to_string())){Ok(x)=>x,_=>"Failed To Parse Success of Start Game".to_string()},
                                    _=>match to_string(&responses::errorResponse("Failed to started Game".to_string())){Ok(x)=>x,_=>"Failed To Parse Failure of Start Game".to_string()},
                                }
                            }else{
                                info!("failed to send websocket to game thread");
                                let result = b.send(false);
                                
                                match result{
                                    Ok(_)=>match to_string(&responses::successResponse("Succesfully deleted Game".to_string())){Ok(x)=>x,_=>"Failed To Parse Deletion of Game".to_string()},
                                    _=>match to_string(&responses::errorResponse("Failed to delete Game".to_string())){Ok(x)=>x,_=>"Failed To Parse Failure of Deletion Game".to_string()},
                                }
                            };
                            info!("Commander or starter not found: {}", infoThing);
                                return;
                            }else{
                                commander=None;
                                starter=None;
                                let thing = to_string(&responses::errorResponse("No game found".to_string())).ok();
                                let poggers = if thing.is_some(){
                                    thing.unwrap()
                                }else{
                                    "No game found".to_string()
                                };
                                info!("{}", poggers);
                                return;
                            }
                        }else if let request::CreateGame = requestType{
                            //TODO: add Game to the hashmap, also read the question data and parse
                            //it into a hashMap
                            
                            let (otx, orx) = tokio::sync::oneshot::channel();

                            let (ctx, crx) = tokio::sync::oneshot::channel();
                            starter=Some(otx);
                            commander=Some(ctx);
                            let (tx, rx) = tokio::sync::mpsc::channel::<(String, WebSocket)>(100);
                            if let Data::QuestionUpload(questions)=data{

                                let mut lock = state.thead_addresses.write().await;
                                let mut newGameId = nanoid!(6, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
                                while lock.contains_key(&newGameId){
                                    newGameId = nanoid!(6, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
                                }
                                lock.insert(newGameId.clone(), Arc::new(tx));
                                tokio::spawn(gameThread(rx, orx, questions, crx, state.clone(), newGameId.clone()));

                                let response = serde_json::to_string(&responses::gameCreatedResponse(newGameId.clone()));
                                match response{
                                    Ok(x)=>x.to_string(),
                                    _=>newGameId,
                                }
                            }else{
                                let response = to_string(&responses::gameCreationErrorResponse("Invalid Question Upload".to_string()));
                                match response{
                                    Ok(x)=>x.to_string(),
                                    _=>"Invalid Question Upload".to_string(),
                                }
                            }
                        }else if let request::SubscribeToGame(gameId) = requestType{
                            //TODO: let people subscribe to the game, split the socket and begin
                            //serving
                            // let mut username = "";
                            if let Data::Username(username)=data{
                            let lock = state.thead_addresses.read().await;
                            let destination = lock[&gameId].clone();
                            let resultOfSend = destination.send((username, socket)).await;
                            if resultOfSend.is_ok(){
                                    info!("Send Success");
                                    return;
                                }else{
                                    info!("Send Failed");
                                    return;
                                }
                            }else{
                                let response = to_string(&responses::errorResponse("Could Not Join Game".to_string()));
                                match response{
                                    Ok(x)=>x.to_string(),
                                    _=>"Could Not Join Game".to_string(),
                                }
                        }
                        }else if let request::Answer(_) = requestType{
                            let response = serde_json::to_string(&responses::errorResponse("Not Taking Answers".to_string()));
                            match response{
                                Ok(x)=>x.to_string(),
                                _=>"Not Taking Answers".to_string(),
                            }
                        }else{

                            "".to_string()
                        }
                    }else{
                    format!("invalid string: {:?}", x).to_string()
                    }
                },
                _=>{format!("unknown: {:?}", msg).to_string()}
            }
        }else {
            info!("Client Disconnected");
            if let Some(x)=starter{
                let result = x.send(false);
                match result{
                    Ok(_)=>info!("Successfully sent deletion request"),
                    Err(_)=>info!("Deletion request failed"),
                }
            }
            // client disconnected
            return;
        };

        if socket.send(Message::Text(msg.to_string())).await.is_err() {
            info!("Client Disconnected");
            // client disconnected
            return;
        }

    }

}
async fn handleRead(receiver: SplitStream<WebSocket>){
    //only expect ANSWER

}
async fn handleSend(sender: SplitSink<WebSocket, Message>){
    //send updates to scoreboard, time, and new questions
}

async fn gameThread(mut receiver:tokio::sync::mpsc::Receiver<(String, WebSocket)>, starter:tokio::sync::oneshot::Receiver<bool>, questions:questionList, commander:tokio::sync::oneshot::Receiver<WebSocket>, state:Arc<Database>, gameId:String){
//first step, be able to accumulate users
    let mut censoredQuestions = questions.clone();
    let censoredQuestions:Vec<censoredQuestion> = censoredQuestions.questions.iter().map(|x|x.clone().censored()).collect();
    let starter = tokio::spawn(startGame(starter));
    let commander = tokio::spawn(waitForCommander(commander));
    let mut inactive_time = SystemTime::now();
    let mut totalPlayers = vec![];
    let allowed_inactive_time = Duration::from_millis(120000);
    let mut finalCommand = None;
    //TODO: INCORPORATE COMMANDER
    loop{
        tokio::time::sleep(Duration::from_millis(50)).await;
        let newPlayers = receiver.recv();
        let newPlayers =  poll!(pin!(newPlayers));
        if let futures::task::Poll::Ready(Some(player))=newPlayers{
            info!("New Player: {:?}, Len Of Players: {}", player, totalPlayers.len());
            // let player = (player.0, Arc::new(player.1));
            totalPlayers.push((player, 0));
            inactive_time=SystemTime::now();
        }
        if let Ok(x) = inactive_time.elapsed(){
            if x>allowed_inactive_time{
                //TODO: DELETE FROM HASHMAP
                info!("Deleting Game Thread Due to Inactivity");
                starter.abort();
                receiver.close();
                let mut lock = state.thead_addresses.write().await;
                lock.remove(&gameId);
                return;
            }
        }
        
        if starter.is_finished(){
            info!("Starter finished");
            if commander.is_finished(){
                info!("Commander finished");
            info!("Starter is finished");
            let starterResult = starter.await;
            let commanderResult = commander.await;
            if let (Ok(x), Ok(c))=(starterResult, commanderResult){
                if !x{
                    info!("Starter Result was False");
                    return;
                }else if c.is_ok(){
                    finalCommand = Some(c.unwrap());
                    info!("Starter returned True");
                    break;
                }else{
                    info!("Final command invalid");
                    return;
                }
            }else{
                info!("Starter or Commander Result was Error");
                return;
            }
            }
            
        }
        
    }
    //begin execution of game loop
    //Game starts

        let gameData = gameData{totalQuestions:questions.questions.len()};
        let gameData = to_string(&gameData);

        if gameData.is_err(){
            info!("Game Data is error!!!!");
        }else if let Ok(x)=gameData{

        for playerSocketIndex in 0..totalPlayers.len(){
            let resultOfGameDataSend = totalPlayers[playerSocketIndex].0.1.send(Message::Text(x.clone())).await;
            if resultOfGameDataSend.is_err(){
                info!("GAME DATA SEND WAS ERROR, CLIENT DISCONNECT?");

            }
        }
    }
    //split all websockets into a wonderful new thing
    let mut totalPlayersNew=vec![];
    totalPlayers.into_iter().map(|x|(x.0.0, x.0.1.split(), x.1 as i32)).map(|x|{
        let futures = x.1;
        let (rx, tx) = tokio::sync::mpsc::channel(20);

        let (rx2, tx2) = tokio::sync::mpsc::channel(20);
        tokio::spawn(websocketSendHandler(futures.0, tx));
        tokio::spawn(websocketReceiverHandler(futures.1, rx2));
        (x.0, (rx, tx2), x.2)
    }).for_each(|x|totalPlayersNew.push(x));

    let mut curQues = 0;
    //TODO:REMEMBER TO DEAL WITH RECONNECTIONS AND NEW CONNECTIONS
    if let Some(mut finalCommand)=finalCommand{
    
    loop{
    let gameCommand = finalCommand.recv().await; 
        if let Some(Ok(extractedData))=gameCommand{
                if let Message::Text(wonderFulText)=extractedData{
                if let Ok(parse) = serde_json::from_str::<commanderCommand>(&wonderFulText){
                match parse{
                        commanderCommand::next=>{
                                if curQues<censoredQuestions.len(){
                                    //Broadcast question to the nerds
                                    
                                    let question = to_string(&censoredQuestions[curQues]);
                                        if question.is_err(){
                                            info!("Oh crap cakes what is happening god god oui oui");
                                            curQues+=1;
                                            continue;
                                        }
                                    let question = question.unwrap();
                                    for playerSocketIndex in 0..totalPlayersNew.len(){
                                        //TODO: FIX THIS MESS, USED HANDLERS
                                        let resultOfQuestionBroadcast = totalPlayersNew[playerSocketIndex].1.0.send(Message::Text(question.clone())).await;
                                        if resultOfQuestionBroadcast.is_err(){
                                            info!("BROADCAST WAS ERROR OH CRACK NO");
                                        }
                                        curQues+=1;
                                    }
                                    
                                    //TODO: MAKE A RECEIVER THING;
                                    //TODO:broadcast time and answers to host
                                    //TODO:Check answers, adjust scores
                                    //TODO:Send user Data, leaderboard
                                }else{
                                    //display end screen
                                }
                            }


                }
                        //end of parse
                    }
                }else{
                    info!("Could not Parse Message: {:?}", extractedData);
                }
                //GameCommand ends here
            }else{
                info!("Commander May Have Disconnected");
            }



    }
    }

}
async fn websocketReceiverHandler(mut receiver: SplitStream<WebSocket>, forwardResponses:tokio::sync::mpsc::Sender<Result<Message, axum::Error>>){
    loop{
        let msg = receiver.next().await;
        match msg{
            Some(Ok(msg))=>{
                let result = forwardResponses.send(Ok(msg)).await;
                if result.is_err(){
                    info!("Send error");
                    return;
                }
            // info!("Websocket returned error in receiver handler, Probably disconnected");
            },
            Some(Err(msg))=>{
            info!("Websocket returned error in receiver handler, Probably disconnected");
                let senderResult = forwardResponses.send(Err(msg)).await;
                if senderResult.is_err(){
                    return;
                }
            },
            None=>{
                info!("recieved: None");
                return;
            }
    }
        //TODO FIGURE OUT A GOOD SOLUTION FOR THIS THING
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    
}
async fn websocketSendHandler(mut sink: SplitSink<WebSocket, Message>, mut sendResponses:tokio::sync::mpsc::Receiver<Message>){
    // sink.send(sendResponses).await;
    while let Some(x) = sendResponses.recv().await{
        let result = sink.send(x).await;
        if result.is_err(){
            info!("Sink send error, probably disconnected");
            return;
        }
    }
    info!("Websocket Send Handler Close");
}

async fn waitForCommander(commander:tokio::sync::oneshot::Receiver<WebSocket>)->Result<WebSocket, RecvError>{
    info!("Waiting for Commander");
    let b = commander.await?;
    Ok(b)
}
async fn startGame(starter:tokio::sync::oneshot::Receiver<bool>)->bool{
    info!("Awaiting starter");
    let b=starter.await;
    if let Ok(x) = b{
        info!("Starter returned: {:?}",b);
        x
    }else{
        info!("Starter returned Error");
        false
    }
}

pub struct Database{
    //Hashmap of id & (currentQuestion Index, <question, (correct, answers)>, Usernames)
thead_addresses:RwLock<HashMap<String, Arc<tokio::sync::mpsc::Sender<(String, WebSocket)>>>>,

}


#[test]
fn serializeTestingForDamienBecauseHeIsNotBigBrainEnoughToJustRunTheCodeInHisBrain(){
    let testUsername = "Hi".to_string();
    let testRequest = pogootRequest { requestType:request::CreateGame, data:Data::Username(testUsername) };
    let magicResult = to_string(&testRequest).unwrap();
    println!("MagicalResult: {}", magicResult);
    info!("MagicalResult: {}", magicResult);

}

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
    requestType:request,
    data:Data
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct questionList{
    questions:Vec<Question>
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Question{
    question:String,
    answers:Vec<(bool, String)>
}

impl Question{
    fn censored(self)->censoredQuestion{
        censoredQuestion { question: self.question, answers: self.answers.iter().map(|x|x.1.clone()).collect::<Vec<String>>() }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct censoredQuestion{
    question:String,
    answers:Vec<String>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum responses{
    errorResponse(String),
    successResponse(String),
    gameCreatedResponse(String),
    gameCreationErrorResponse(String),
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct gameData{
    totalQuestions:usize,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum commanderCommand{
    next,
}
