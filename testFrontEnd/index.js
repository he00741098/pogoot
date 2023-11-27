let socket1 = new WebSocket("wss://play.sweep.rs/commandSocket");
let create_request = "{\"request\":\"CreateGame\",\"data\":{\"CreateGameData\":{\"questions\":[{\"question\":\"What is this question: 0\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"0\"]]},{\"question\":\"What is this question: 1\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"1\"]]},{\"question\":\"What is this question: 2\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"2\"]]},{\"question\":\"What is this question: 3\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"3\"]]},{\"question\":\"What is this question: 4\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"4\"]]},{\"question\":\"What is this question: 5\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"5\"]]},{\"question\":\"What is this question: 6\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"6\"]]},{\"question\":\"What is this question: 7\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"7\"]]},{\"question\":\"What is this question: 8\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"8\"]]},{\"question\":\"What is this question: 9\",\"answers\":[[false,\"Pog\"],[false,\"JFK\"],[false,\"Plog\"],[true,\"9\"]]}]}}}"
let game_request = "{\"request\":\"StartGame\",\"data\":\"StartGameData\"}"
let next_request = 
  "{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}"
let parsed_data;
//Join token
let token;
let recon_pass;
let login_token;

fetch('https://play.sweep.rs/login', {
    method: 'POST',
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
    },
    body: "{\"request\":\"Temp\",\"data\":{\"TempData\":\"Poggers\"}}"
})
   .then(response => response.text()
  ).then(response=>{
    console.log(response);
    login_token=response;
  });

socket1.onopen = function (){
  socket1.send(create_request);
}

socket1.onmessage = function(data){
  let new_data = data.data
  let parse_data = JSON.parse(new_data);
  switch (parse_data.response){
    case "gameCreatedSuccessResponse":
      console.log("Not default")
      console.log(parse_data);
      
      token = parse_data.data.GameCreationSuccessData[0];
      recon_pass = parse_data.data.GameCreationSuccessData[1];
      break;
    default:
      console.log(data);
      break;
}
}

socket1.onclose = function(){
  console.log("Socket 1 closed")
}

let socket2 = new WebSocket("wss://play.sweep.rs/pogootSocket");

socket2.onopen = function (){
  while (login_token==null){
  }
    let token_verify_request = "{\"request\":\"VerifyToken\",\"data\":{\"VerifyToken\":\""+login_token+"\"}}"
    socket2.send(token_verify_request);
}

socket2.onmessage = function(data){
  console.log(data);
  let new_data = data.data;
  let parse_data= JSON.parse(new_data);
  switch(parse_data.response){
    case "successResponse":
      if (parse_data.data=="SocketVerified"){
        socket2.send("{\"request\":\"SubscribeToGame\",\"data\":{\"SubscribeToGameData\":\""+token+"\"}}");
      }

      break;

    case "questionResponse":
      if (parse_data.data.QuestionData.question_number!=null){
        socket2.send("{\"request\":\"Answer\",\"data\":{\"AnswerData\":["+parse_data.data.QuestionData.question_number+","+Math.round(Math.random()*4)+"]}}")
      }
      break;
  }
}

socket2.onclose = function(){
  console.log("Socket 2 closed")
}

function start_game(){
  socket1.send("{\"request\":\"StartGame\",\"data\":\"StartGameData\"}");
}
function next_question(){
  socket1.send("{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}");
}
