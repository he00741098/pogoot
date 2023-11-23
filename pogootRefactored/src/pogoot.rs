use crate::{*, util::{util}, dataTypes::state_storage::state_storage};
use tokio::sync::{mpsc::Sender, broadcast};
use tokio::sync::RwLock;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
pub struct pogootGame{
    questions:questionList,
    game_id:String,
    ///Real token, Visible playername, points
    player_list:Vec<(String, String, usize)>,
}
impl pogootGame{
    //pogoot entry manager
    pub fn build_game(questions:questionList)->Self{
        pogootGame { questions, game_id: nanoid!(20), player_list: vec![] }
    }
        pub async fn player_thread_start(mut socket:WebSocket, state:state_storage){
        //Usernames are dealt with in the login phase
        while let Some(msg) = socket.recv().await{
            if msg.is_err(){continue;}
            let request = util::parse_msg_to_pogoot(msg.unwrap());
            if request.is_err(){let result = socket.send(util::websocket_message_wrap(request.err().unwrap())).await;if result.is_err(){return;} continue;}
            let request=request.unwrap();
            //expecting request to be a Verify Token request
            let token = util::unpack_token_verify(request);
            if token.is_err(){let result = socket.send(util::websocket_message_wrap(token.err().unwrap())).await;if result.is_err(){return;} continue;}
            let token = token.unwrap();
            tokio::spawn(Self::player_thread_token_verify(socket, state, token));
            return;
        }
    }
    async fn player_thread_token_verify(mut socket:WebSocket, state:state_storage, token:String){
        let token_clone = token.clone();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let login_result = state.login_channel.send(loginRequest{request_type:loginRequestTypes::TokenVerify, data:loginData::TokenVerify(token, tx)}).await;
        if login_result.is_err(){info!("Login Channel Error: Potential Catastrophic failure! TODO: Implement reset system in case of failure");return;}
        let callback_response = rx.await;
        if callback_response.is_err(){info!("Callback was error!"); socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Callback failed, unknow issues have arisen"))).await; return;}
        let callback_response=callback_response.unwrap();
        if callback_response.is_err(){let result = socket.send(util::websocket_message_wrap(callback_response.err().unwrap())).await; if result.is_err(){return} return;}
        let username=callback_response.unwrap();
        //Username obtained, Player logged in successfully, token obtained
        //Begin game connection
        tokio::spawn(Self::player_thread_game_join(socket, state, token_clone, username));
    }
    async fn player_thread_game_join(mut socket:WebSocket, state:state_storage, token:String, username:String){
        let game_id_ask_response = socket.send(util::websocket_message_wrap(pogootResponse{response:responseType::successResponse, data:Data::SocketVerified})).await;
        if game_id_ask_response.is_err(){info!("Socket disconnect"); return;}
        //Client prompted to send target game
        while let Some(msg) = socket.recv().await{
            if msg.is_err(){let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Message is Error"))).await; if result.is_err(){return} continue;}
            let msg = msg.unwrap();
            let parsed_msg = util::parse_msg_to_pogoot(msg);
            if parsed_msg.is_err(){let result = socket.send(util::websocket_message_wrap(parsed_msg.err().unwrap())).await; if result.is_err(){return;} continue;}
            let request = parsed_msg.unwrap();
            //check if request is the right type of request
            match request.request {
                requestType::SubscribeToGame=>{
                    //check if data is the right data type
                    match request.data{
                        Data::SubscribeToGameData(target)=>{
                            //grab the game progress broadcaster
                            let broadcaster = state.games.read().await;
                            let broadcaster2 = broadcaster.get(&target);
                            if broadcaster2.is_some(){
                                let unwrapped_broadcasters = broadcaster2.unwrap();
                                let broadcaster3 = unwrapped_broadcasters.1.subscribe();
                                let merger = unwrapped_broadcasters.0.clone();
                                drop(broadcaster);
                                let (splitS, splitT) = socket.split();
                                tokio::spawn(Self::player_thread_play(splitS, state, token, username, broadcaster3));
                                return;
                            }else{
                                let failure_send = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Broadcaster not found"))).await;
                                if failure_send.is_err(){return;}
                                return;
                            }
                        },
                        _=>{
                            let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Wrong data Type"))).await;
                            if result.is_err(){return;}
                            continue;
                        }
                    }
                },
                _=>{
                    let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Wrong request Type"))).await;
                    if result.is_err(){return;}
                    continue;
                }
            }
        }
    }
    async fn player_thread_play(mut socket:SplitSink<WebSocket, Message>, state:state_storage, token:String, username:String, mut broadcast:broadcast::Receiver<GameUpdate>){
        //Waiting for game to start...
        //wait for broadcast
        while let Ok(gameUpdate) = broadcast.recv().await{
            if gameUpdate.is_player_list(){
                //filter playerlist and send relevant data to player
                let player_list = gameUpdate.get_player_list();
                if player_list.is_err(){info!("Server malfunction"); socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Something went very wrong"))).await.err(); return;}
                let mut player_list = player_list.unwrap();
                let pogoot_response = util::get_relevant_data(&mut player_list, &username, &token);
                let socket_send_result = socket.send(util::websocket_message_wrap(pogoot_response)).await;
                if socket_send_result.is_err(){
                    return;
                }
            }else{
                //send question to player
                let question = gameUpdate.get_question();
                if question.is_err(){let result = socket.send(util::websocket_message_wrap(question.err().unwrap())).await; if result.is_err(){return;}return;}
                let question = question.unwrap();
                let question_send_result = socket.send(util::websocket_message_wrap(pogootResponse { response: responseType::questionResponse, data: Data::QuestionData(question) })).await;
                if question_send_result.is_err(){return;}
            }
        }
    }
    async fn player_thread_receiver(mut socket:SplitStream<WebSocket>, merger:tokio::sync::mpsc::Sender<pogootRequest>, token:String, username:String){
        while let Some(msg) = socket.next().await{
            if msg.is_ok(){
                let msg = msg.unwrap();
                let msg = util::parse_msg_to_pogoot(msg);
                if msg.is_err(){
                    info!("Msg could not be parsed to pogoot, probable failure will occur soon");
                    continue;
                }
                let msg = msg.unwrap();
                let merger_result = merger.send(msg).await;
                if merger_result.is_err(){
                    return;
                }
            }
        }
    }

    
}

