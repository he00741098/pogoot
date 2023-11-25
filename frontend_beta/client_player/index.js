document.getElementById("playPhase").style.display="none";
document.getElementById("tokenPhase").style.display="none";
document.getElementById("waitPhase").style.display="none";
let token = "";
let login_token = "";
document.getElementById("tokenButton").onclick = function(){
  token = document.getElementById("tokenInput").value;
  document.getElementById("tokenPhase").style.display="none";
  document.getElementById("waitPhase").style.display="block";
  joinGame();
}

document.getElementById("nameButton").onclick=function(){
  fetch('https://play.sweep.rs/login', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: "{\"request\":\"Temp\",\"data\":{\"TempData\":\""+document.getElementById("nameInput").value+"\"}}"
  })
    .then(response => response.text()
    ).then(response=>{
      console.log(response);
      login_token=response;
    });
  document.getElementById("usernamePhase").style.display="none";
  document.getElementById("tokenPhase").style.display="block";
}

function joinGame(){
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
          let display = "";
          display+="<h2>"+parse_data.data.QuestionData.question+"</h2>";
          display+="<ul>";
          for (i of parse_data.data.QuestionData.answers){
            display+="<li> <button onclick='answer(+"+parse_data.data.QuestionData.question_number+","+parse_data.data.QuestionData.answers.indexOf(i)+"+)'>"+i+"</button> </li>"
          }
          display+="</ul>";
          document.getElementById("playPhase").style.display="block";
          document.getElementById("playPhase").innerHTML=display;
          document.getElementById("waitPhase").style.display="none";
        }
        break;
      case "gameUpdateResponse":
        if (parse_data.data.LeaderBoardUpdate!=null){

        }
      break;
    }
  }

  socket2.onclose = function(){
    console.log("Socket 2 closed")
  }

  function answer(questionNumber, answer){
          socket2.send("{\"request\":\"Answer\",\"data\":{\"AnswerData\":["+questionNumber+","+answer+"]}}")
        document.getElementById("playPhase").style.display="none";
        document.getElementById("waitPhase").style.display="block";
  }
}
