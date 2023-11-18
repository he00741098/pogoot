use std::pin::pin;
use std::time::{SystemTime, Duration};
use axum::{Router, routing::get};
use futures::{poll};
use futures_util::future::join_all;
use futures_util::stream::FuturesUnordered;
use tracing::info;
use axum::response::Response;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::extract::State;
use std::sync::Arc;
use axum::extract::ws::Message;
use futures_util::{stream::{StreamExt, SplitSink, SplitStream}};
use std::collections::HashMap;
use nanoid::nanoid;
use async_std::sync::RwLock;
use serde_json::to_string;
mod requestTypes;
mod tests;
mod websocketHandlers;
use crate::requestTypes::*;
use crate::websocketHandlers::*;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // Build the subscriber
    .finish();
tracing::subscriber::set_global_default(subscriber)
.expect("setting default subscriber failed");
info!("Initiated subscriber");

    let state = Arc::new(Database{
        thead_addresses:RwLock::new(HashMap::new())    

    });
    let app = Router::new()
    .route("/hello", get(|| async {"hello!"}))
    .route("/ws", get(handler))
    .with_state(state)
    ;
info!("App initiated");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


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
                                //TODO: QUESTION LIST VERIFICATION? MAKE SURE THERE IS AT LEAST ONE CORRECT ANSWER
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
                            let destination = lock.get(&gameId).clone();
                            if destination.is_err(){
                                info!("Game Id Not found");
                                let response_msg = responses::gameNotFoundError;
                                let response_msg = to_string(&response_msg);
                                let response_msg = 
                                let socketSend = socket.send();
                            }
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

async fn verify_reconnection(mut player:(String, WebSocket), correctToken:String)->Result<(String, WebSocket), ()>{
    let msg = to_string(&crate::requestTypes::responses::sendReconToken);
    if msg.is_err(){
        return Err(())
    }
    let msg = msg.unwrap();
    let request_send = player.1.send(Message::Text(msg)).await;
    if request_send.is_err(){
        return Err(())
    }
    let returnMessage = player.1.recv().await;
    if let Some(Ok(Message::Text(msg))) = returnMessage{
        let parsed = serde_json::from_str::<crate::requestTypes::responses>(&msg);
        if parsed.is_ok(){
            let parsed = parsed.unwrap();
            if let responses::reconToken(parsed)=parsed{
            if parsed==correctToken{
                return Ok(player);
            }
        }
        }
    }
    Err(())
}

async fn gameThread(mut receiver:tokio::sync::mpsc::Receiver<(String, WebSocket)>, starter:tokio::sync::oneshot::Receiver<bool>, questions:questionList, commander:tokio::sync::oneshot::Receiver<WebSocket>, state:Arc<Database>, gameId:String){
//first step, be able to accumulate users
//TODO: GET REJOIN/JOIN WORKING IN GAME LOOP, Make COMMANDER RECONNECT, SEND STATUS TO PLAYERS.
    let mut censoredQuestions = questions.clone();
    let censoredQuestions:Vec<censoredQuestion> = censoredQuestions.questions.iter().map(|x|x.clone().censored()).collect();
    let starter = tokio::spawn(startGame(starter));
    let commander = tokio::spawn(waitForCommander(commander));
    let mut inactive_time = SystemTime::now();
    let mut totalPlayers = vec![];
    let allowed_inactive_time = Duration::from_millis(120000);
    let mut finalCommand = None;
    let mut recons = vec![];
    //Username, Recconection Id
    let mut taken_usernames:Vec<(String, String)> = vec![];
    //TODO: INCORPORATE COMMANDER
    loop{
        tokio::time::sleep(Duration::from_millis(50)).await;
        let newPlayers = receiver.recv();
        let newPlayers =  poll!(pin!(newPlayers));
        if let futures::task::Poll::Ready(Some(mut player))=newPlayers{

            info!("New Player: {:?}, Len Of Players: {}", player, totalPlayers.len());
            // let player = (player.0, Arc::new(player.1));
            if taken_usernames.iter().map(|x|x.0.clone()).collect::<Vec<String>>().contains(&player.0){
                info!("Username taken: try recon");
                let username = player.0.clone();
                let correct_token = taken_usernames.iter().filter(|x|x.0.clone()==username).map(|x|x.1.clone()).collect::<Vec<String>>()[0].clone();
                recons.push(tokio::spawn(verify_reconnection(player, correct_token)));
            }else{
                let temp_id = nanoid!(10);
                taken_usernames.push((player.0.clone(), temp_id.clone()));
                let temp_token = responses::reconnectorToken(temp_id.clone());
                let temp_token = to_string(&temp_token);
                if temp_token.is_ok(){
                let resulting_thing = player.1.send(Message::Text(temp_token.unwrap())).await;
                if resulting_thing.is_err(){
                    info!("Player send error, disconnection?");
                }
                }
                totalPlayers.push((player, 0));
                
            }
            inactive_time=SystemTime::now();

        }
        let ready_recons = join_all(recons.iter_mut().filter(|x|x.is_finished())).await;
        ready_recons.into_iter().filter(|pog|pog.is_ok()).map(|g|g.unwrap()).filter(|e|e.is_ok()).map(|w|w.unwrap()).for_each(|x|{
            let index = totalPlayers.iter().enumerate().filter(|b|b.1.0.0==x.0).map(|c|c.0).collect::<Vec<usize>>()[0];
            let temp_username = x.0.clone();
            let cloned_taken = taken_usernames.clone();
            totalPlayers[index].0=x;
            if cloned_taken.iter().map(|x|x.0.clone()).collect::<Vec<String>>().contains(&temp_username){
                    taken_usernames = cloned_taken.into_iter().filter(|x|x.0!=temp_username).collect();

            }
        });
        if let Ok(x) = inactive_time.elapsed(){
            if x>allowed_inactive_time{
                //TODO: DELETE FROM HASHMAP
                info!("Deleting Game Thread Due to Inactivity");
                recons.into_iter().for_each(|x|x.abort());
                starter.abort();
                commander.abort();
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
        let length = totalPlayers.len();
        for playerSocketIndex in 0..length{
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
        tokio::spawn(websocketReceiverHandler(futures.1, rx2, x.0.clone()));
        (x.0, (rx, tx2), x.2)
    }).for_each(|x|totalPlayersNew.push(x));

    let mut curQues = 0;
    //TODO:REMEMBER TO DEAL WITH RECONNECTIONS AND NEW CONNECTIONS
    if let Some(mut finalCommand)=finalCommand{
    let mut required_completes = totalPlayersNew.len();
    let mut ignoreList:Vec<usize> = vec![];
    loop{
    let gameCommand = finalCommand.recv().await; 
        if let Some(Ok(extractedData))=gameCommand{
                if let Message::Text(wonderFulText)=extractedData{
                if let Ok(parse) = serde_json::from_str::<commanderCommand>(&wonderFulText){
                match parse{
                        commanderCommand::next=>{
                                if curQues<censoredQuestions.len(){
                                    //Broadcast question to the nerds
                                    let correct_answer:Vec<usize> = questions.questions[curQues].clone().answers.into_iter().enumerate().filter(|x|x.1.0).map(|x|x.0).collect();
                                    let question = to_string(&censoredQuestions[curQues]);
                                        if question.is_err(){
                                            info!("Oh crap cakes what is happening god god oui oui");
                                            curQues+=1;
                                            continue;
                                        }
                                    let question = question.unwrap();
                                    //BROADCAST QUESTION TO COMMANDER AS WELL
                                    let commanderSendQuestion = finalCommand.send(Message::Text(question.clone())).await;
                                    if commanderSendQuestion.is_err(){
                                        info!("Commander is error, Will try to continue");
                                        
                                    }
                                    for playerSocketIndex in 0..totalPlayersNew.len(){
                                        let resultOfQuestionBroadcast = totalPlayersNew[playerSocketIndex].1.0.send(Message::Text(question.clone())).await;
                                        if resultOfQuestionBroadcast.is_err()&&!ignoreList.contains(&playerSocketIndex){
                                            info!("BROADCAST WAS ERROR OH CRACK NO");
                                            required_completes-=1;
                                            ignoreList.push(playerSocketIndex);
                                        }else if resultOfQuestionBroadcast.is_ok()&&ignoreList.contains(&playerSocketIndex){
                                            required_completes+=1;
                                            ignoreList=ignoreList.into_iter().filter(|x|*x!=playerSocketIndex).collect();
                                        }
                                    }
                                    curQues+=1;
                                    
                                    //MADE A RECEIVER THING;
                                    let mut receiver: FuturesUnordered<_> = totalPlayersNew.iter_mut().map(|x|x.1.1.recv()).collect();
                                    let start_time = SystemTime::now();
                                    let max_wait_time = Duration::from_secs(30);
                                    let mut answers:Vec<(String, usize)> = vec![];
                                    loop{
                                        //CHECK FOR NEXT AND CHECK FOR FINISHED AND IGNORE DEAD CONNECTIONS
                                        if let Ok(x) = start_time.elapsed(){
                                            if x>=max_wait_time{
                                                break;
                                            }
                                            if answers.len()>=required_completes{
                                                break;
                                            }
                                            let temporary_result = tokio::time::timeout(Duration::from_secs(1),receiver.next()).await;
                                            match temporary_result{
                                                Ok(Some(Some(Ok(choices))))=>{
                                                    if let Message::Text(text) = choices.1{
                                                        if let Ok(final_choice) = text.parse::<usize>(){
                                                            answers.push((choices.0, final_choice));
                                                        }else{
                                                            info!("Parse error: {:?}", text);
                                                        }
                                                    }else{
                                                        info!("Not Text: {:?}", choices);
                                                    }
                                                },
                                                Err(_)=>{info!("Timeout"); continue;}
                                                _=>{info!("Not Some(Some(Ok(_))): {:?}", temporary_result); continue;}
                                            }
                                        }
                                    }
                                    drop(receiver);
                                    //check answers
                                    let mut max_point_bonus = 1000.0;
                                    for i in answers{
                                        let mut correct = false;
                                        for b in 0..correct_answer.len(){
                                            if i.1==correct_answer[b]{
                                                correct=true;
                                            }
                                        }
                                        if correct{
                                            for c in 0..totalPlayersNew.len(){
                                                if i.0==totalPlayersNew[c].0{
                                                    totalPlayersNew[c].2+=(max_point_bonus) as i32;
                                                    let send_score_result = totalPlayersNew[c].1.0.send(Message::Text(format!("{}",max_point_bonus))).await;
                                                    info!("Score send Result!!: {:?}, {:?}", send_score_result, totalPlayersNew[c].0);
                                                    max_point_bonus*=0.9;
                                                }
                                            }
                                        }
                                    }

                                    //TODO:broadcast time? and answers to host [x]
                                    //Check answers, adjust scores
                                    //TODO:Send user Data, leaderboard
                                    let round_result = totalPlayersNew.iter().map(|x|(x.0.clone(),x.2)).collect::<Vec<(String, i32)>>();
                                    let round_result = to_string(&commanderGamePlayResults::Leaderboard(round_result));
                                    if round_result.is_ok(){
                                        let fun_final_result_thing = finalCommand.send(Message::Text(round_result.unwrap())).await;
                                        match fun_final_result_thing{
                                            Ok(_)=>info!("Leaderboard Sent Successfully"),
                                            Err(_)=>info!("Commander may have disconnected. Attempting to continue")
                                        }
                                    }
                                }else{
                                    let mut clonedTotalPlayers =vec![];
                                    for playerDataTemp in 0..totalPlayersNew.len(){
                                        clonedTotalPlayers.push((totalPlayersNew[playerDataTemp].0.clone(), totalPlayersNew[playerDataTemp].2));
                                    }
                                    //display end screen
                                    let round_result = to_string(&commanderGamePlayResults::GameOver(clonedTotalPlayers));
                                    if round_result.is_ok(){
                                        let fun_final_result_thing = finalCommand.send(Message::Text(round_result.unwrap())).await;
                                        match fun_final_result_thing{
                                            Ok(_)=>info!("Game Over Sent Successfully"),
                                            Err(_)=>info!("Commander may have disconnected. Attempting to continue")
                                        }
                                    }
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

pub struct Database{
    //Hashmap of id & (currentQuestion Index, <question, (correct, answers)>, Usernames)
thead_addresses:RwLock<HashMap<String, Arc<tokio::sync::mpsc::Sender<(String, WebSocket)>>>>,

}



