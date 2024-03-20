use crate::AwsSecrets;

pub mod pogoots {
    include!("../pogoot_refactored_refactored.rs");
}
use pogoots::*;
use std::pin::Pin;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};
use self::{login_server_server::LoginServer, notecard_service_server::NotecardService, pogoot_player_server_server::PogootPlayerServer};
use tokio_stream::{wrappers::ReceiverStream, Stream};

type Callback<C> = tokio::sync::oneshot::Sender<C>;
pub async fn start_serving(secrets:AwsSecrets){
    
    

}

pub enum NotecardDBRequest{
    ///Stores a notecard
    Store(NotecardListUploadRequest, Callback<NotecardUploadResponse>),
    ///Takes an ID and a callback
    Fetch(String, Callback<NotecardList>)
}

#[derive(Debug)]
struct NotecardServer{
    pub send_channel:mpsc::Sender<NotecardDBRequest>
}

enum LoginDBRequest{
    Register(UserRegisterWithEmailRequest, Sender<LoginResponse>),
    Login(UserLoginRequest, Sender<LoginResponse>),
    Update(UserPasswordUpdateRequest, Sender<LoginResponse>)
    
}

#[derive(Debug)]
struct LoginService{
    pub send_channel:mpsc::Sender<LoginDBRequest>
}

#[derive(Debug)]
struct PogootClientService;



#[tonic::async_trait]
impl NotecardService for NotecardServer{
    async fn upload(&self, request:tonic::Request<NotecardListUploadRequest>)->Result<tonic::Response<NotecardUploadResponse>, Status>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let send_result = self.send_channel.send(NotecardDBRequest::Store(request.into_inner(), tx)).await;
        let result = rx.await;
        if result.is_ok(){
            return Ok(Response::new(result.unwrap()))
        }
        //TODO:Better debug info
        Err(Status::new(tonic::Code::Internal, "Something went wrong"))
    }
    async fn modify(&self, request:tonic::Request<NotecardModifyRequest>)->Result<tonic::Response<NotecardUploadResponse>, Status>{
        unimplemented!()
    }
    async fn fetch(&self, request:tonic::Request<NotecardFetchRequest>)->Result<tonic::Response<NotecardList>, Status>{
        unimplemented!()
    }

}


#[tonic::async_trait]
impl LoginServer for LoginService{
  // rpc Login(UserLogin) returns (LoginResponse);
  // rpc Register(UserRegisterWithEmail) returns (LoginResponse);
  //rpc Update(UserPasswordUpdate) returns (LoginResponse);
    async fn login(&self, userlogin:Request<UserLoginRequest>)->Result<Response<LoginResponse>, Status>{
        unimplemented!()
    }
    async fn register(&self, userRegisterWithEmail:Request<UserRegisterWithEmailRequest>)->Result<Response<LoginResponse>, Status>{
        unimplemented!()
    }
    async fn update(&self, userNewInfo:Request<UserPasswordUpdateRequest>)->Result<Response<LoginResponse>, Status>{
        unimplemented!()
    }
}

// type Stream<T> = Pin<Box<dyn tokio_stream::Stream<Item = std::result::Result<T, Status>> + Send + 'static>>;
// type Streaming<T> = Request<tonic::Streaming<T>>;
#[tonic::async_trait]
impl PogootPlayerServer for PogootClientService{
    type AnswerStream = ReceiverStream<Result<PogootResultsResponse, Status>>;
    type EstablishQuestionStreamStream = Pin<Box<dyn Stream<Item = Result<PogootQuestion, Status>> + Send  + 'static>>;

    async fn join(&self, request:Request<PogootRequest>)->Result<Response<PogootJoinCode>, Status>{
        unimplemented!()
    }
    async fn answer(&self, request:Request<Streaming<PogootAnswerRequest>>)->Result<Response<ReceiverStream<Result<PogootResultsResponse, Status>>>, Status>{
        unimplemented!()
    }
    async
    fn establish_question_stream(&self, request:Request<PogootJoinCode>)
    ->Result<Response<Pin<Box<dyn Stream<Item = Result<PogootQuestion, Status>> + Send  + 'static>>>, Status>{

        unimplemented!()
    }
}


// service PogootPlayerServer{
//   //Starts conversation with the server
//   rpc Join(PogootRequest) returns (PogootJoinCode);
//   //Answering is essentially posting to the server
//   rpc Answer(stream PogootAnswer) returns (stream PogootResults);
//   rpc EstablishQuestionStream(PogootJoinCode) returns (stream PogootQuestion);
// }
// service LeadServer{
//   rpc Create(PogootCreationRequest) returns (PogootCreationResponse);
//   rpc FinishRound(Progress) returns (RoundResult);
//   rpc StartNextRound(Progress) returns (PogootQuestion);
//   rpc Manage(ManagePlayer) returns (Progress);
//   rpc PlayerJoins(Progress) returns (stream GameStartInfo);
// }
