use std::pin::pin;
use std::time::{SystemTime, Duration};
use axum::{Router, routing::get, response::IntoResponse};
use futures::poll;
use futures_util::Future;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::error::RecvError;
use tracing::info;
use axum::response::Response;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::extract::State;
use std::sync::Arc;
use axum::extract::ws::Message;

use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use nanoid::nanoid;
use async_std::sync::RwLock;
use futures_util::stream::{SplitSink, SplitStream};
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
                            let (tx, rx) = tokio::sync::mpsc::channel::<(String, WebSocket)>(100);
                            if let Data::QuestionUpload(questions)=data{

                                let mut lock = state.thead_addresses.write().await;
                                let mut newGameId = nanoid!(6, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
                                while lock.contains_key(&newGameId){
                                    newGameId = nanoid!(6, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
                                }
                                lock.insert(newGameId.clone(), Arc::new(tx));
                                tokio::spawn(gameThread(rx, orx, questions, crx));

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

async fn gameThread(mut receiver:tokio::sync::mpsc::Receiver<(String, WebSocket)>, starter:tokio::sync::oneshot::Receiver<bool>, questions:questionList, commander:tokio::sync::oneshot::Receiver<WebSocket>){
//first step, be able to accumulate users
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
            totalPlayers.push(player);
            inactive_time=SystemTime::now();
        }
        if let Ok(x) = inactive_time.elapsed(){
            if x>allowed_inactive_time{
                info!("Deleting Game Thread Due to Inactivity");
                starter.abort();
                receiver.close();
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum responses{
    errorResponse(String),
    successResponse(String),
    gameCreatedResponse(String),
    gameCreationErrorResponse(String),
}
