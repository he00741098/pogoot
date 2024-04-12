const {
  GameStartInfoResponse,
  LoginResponse,
  ManagerPlayerRequest,
  Notecard,
  NotecardFetchRequest,
  NotecardList,
  NotecardListUploadRequest,
  NotecardModifyRequest,
  NotecardUploadResponse,
  PogootAnswerRequest,
  PogootCreationRequest,
  PogootCreationResponse,
  PogootJoinCode,
  PogootQuestion,
  PogootQuestionList,
  PogootRequest,
  PogootResultsResponse,
  Progress,
  RoundResultResponse,
  UserLoginRequest,
  UserPasswordUpdateRequest,
  UserRegisterWithEmailRequest,
} = require("./pogoots_pb.js");
const {
  NotecardServiceClient,
  LoginServerClient,
  PogootPlayerServerClient,
} = require("./pogoots_grpc_web_pb.js");

let client = new LoginServerClient("http://localhost:80");
let regReq = new UserRegisterWithEmailRequest();
regReq.setEmail("pogo@sweep.rs");
regReq.setPassword("Brug@sweep.rs");
client.register(regReq, {}, (err, response) => {
  console.log(response);
});
