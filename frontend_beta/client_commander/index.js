let socket1;
document.getElementById("startGame").onclick = function(){
  document.getElementById("commanderWaiting").style.display="block";
  document.getElementById("questionCreator").style.display="none";
  document.getElementById("commanderControl").innerHTML="<button id='next'>Skip!</button>";
  start_game()
};

function start_game(){

  socket1 = new WebSocket("wss://play.sweep.rs/commandSocket");
  let create_request = mapQuestionsToRequest();
  let game_request = "{\"request\":\"StartGame\",\"data\":\"StartGameData\"}"
  let next_request = 
    "{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}"
  let parsed_data;
  //Join token
  let token;
  let recon_pass;

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
        show_token();
        document.getElementById("startGame").onclick=function(){
          start_game();
        };
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

document.getElementById("actualStart").onclick = function(){
  start_game2()
}

function show_token(){
  document.getElementById("commanderControl").innerHTML="<h1>Token: "+token+"</h1><button id='startGame'>START</button>";
}
  function start_game2(){
    if (socket1!=null&&socket1.readyState==1){
    socket1.send("{\"request\":\"StartGame\",\"data\":\"StartGameData\"}");
    document.getElementById("commanderWaiting").style.display="none";
    document.getElementById("commanderControl").style.display="block";
    document.getElementById("next").onclick=function(){next_question()};
    }
  }
  function next_question(){
    socket1.send("{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}");
  }
}
