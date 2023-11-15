use std::time::Duration;
use tokio::sync::oneshot::error::RecvError;
use tracing::info;
use axum::extract::ws::WebSocket;
use axum::extract::ws::Message;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
pub async fn websocketReceiverHandler(mut receiver: SplitStream<WebSocket>, forwardResponses:tokio::sync::mpsc::Sender<Result<(String, Message), axum::Error>>, username:String){
    loop{
        let msg = receiver.next().await;
        info!("Msg received?");
        match msg{
            Some(Ok(msg))=>{
                let result = forwardResponses.send(Ok((username.clone(),msg))).await;
                if result.is_err(){
                    info!("Send error");
                    return;
                }
            // info!("Websocket returned error in receiver handler, Probably disconnected");
            },
            Some(Err(msg))=>{
            info!("Websocket returned error in receiver handler, Probably disconnected");
                let senderResult = forwardResponses.send(Err(msg)).await;
                // if senderResult.is_err(){
                    return;
                // }
            },
            None=>{
                info!("recieved: None");
                return;
            }
    }
        //Not required right?
        // tokio::time::sleep(Duration::from_millis(250)).await;
    }
    
}
pub async fn websocketSendHandler(mut sink: SplitSink<WebSocket, Message>, mut sendResponses:tokio::sync::mpsc::Receiver<Message>){
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

pub async fn waitForCommander(commander:tokio::sync::oneshot::Receiver<WebSocket>)->Result<WebSocket, RecvError>{
    info!("Waiting for Commander");
    let b = commander.await?;
    Ok(b)
}
pub async fn startGame(starter:tokio::sync::oneshot::Receiver<bool>)->bool{
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
