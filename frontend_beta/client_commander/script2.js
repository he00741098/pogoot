document.getElementById("existingChoices").style.display="none";
document.getElementById("existingQuestions").style.display="none";
document.getElementById("commanderControl").style.display="none";
document.getElementById("commanderWaiting").style.display="none";
let questions = [];
let choices = [];
document.getElementById("addChoice").onclick = function(){
  choices.push([document.getElementById("correct").checked,document.getElementById("answerChoice").value]);
  document.getElementById("existingChoicesList").innerHTML = document.getElementById("existingChoicesList").innerHTML + "<li>"+document.getElementById("answerChoice").value+"</li>";
  document.getElementById("existingChoices").style.display="block";
  document.getElementById("correct").checked=false;
  document.getElementById("answerChoice").value="";
};
document.getElementById("addQuestion").onclick = function(){
  questions.push([document.getElementById("question").value,choices]);
  console.log("Doing something")
  document.getElementById("existingQuestionsList").innerHTML = document.getElementById("existingQuestionsList").innerHTML + "<li>"+document.getElementById("question").value+"<ul>"+document.getElementById("existingChoicesList").innerHTML+"</ul>"+"</li>";
  document.getElementById("existingQuestions").style.display="block";
  // document.getElementById("existingQuestionsList").innerHTML ="";
  document.getElementById("existingChoices").style.display="none";
  document.getElementById("existingChoicesList").innerHTML="";
  document.getElementById("answerChoice").value="";
  document.getElementById("question").value="";
  choices=[];
};


function mapQuestionsToRequest(){
  let starter = "{\"request\":\"CreateGame\",\"data\":{\"CreateGameData\":{\"questions\":["
  let ender = "]}}}"
  let index = 0;
  for (i of questions){
    starter+="{\"question\":\""+i[0]+"\",\"answers\":"+JSON.stringify(i[1])+"}"
    if (index+1<questions.length){
      starter+=","
    }
    index+=1
  }
  starter+=ender;
  return starter;
}

