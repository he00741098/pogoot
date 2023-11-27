let recon_pass;
let socket1;
let token;
let inJoinPhase = false;
document.getElementById("startGame").onclick = function(){
  // document.getElementById("commanderWaiting").style.display="block";
  document.getElementById("questionCreator").style.display="none";
  start_socket()
};

function show_token(){
  inJoinPhase=true;
  document.getElementById("commanderWaiting").style.display="block";
  document.getElementById("commanderWaiting").innerHTML="<h1>Token: "+token+"</h1><div id='actualStart'>START</div><br><div id='playerJoinDisplay'></div>";
  document.getElementById("actualStart").addEventListener("mousedown", (event) => {
    if (event.button==0){
      inJoinPhase=false;
      start_game()
    }
  });
}

function start_socket(){

  socket1 = new WebSocket("wss://play.sweep.rs/commandSocket");
  let create_request = mapQuestionsToRequest();
  let game_request = "{\"request\":\"StartGame\",\"data\":\"StartGameData\"}"
  let next_request = 
    "{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}"
  let parsed_data;
  //Join token

  socket1.onopen = function (){
    socket1.send(create_request);
    //atempt at keepalive
    // setInterval(function(){socket1.send("k")}, 500);
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
        recon_pass = parse_data.data.GameCreationSuccessData[1];
        break;
      case "PlayerJoinUpdateResponse":
        console.log("Player Join!");
        if (inJoinPhase){
          document.getElementById("playerJoinDisplay").innerHTML+="<div class='newPlayerBox'>"+ parse_data.data.PlayerJoinUpdateData+"</div>";
        }
      break;
      default:
        console.log(data);
        break;
    }
  }

  socket1.onclose = function(){
    console.log("Socket 1 closed")
  }

}

function start_game(){
  if (socket1!=null&&socket1.readyState==1){
    socket1.send("{\"request\":\"StartGame\",\"data\":\"StartGameData\"}");
    document.getElementById("commanderWaiting").style.display="none";
    document.getElementById("commanderControl").style.display="block";
    document.getElementById("commanderControl").innerHTML="<button id='next'>Skip!</button>";
    document.getElementById("next").onclick=function(){next_question()};
  }
}
function next_question(){
  console.log("NEXTING");
  socket1.send("{\"request\":\"NextQuestion\",\"data\":\"NextGameData\"}");
}
