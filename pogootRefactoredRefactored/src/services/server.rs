use crate::services::user_manage::UserManager;
use crate::AwsSecrets;
use base64::prelude::*;
use tokio::sync::Mutex;
use tonic::transport::ServerTlsConfig;
pub mod pogoots {
    include!("../pogoot_refactored_refactored.rs");
}
use self::{
    login_server_server::LoginServer, notecard_service_server::NotecardService,
    pogoot_player_server_server::PogootPlayerServer,
};
use pogoots::login_server_server::LoginServerServer;
use pogoots::notecard_service_server::NotecardServiceServer;
use pogoots::*;
use std::pin::Pin;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{
    transport::{Identity, Server},
    Request, Response, Status, Streaming,
};

type Callback<C> = tokio::sync::oneshot::Sender<C>;
///Entry point into starting the service.
pub async fn start_serving(mut secrets: AwsSecrets) {
    let addr = "[::]:443".parse().unwrap();
    let cert = BASE64_STANDARD
        .decode(std::mem::take(&mut secrets.cloudflare_cert))
        .unwrap();
    let key = BASE64_STANDARD
        .decode(std::mem::take(&mut secrets.cloudflare_key))
        .unwrap();
    // let greeter = MyGreeter::default();
    // let greeter = GreeterServer::new(greeter);
    let mut con = crate::services::database::new_connection(secrets.clone()).await;
    while con.is_none() {
        println!("Turso connection failed, Trying again");
        tokio::time::sleep(Duration::new(5, 0)).await;
        con = crate::services::database::new_connection(secrets.clone()).await;
    }

    let con = con.unwrap();
    let clonecon = con.clone();

    //repeat connection attempts every 5 seconds
    let user_manager = UserManager {
        tokens: Arc::new(Mutex::new(HashMap::new())),
        users: Arc::new(Mutex::new(HashMap::new())),
        connection: con,
    };
    let (ltx, lrx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        user_manager.proccess_user_auth(lrx).await;
    });
    let login_server = LoginService {
        send_channel: ltx.clone(),
    };
    let login_server = LoginServerServer::new(login_server);

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        crate::services::notecard::upload_proccessor(clonecon, rx, ltx, secrets).await;
    });
    let notecard_server = NotecardServer { send_channel: tx };
    let notecard_server = NotecardServiceServer::new(notecard_server);

    println!("Server listening on {}", addr);
    let result = Server::builder()
        .tls_config(ServerTlsConfig::new().identity(Identity::from_pem(&cert, &key)))
        .unwrap()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(notecard_server))
        .add_service(tonic_web::enable(login_server))
        .serve(addr)
        .await;
    println!("Result: {:?}", result);
}

//The proto implementations
pub enum NotecardDBRequest {
    ///Stores a notecard
    Store(NotecardListUploadRequest, Callback<NotecardUploadResponse>),
    ///Takes an ID and a callback
    Fetch(String, Callback<NotecardList>),
    ///Takes an ID and a callback
    Modify(NotecardModifyRequest, Callback<NotecardUploadResponse>),
}

#[derive(Debug)]
struct NotecardServer {
    pub send_channel: mpsc::Sender<NotecardDBRequest>,
}

pub enum LoginDBRequest {
    Register(UserRegisterWithEmailRequest, Callback<LoginResponse>),
    Login(UserLoginRequest, Callback<LoginResponse>),
    Update(UserPasswordUpdateRequest, Callback<LoginResponse>),
    ///Token, Username
    VerifyToken(String, String, Callback<bool>),
}

#[derive(Debug)]
struct LoginService {
    pub send_channel: mpsc::Sender<LoginDBRequest>,
}

#[derive(Debug)]
struct PogootClientService;

#[tonic::async_trait]
impl NotecardService for NotecardServer {
    ///The request is forwarded through a channel.
    ///A store request will be processed by the database system
    ///The NotecardList will be serialized and compressed.
    ///Then it will be assigned an id and it will be compressed. The id will be stored in turso
    ///and the data will be stored eventually in cloudflare R2
    ///The storing will be proccessed in batches. Each upload will be forwarded to at least 2 other
    ///servers. The 2 other servers will store replicas. During the batch storage, one server,
    ///likely the most powerful, will verify its contents and then store everything
    ///The other servers will be notified and will flush their memory.
    ///Input = Upload request, Result = Stored in cloudflare
    async fn upload(
        &self,
        request: tonic::Request<NotecardListUploadRequest>,
    ) -> Result<tonic::Response<NotecardUploadResponse>, Status> {
        println!("Recieved upload request");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let send_result = self
            .send_channel
            .send(NotecardDBRequest::Store(request.into_inner(), tx))
            .await;
        if send_result.is_err() {
            println!("Upload Channel Failed");
            return Err(Status::new(tonic::Code::Internal, "Channel send failed"));
        }
        let result = rx.await;
        if result.is_ok() {
            let result = result.unwrap();
            //Print for testing
            println!("Note Card upload processed: {:?}", result);
            return Ok(Response::new(result));
        }
        //TODO:Better debug info
        Err(Status::new(tonic::Code::Internal, "Something went wrong"))
    }
    async fn modify(
        &self,
        request: tonic::Request<NotecardModifyRequest>,
    ) -> Result<tonic::Response<NotecardUploadResponse>, Status> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let send_result = self
            .send_channel
            .send(NotecardDBRequest::Modify(request.into_inner(), tx))
            .await;
        if send_result.is_err() {
            return Err(Status::new(
                tonic::Code::Internal,
                "Modify request failed when send channel failed to send",
            ));
        }

        let result = rx.await;
        if result.is_ok() {
            let result = result.unwrap();
            //Print for testing
            println!("Note Card modify processed: {:?}", result);
            return Ok(Response::new(result));
        }
        //TODO:Better debug info
        Err(Status::new(tonic::Code::Internal, "Something went wrong"))
    }
    async fn fetch(
        &self,
        request: tonic::Request<NotecardFetchRequest>,
    ) -> Result<tonic::Response<NotecardList>, Status> {
        unimplemented!()
    }
}

#[tonic::async_trait]
impl LoginServer for LoginService {
    // rpc Login(UserLogin) returns (LoginResponse);
    // rpc Register(UserRegisterWithEmail) returns (LoginResponse);
    //rpc Update(UserPasswordUpdate) returns (LoginResponse);
    async fn login(
        &self,
        userlogin: Request<UserLoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let userlogin = userlogin.into_inner();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let login_dbrequest = LoginDBRequest::Login(userlogin, tx);
        let send_result = self.send_channel.send(login_dbrequest).await;
        if send_result.is_err() {
            println!("Login Send Channel Failed");
            return Err(Status::new(tonic::Code::Internal, "Login Channel Failed"));
        }
        let callback_result = rx.await;
        if callback_result.is_err() {
            println!("Callback Channel failed");
            return Err(Status::new(
                tonic::Code::Internal,
                "Callback Channel failed",
            ));
        }
        let callback_result = callback_result.unwrap();
        Ok(Response::new(callback_result))
    }
    async fn register(
        &self,
        user_register_with_email: Request<UserRegisterWithEmailRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let user_register = user_register_with_email.into_inner();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let register_dbrequest = LoginDBRequest::Register(user_register, tx);
        let send_result = self.send_channel.send(register_dbrequest).await;
        if send_result.is_err() {
            println!("Login Send Channel Failed");
            return Err(Status::new(tonic::Code::Internal, "Login Channel Failed"));
        }
        let callback_result = rx.await;
        if callback_result.is_err() {
            println!("Callback Channel failed");
            return Err(Status::new(
                tonic::Code::Internal,
                "Callback Channel failed",
            ));
        }
        let callback_result = callback_result.unwrap();
        Ok(Response::new(callback_result))
    }
    async fn update(
        &self,
        userNewInfo: Request<UserPasswordUpdateRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        unimplemented!()
    }
}

///The pogoot Client Service
///Handle joins, answers, and stream questions
#[tonic::async_trait]
impl PogootPlayerServer for PogootClientService {
    type AnswerStream = ReceiverStream<Result<PogootResultsResponse, Status>>;
    type EstablishQuestionStreamStream =
        Pin<Box<dyn Stream<Item = Result<PogootQuestion, Status>> + Send + 'static>>;

    async fn join(
        &self,
        request: Request<PogootRequest>,
    ) -> Result<Response<PogootJoinCode>, Status> {
        unimplemented!()
    }
    async fn answer(
        &self,
        request: Request<Streaming<PogootAnswerRequest>>,
    ) -> Result<Response<ReceiverStream<Result<PogootResultsResponse, Status>>>, Status> {
        unimplemented!()
    }
    async fn establish_question_stream(
        &self,
        request: Request<PogootJoinCode>,
    ) -> Result<
        Response<Pin<Box<dyn Stream<Item = Result<PogootQuestion, Status>> + Send + 'static>>>,
        Status,
    > {
        unimplemented!()
    }
}
