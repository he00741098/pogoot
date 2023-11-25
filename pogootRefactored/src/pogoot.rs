use crate::{*, util::{util}, dataTypes::state_storage::state_storage};
use futures::future::select;
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
        ///regular players directed here
        pub async fn player_thread_start(mut socket:WebSocket, state:Arc<state_storage>){
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
    async fn player_thread_token_verify(mut socket:WebSocket, state:Arc<state_storage>, token:String){
        let token_clone = token.clone();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let login_result = state.login_channel.send(loginRequest{request_type:loginRequestTypes::TokenVerify, data:loginData::TokenVerify(token, tx)}).await;
        if login_result.is_err(){info!("Login Channel Error: Potential Catastrophic failure! TODO: Implement reset system in case of failure");return;}
        let callback_response = rx.await;
        if callback_response.is_err(){info!("Callback was error!"); let callback_result_send_result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Callback failed, unknow issues have arisen"))).await; if callback_result_send_result.is_err(){return;} return;}
        let callback_response=callback_response.unwrap();
        if callback_response.is_err(){let result = socket.send(util::websocket_message_wrap(callback_response.err().unwrap())).await; if result.is_err(){return} return;}
        let username=callback_response.unwrap();
        //Username obtained, Player logged in successfully, token obtained
        //Begin game connection
        tokio::spawn(Self::player_thread_game_join(socket, state, token_clone, username));
    }
    async fn player_thread_game_join(mut socket:WebSocket, state:Arc<state_storage>, token:String, username:String){
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
                                let playerJoinUpdate = merger.send(pogootRequest { request: requestType::PlayerJoinUpdate, data: Data::PlayerJoinUpdateData(username.clone())}).await;
                                if playerJoinUpdate.is_err(){
                                    info!("Player Join Update is Err");
                                }
                                drop(broadcaster);
                                let (splitS, splitT) = socket.split();
                                tokio::spawn(Self::player_thread_play(splitS, state, token.clone(), username.clone(), broadcaster3));
                                tokio::spawn(Self::player_thread_receiver(splitT, merger, token, username));
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
    async fn player_thread_play(mut socket:SplitSink<WebSocket, Message>, state:Arc<state_storage>, token:String, username:String, mut broadcast:broadcast::Receiver<GameUpdate>){
        //Waiting for game to start...
        //wait for broadcast
        while let Ok(gameUpdate) = broadcast.recv().await{
            if gameUpdate.is_player_list(){
                //filter playerlist and send relevant data to player
                if gameUpdate.update_version==0{
                    println!("Update version: {}", gameUpdate.update_version);
                    continue;
                }
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
                let mut msg = msg.unwrap();
                if msg.is_answer(){
                    let map_result = util::map_answer_to_internal(msg, &username, &token);
                    if map_result.is_err(){
                        info!("Map result error");
                        continue;
                    }
                    msg = map_result.unwrap();
                }
                let merger_result = merger.send(msg).await;
                if merger_result.is_err(){
                    return;
                }
            }
        }
    }
    ///Game creation sequence = connection, send questions, start streaming game updates
   pub async fn game_commander_handler(mut socket:WebSocket, state:Arc<state_storage>){
        //TODO; require login
        while let Some(msg) = socket.recv().await{
            if msg.is_err(){
                let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Msg is error"))).await;
                if result.is_err(){return;}
                continue;
            }
            let msg = msg.unwrap();
            if let Ok(msg) = util::parse_msg_to_pogoot(msg){
                match msg.request{
                    requestType::CreateGame=>{
                        match msg.data{
                            Data::CreateGameData(questions)=>{
                                let game = Self::build_game(questions);
                                let password = nanoid!(20);
                                let game_id = nanoid!(6, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
                                let (gtx, grx) = tokio::sync::mpsc::channel::<(usize, Duration)>(100);
                                let sender = tokio::spawn(game.game_builder(state, game_id.clone(), password.clone(), gtx)).await;
                                let result = socket.send(util::websocket_message_wrap(pogootResponse { response: responseType::gameCreatedSuccessResponse, data: Data::GameCreationSuccessData(game_id, password) })).await;
                                if result.is_err(){
                                    return;
                                }
                                if sender.is_err(){
                                    info!("Join error on sender");
                                    return;
                                }
                                let sender = sender.unwrap();
                                tokio::spawn(Self::commander_proccessor(socket, sender.0, sender.1, grx));
                                return;
                            },
                            _=>{

                            }
                        }
                    },
                    _=>{
                        let result = socket.send(util::websocket_message_wrap(pogootResponse{
                            response:responseType::gameCreationErrorResponse,
                            data:Data::GameCreationErrorData("Not a Create Game Request".to_string())
                        })).await;
                        if result.is_err(){return;}
                        continue;
                    },
                }
            }else{
                let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Msg could not be parsed"))).await;
                if result.is_err(){return;}
                continue;
            }
        }

    }
    async fn commander_proccessor(mut socket:WebSocket, commander:tokio::sync::mpsc::Sender<GameCommand>, mut broadcast_results:tokio::sync::broadcast::Receiver<GameUpdate>, mut player_count:tokio::sync::mpsc::Receiver<(usize, Duration)>){
        //deal with starting the game
        loop{
        tokio::select!{
        request = socket.recv()=>{
            if request.is_none(){
                    break;
                }
            let request = request.unwrap();
            if request.is_err(){return;}
            let request = request.unwrap();
            let request = util::parse_msg_to_pogoot(request);
            match request{
                Ok(pogootRequest)=>{
                    match pogootRequest.request{
                        requestType::StartGame=>{
                            let commander_result = commander.send(GameCommand::Start).await;
                            if commander_result.is_err(){
                                info!("Commander failed");
                                let resulting = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Commander Failed to Start Game"))).await;
                                if resulting.is_err(){
                                    info!("Commander socket discconect");
                                    return;
                                }
                                return;
                            }
                            break;
                        },
                        _=>{
                            let send_result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Not start game request"))).await;
                            if send_result.is_err(){return;}
                            continue;
                        }
                    }
                },
                Err(_)=>{return;}
            }
        },
        player_update = broadcast_results.recv()=>{
                if player_update.is_err(){
                    continue;
                }
                let player_update = player_update.unwrap();
                if let Ok(request) = player_update.get_player_list(){
                    let socket_send = socket.send(util::websocket_message_wrap(pogootResponse{response:responseType::PlayerJoinUpdateResponse, data:Data::PlayerJoinUpdateData(request[0].0.clone())})).await;
                    if socket_send.is_err(){
                        info!("Player join update failed");
                    }
                }
            }
        }
        }
        //empty broadcaster
        while !broadcast_results.is_empty(){
            let thing = broadcast_results.recv().await;
        }
        //game loop - deals with receiving commands and receiving the broadcast_results
        loop{
            tokio::select! {
                v = socket.recv() => {
                    if v.is_none(){
                        return;
                    }
                    let v= v.unwrap();
                    if v.is_err(){
                        info!("Message is error");
                        continue;
                    }
                    let msg = util::parse_msg_to_pogoot(v.unwrap());
                    if msg.is_err(){
                        let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Not a request"))).await;
                        if result.is_err(){
                            return;
                        }
                        info!("Could not parse msg");
                        continue;
                    }

                    let msg = msg.unwrap();
                    match msg.request{
                        requestType::NextQuestion=>{
                            let commander_send = commander.send(GameCommand::Next).await;
                            if commander_send.is_err(){
                                let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Commander is err"))).await;
                                if result.is_err(){
                                    return;
                                }
                                return;
                            }
                        },
                            //TODO: support more request types
                        _=>{},
                    }
                },
                v = broadcast_results.recv()=>{
                    if v.is_err(){
                        let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Broadcast Error"))).await;
                        if result.is_err(){
                            return;
                        }
                    }
                    let v = v.unwrap();
                    let response = if v.is_player_list(){
                        let player_list = v.get_player_list();
                        if player_list.is_err(){
                            let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Broadcast Err: Playerlist not correct"))).await;
                            if result.is_err(){
                                return;
                            }
                            //TODO:atempt to continue the game
                            return;
                        }
                        let player_list=player_list.unwrap();
                        pogootResponse{response:responseType::gameUpdateResponse, data: Data::LeaderBoardUpdate(player_list)}
                    }else{
                        let question = v.get_question();
                        if question.is_err(){
                            let result = socket.send(util::websocket_message_wrap(pogootResponse::standard_error_message("Broadcast Err: Question is err"))).await;
                            if result.is_err(){
                                return;
                            }
                            //TODO:atempt to continue the game
                            return;
                        }
                        let question = question.unwrap();
                        pogootResponse{
                            response:responseType::questionResponse,
                            data:Data::QuestionData(question)
                        }
                    };
                    let response_response = socket.send(util::websocket_message_wrap(response)).await;
                    if response_response.is_err(){
                        info!("Commander discconnected");
                        return
                    }
            },
            v = player_count.recv()=>{
                    if v.is_some(){
                    let v=v.unwrap();
                        let send_result = socket.send(util::websocket_message_wrap(pogootResponse { response: responseType::GameTimeUpdateResponse, data: Data::GamePlayerTimeUpdate(v.0, v.1) })).await;
                        if send_result.is_err(){
                            info!("Game commander disconnected, relinquishing control");
                            //TODO: Make game commander reconnect actually work
                            return;
                        }
                    }
                }
            }
        }
    }

    ///commander will be directed here
    async fn game_builder(self, state:Arc<state_storage>, game_id:String, game_password:String, player_count_sender:tokio::sync::mpsc::Sender<(usize, Duration)>)->(tokio::sync::mpsc::Sender<GameCommand>, tokio::sync::broadcast::Receiver<GameUpdate>){
        //merger channels
        let (tx, rx) = tokio::sync::mpsc::channel::<pogootRequest>(100);
        //game update broadcast channel
        let (btx, brx) = tokio::sync::broadcast::channel::<GameUpdate>(100);
        let game_broadcaster = btx.clone();
        let game_receiver = rx;
        //insert the channels into the games hash map
        let mut lock = state.games.write().await;
        lock.insert(game_id.to_string(), (tx, btx));
        drop(lock);
        //game is now available
        // todo!()
        let (ctx, crx) = tokio::sync::mpsc::channel::<GameCommand>(20);
        tokio::spawn(self.game_proccessor(game_receiver, game_broadcaster, crx, player_count_sender));
        //add the commander channel to the hashmap
        let mut lock = state.commander_portals.write().await;
        lock.insert(game_id.to_string(), (game_password.to_string(),ctx.clone()));
        (ctx, brx)
    }

    // async fn commander_proccessor(commander_retriever:tokio::sync::mpsc::Receiver<WebSocket>)

    async fn game_proccessor(mut self, mut game_receiver:tokio::sync::mpsc::Receiver<pogootRequest>, game_broadcaster:tokio::sync::broadcast::Sender<GameUpdate>, mut crx:tokio::sync::mpsc::Receiver<GameCommand>, player_count_sender:tokio::sync::mpsc::Sender<(usize, Duration)>){
        //wait for start command
        // while let Some(command) = crx.recv().await{
        // }
        loop{
        tokio::select! {
            player_update = game_receiver.recv()=>{
                if player_update.is_none(){
                    info!("Player Update is None!");
                    return;
                }
                let player_update=player_update.unwrap();
                match player_update.request{
                    requestType::PlayerJoinUpdate=>{
                        match player_update.data{
                            Data::PlayerJoinUpdateData(data)=>{
                                let player_update_result = game_broadcaster.send(GameUpdate { update_version: 0, data: gameUpdateHelper::playerListUpdate(vec![(data, String::with_capacity(0), 0)]) });
                                if player_update_result.is_err(){
                                    info!("Player update result is error");
                                }
                            },
                            _=>{},
                        }
                    },
                    _=>{},
                }
            },
            command = crx.recv()=>{
                if command.is_none(){
                    //TODO FIGURE FAILBACK
                    info!("Command receiver is NONE!");
                    return;
                }
                let command = command.unwrap();
                match command{
                    GameCommand::Start | GameCommand::Next=>{
                        break;
                    }
                    _=>{
                        continue;
                    }
                }
            }
            }
        }


        let mut currQuestion = 0;
        let mut update_version = 1;
        while currQuestion<self.questions.questions.len(){
            let currentQuestion = self.questions.questions[currQuestion].clone();
            let answers = currentQuestion.answers.clone();
            //broadcast question
            let broadcast_result = game_broadcaster.send(GameUpdate{update_version, data:gameUpdateHelper::newQuestion(currentQuestion.censored(currQuestion), currQuestion)});
            if broadcast_result.is_err(){
                info!("Broadcast returned error");
                currQuestion+=1;
                update_version+=1;
                continue;
            }
            let subscribers = broadcast_result.unwrap();
            //set timer: 30 sec or full completion
            // let over = futures_util::future::select(tokio::time::sleep(Duration::from_secs(30)));
            let destination_time = tokio::time::Instant::now()+Duration::from_secs(30);
            let mut correct_answers = 0;
            let mut total_answers = 0;
            let updater = player_count_sender.clone();
            let (utx, mut urx) = tokio::sync::mpsc::channel::<usize>(10);
            let updating_destination_time = destination_time.clone();
            let update_handle = tokio::spawn(async move{
                let destination = updating_destination_time;
                while let Some(update) = urx.recv().await{
                    if tokio::time::Instant::now()>destination{
                        return;
                    }
                    let result = updater.send((update, updating_destination_time-tokio::time::Instant::now())).await;
                    if result.is_err(){
                        return;
                    }
                }
            });
            loop{
            // while let Ok(Some(response)) = tokio::time::timeout_at(destination_time, game_receiver.recv()).await
                tokio::select!{
                response = tokio::time::timeout_at(destination_time, game_receiver.recv())=>{
                        if let Ok(Some(response))=response{
                            match response.request{
                                requestType::InternalAnswer=>{
                                    match response.data{
                                        Data::InternalAnswerData(username, token, questionId, answer)=>{
                                            total_answers+=1;
                                            let answer_update = utx.send(total_answers).await;
                                                if answer_update.is_err(){
                                                    info!("Updater is broken somehow");
                                                }
                                            if questionId==currQuestion&&answer<answers.len()&&answers[answer].0{
                                                correct_answers+=1;
                                                //find the correct player
                                                //TODO: Check that the player has not already
                                                //answered
                                                let player_index = self.player_list.iter().enumerate().filter(|x|&x.1.1==&username&&&x.1.0==&token).collect::<Vec<(usize, &(String, String, usize))>>();
                                                if player_index.len()==1{
                                                    info!("Player found");
                                                    let player_index_in_vec = player_index[0].0;
                                                    drop(player_index);
                                                    self.player_list[player_index_in_vec].2+=((subscribers as f64/correct_answers as f64) * 800.0).floor() as usize;
                                                }else if player_index.len()==0{
                                                    info!("Player added!");
                                                    self.player_list.push((token, username, ((subscribers as f64/correct_answers as f64) * 800.0).floor() as usize));
                                                }else if player_index.len()>1{
                                                    info!("Crazy things are happening right now! More than one player with same username and TOKEN")
                                                }
                                            }else if questionId==currQuestion&&answer<answers.len()&&!answers[answer].0{
                                                //answer is wrong
                                                let player_index = self.player_list.iter().enumerate().filter(|x|&x.1.1==&username&&&x.1.0==&token).collect::<Vec<(usize, &(String, String, usize))>>();
                                                if player_index.len()==1{
                                                    info!("Player found");
                                                    drop(player_index);
                                                }else if player_index.len()==0{
                                                    info!("Player added!");
                                                    self.player_list.push((token, username, 0));
                                                }else if player_index.len()>1{
                                                    info!("Crazy things are happening right now! More than one player with same username and TOKEN")
                                                }
                                            }else{
                                                if answer<answers.len(){
                                                info!("Wrong Question!, QuestionId: {:?}, currQuestion: {:?}, answer: {:?}, answers len: {:?}, answer bool: {:?}", questionId, currQuestion, answer, answers.len(), answers[answer].0);
                                                }else{
                                                    info!("Answer is greater than answers len");
                                                info!("Wrong Question!, QuestionId: {:?}, currQuestion: {:?}, answer: {:?}, answers len: {:?}", questionId, currQuestion, answer, answers.len());
                                                }
                                            }
                                            if total_answers>=subscribers-1{
                                                break;
                                            }
                                        },
                                        _=>{}
                                    }
                                },
                                _=>{},
                            }
                        }else{
                            break;
                        }
                    },
                game_command = crx.recv()=>{
                        if game_command.is_some(){
                            match game_command.unwrap(){
                                GameCommand::Next=>{
                                    update_handle.abort();
                                    // currQuestion+=1;
                                    break;
                                },
                                _=>{}
                            }
                        }
                    }
            }
            }
            //send updates about game
            // update_version+=1;
            // let broadcast_result_leaderboard = game_broadcaster.send(GameUpdate { update_version, data: gameUpdateHelper::playerListUpdate(self.player_list.clone()) });
            // if broadcast_result_leaderboard.is_err(){
            //     info!("broadcast_result_leaderboard errored");
            // }

            let question_sender_result = game_broadcaster.send(GameUpdate { update_version, data: gameUpdateHelper::playerListUpdate(self.player_list.clone()) });
            update_version+=1;
            if question_sender_result.is_err(){
                info!("Question sender is error");
            }
            if currQuestion+1<self.questions.questions.len(){
                while let Some(command) = crx.recv().await{
                    match command{
                        GameCommand::Next=>{break;},
                        _=>{}
                    }
                }
            }
            update_handle.abort();
            update_version+=1;
            currQuestion+=1;
        }
        
        //grade questions
        //listen for next
        //send amount of responses
        //send results
    }

    
}
#[derive(Clone, Debug)]
pub enum GameCommand{
    ///Next will skip the timer and move on to the next question
    Next,
    ///"ban a player"
    RemovePlayer(String, String),
    ///Start behavior - start will start the game and send out the initial question
    Start
}
