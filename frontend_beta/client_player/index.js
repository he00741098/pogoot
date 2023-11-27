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
let username;

document.getElementById("nameButton").onclick=function(){
  username = document.getElementById("nameInput").value;
  try{
  document.getElementById("usernamePhase").innerHTML="<h2>Setting Up...</h2>";

  fetch('https://play.sweep.rs/login', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: "{\"request\":\"Temp\",\"data\":{\"TempData\":\""+username+"\"}}"
  })
    .then(response => response.text()
    ).then(response=>{
      console.log(response);
      login_token=response;
      username_phase_over();
    });
  }catch(error){
    console.log(error);
    document.getElementById("usernamePhase").innerHTML="<h2>Error occured, Retrying...</h2>";
try{
  fetch('https://play.sweep.rs/login', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: "{\"request\":\"Temp\",\"data\":{\"TempData\":\""+username+"\"}}"
  })
    .then(response => response.text()
    ).then(response=>{
      console.log(response);
      login_token=response;
      username_phase_over();
    });
  }catch(error){
      console.log("failed");
    document.getElementById("usernamePhase").innerHTML="<h2>Login Failed, Reload and try again...</h2>";
    }
  }

}

function username_phase_over(){
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
          let arrayOfThings = [];
          let display = document.getElementById("playPhase");
          display.innerHTML="";
          display.innerHTML+="<h2>"+parse_data.data.QuestionData.question+"</h2>";
          display.innerHTML+="<ul>";
          for (i of parse_data.data.QuestionData.answers){
            display.innerHTML+="<li> <button id='"+i+"' >"+i+"</button> </li>"
            arrayOfThings.push([i, parse_data.data.QuestionData.question_number, parse_data.data.QuestionData.answers.indexOf(i)]);

          }
          display.innerHTML+="</ul>";
          document.getElementById("playPhase").style.display="block";
          // document.getElementById("playPhase").innerHTML=display;
          document.getElementById("waitPhase").style.display="none";
          console.log(arrayOfThings);
          for (i of arrayOfThings){
            let g = i;
            console.log("Adding things");
            console.log(g);
            document.getElementById(""+g[0]).addEventListener("click",(event)=>{
              console.log("answered: ");
              console.log(""+g[0]);
              socket2.send("{\"request\":\"Answer\",\"data\":{\"AnswerData\":["+g[1]+","+g[2]+"]}}")
              document.getElementById("playPhase").style.display="none";
              document.getElementById("waitPhase").innerHTML="<h2>Waiting...</h2>";
              document.getElementById("waitPhase").style.display="block";
            });
          }
        }
        break;
      case "gameUpdateResponse":
        if (parse_data.data.gameUpdateData!=null){
          //TODO: Streak, gains
          let info_text = "<h2> You have [";
          // document.getElementById("waitPhase").innerHTML="<h2> You have ["+parse_data.data.gameUpdateData[0]+"] points. with ["+parse_data.data.gameUpdateData+"]</h2>"
          info_text+=parse_data.data.gameUpdateData[0]+"] points. ";
          if (parse_data.data.gameUpdateData[1]!=""){
            info_text+=parse_data.data.gameUpdateData[1]+" is in front of you with ["+(parse_data.data.gameUpdateData[2]-parse_data.data.gameUpdateData[0])+"] more points.";
          }
          document.getElementById("waitPhase").innerHTML=info_text;
        }
        break;
    }
  }

  socket2.onclose = function(){
    console.log("Socket 2 closed")
  }

}
