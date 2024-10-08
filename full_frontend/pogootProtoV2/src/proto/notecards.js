const {
  Notecard,
  // NotecardFetchRequest,
  NotecardList,
  // NotecardListUploadRequest,
  NotecardModifyRequest,
  // NotecardUploadResponse,
} = require("./pogoots_pb.js");
const { NotecardServiceClient } = require("./pogoots_grpc_web_pb.js");

document.addEventListener("astro:page-load", function () {
  if (document.URL.indexOf("notecards") < 1) {
    return;
  }
  let data = JSON.parse(document.getElementById("rawData").innerText);
  let url = document.URL.split("/");
  url = url[url.length-1];
  //attempt to grab progress data 
  let progressData = localStorage.getItem("LearnProgress"+url);
  let current_turn = 0;
  if (progressData!=null){
    progressData = JSON.parse(progressData);
  }else{
    progressData = [];
    for (var d of data){
      progressData.push({
        front:d.front,
        back:d.back,
        rights:0,
        wrongs:0,
        ratio:0,
        lastTurn:0
      });
    }
    // console.log(progressData)
  }
  //add the list

  let edit_button = document.getElementById("edit");
  edit_button.addEventListener("click", function (e) {

    //example data
    //{
    //    title:String
    //    description:String
    //    cfid:String
    //    tags:String
    //    school:String
    //    Notecards: [
    //        {
    //          fronts:[]
    //          backs:[]
    //        }
    //    ]
    //
    //}
    //
    let save_data = {
      title:document.getElementById("infoTitle").innerText,
      description:document.getElementById("infoDesc").innerText,
      cfid:document.URL.split("notecards")[1].split("/")[1],
      tags:document.getElementById("tags").innerText,
      school:document.getElementById("school").innerText,
      notecards:JSON.parse(document.getElementById("rawData").innerText)
    };
    // let client = new NotecardServiceClient("https://bigpogoot.sweep.rs");
    // var request = new NotecardModifyRequest();
    // let notecardList = new NotecardList();
    // request.setNotecards(notecardList);
    // request.setAuthToken(cookie_get("auth"));
    // request.setTitle(document.getElementById("titleInput").value);
    // request.setDescription(document.getElementById("description").value);
    // request.setTags(document.getElementById("tags").value);
    // request.setSchool(document.getElementById("school"));
    // request.setUsername(cookie_get("username"));
    // request.setCfid();
    // client.modify(edit_request);
    localStorage.setItem("to_edit", JSON.stringify(save_data));
    redirect_to("/create");
  });

  let learn_button = document.getElementById("learn");
  learn_button.addEventListener("click", function(e){

    if(data.length<5){
      send_alert("red", "Not Available", "Learn mode is not available for sets of less than 5 cards");
      return;
    }

    document.getElementById("notecardView").style.display = "none";
    document.getElementById("slot_container").style.marginTop = "0px";
    document.getElementById("footer-line").style.display = "none";
    document.getElementById("footer").style.display = "none";
    document.getElementById("learnView").style.display = "grid";
  });

  let close_button = document.getElementById("learnCloseButton");

  close_button.addEventListener("click", function(e){
    document.getElementById("notecardView").style.display = "block";
    document.getElementById("slot_container").style.marginTop = "150px";
    document.getElementById("footer-line").style.display = "block";
    document.getElementById("footer").style.display = "flex";
    document.getElementById("learnView").style.display = "none";
  });

  let questionText = document.getElementById("questionText");
  function show_short_answer(){
    document.getElementById("shortAnswer").style.display = "block";
    document.getElementById("progressButtons").style.display ="flex";
    document.getElementById("revealHintButton").style.display = "block";
    document.getElementById("multipleChoice").style.display = "none";
  }
  function show_multiple_choice(){
    document.getElementById("shortAnswer").style.display = "none";
    document.getElementById("progressButtons").style.display ="none";
    document.getElementById("revealHintButton").style.display = "none";
    document.getElementById("multipleChoice").style.display = "grid";
  }

  function generate_unique_randoms(max, start, count){
    let randoms = [];
    while (randoms.length<count){
      let random = Math.floor(Math.random()*(max-start))+start
      if (randoms.includes(random)){
        continue;
      }else{
        randoms.push(random);
      }
    }
    return randoms;
  }
  function sort_progress_data(progressData, current_turn){
    let positive_list = [];
    let negative_list = [];
    let last_seen = [];
    for (var p of progressData){
      let rights = p.rights;
      let wrongs = p.wrongs;
      if(wrongs==0){
        wrongs = 1;
      }
      let ratio = rights/wrongs;
      p.ratio = ratio;
      if(p.lastTurn!=0&&current_turn-p.lastTurn<3){
        last_seen.push(p);
      }else if (ratio>1){
        positive_list.push(p);
      }else{
        negative_list.push(p);
      }
    }
    positive_list = positive_list.sort((a,b)=>{
      return b.ratio*100 - a.ratio*100
    });
    negative_list = negative_list.sort((a,b)=>{
      return b.ratio*100 - a.ratio*100
    });
    last_seen = last_seen.sort((a,b)=>{
      return (current_turn-b.lastTurn) - (current_turn-a.lastTurn);
    });

    // let length = progressData.length;
    progressData = [];
    while(negative_list.length>0 && positive_list.length>0){
      progressData.push(negative_list.pop());
      progressData.push(positive_list.pop());
    }
    if(negative_list.length>0){
      progressData = progressData.concat(negative_list.reverse());
    }
    if (positive_list.length>0){
      progressData = progressData.concat(positive_list.reverse());
    }
    if(last_seen.length>0){
      progressData = progressData.concat(last_seen);
    }
    return progressData
  }
  //SORT PROGRESS DATA END


  let saves = 0;
  function show_next_card(){
    current_turn++;
    document.getElementById("shortAnswerInput").setCustomValidity("");
    document.getElementById("correctAnswer").style.display = "none";
    if(saves%3==0){
      localStorage.setItem("LearnProgress"+url, JSON.stringify(progressData));
    }

    //sort study queue based on study sets.

    //start learning proccess
    //sort the progressData by rights/wrongs
    let hinted = false;
    progressData = sort_progress_data(progressData, current_turn);
    progressData[0].lastTurn = current_turn;
    //SHORT ANSWER
    if((progressData[0].rights+progressData[0].wrongs>1 && progressData[0].ratio > 0.5)){
      //THE BACK IS THE ENGLISH PORTION
    questionText.innerText = progressData[0].back.join("\n");
      show_short_answer();
      document.getElementById("shortAnswerInput").onkeyup = function(e){
        if (e.key!="Enter"){
          return;
        }
        let answer = document.getElementById("shortAnswerInput").value;
        let correct = false;
        //THE FRONT IS THE SPANISH PART
        for(var b of progressData[0].front){
          if (answer == b){
            correct = true;
          }
        }
        if (correct){
          progressData[0].rights++;
          document.getElementById("correctAnswer").innerText = "Correct!";
          document.getElementById("correctAnswer").style.display = "block";
        }else{
          progressData[0].wrongs++;
          document.getElementById("shortAnswerInput").setCustomValidity("Incorrect");
          let reveal = "";
          if(progressData[0].front.length>1){
            document.getElementById("correctAnswer").innerText = "Answers:\n- "+progressData[0].front.join(" or\n- ");
          }else{
            document.getElementById("correctAnswer").innerText = "Answer:\n"+progressData[0].front[0];
          }
          document.getElementById("correctAnswer").style.display = "block";
        }
        document.getElementById("answerButton").innerText = "Continue";
        document.getElementById("shortAnswerInput").onkeyup = document.getElementById("answerButton").onclick = function(e){
          document.getElementById("answerButton").value = "Answer";
          document.getElementById("shortAnswerInput").value = "";
          show_next_card();
        }
      }
      document.getElementById("answerButton").onclick = function(e){
        let answer = document.getElementById("shortAnswerInput").value;
        let correct = false;
        for(var b of progressData[0].front){
          if (answer == b){
            correct = true;
          }
        }
        if (correct){
          progressData[0].rights++;
          document.getElementById("correctAnswer").innerText = "Correct!";
          document.getElementById("correctAnswer").style.display = "block";
        }else{
          progressData[0].wrongs++;
          document.getElementById("shortAnswerInput").setCustomValidity("Incorrect");
          let reveal = "";
          if(progressData[0].front.length>1){
            document.getElementById("correctAnswer").innerText = "Answers:\n- "+progressData[0].front.join(" or\n- ");
          }else{
            document.getElementById("correctAnswer").innerText = "Answer:\n"+progressData[0].front[0];
          }
          document.getElementById("correctAnswer").style.display = "block";
        }
        document.getElementById("answerButton").innerText = "Continue";
        document.getElementById("shortAnswerInput").onkeyup = function(e){
          if (e.key!="Enter"){
            return;
          }
          document.getElementById("answerButton").value = "Answer";
          document.getElementById("shortAnswerInput").value = "";
          show_next_card();
        }
        document.getElementById("answerButton").onclick = function(e){
          document.getElementById("answerButton").value = "Answer";
          document.getElementById("shortAnswerInput").value = "";
          show_next_card();
        }
      }

    }else{
      //MULTIPLE CHOICE OPTIONS

      questionText.innerText = progressData[0].front.join("\n");
      // document.getElementById("shortAnswerInput").onkeyup = function(e){}

      let randoms = generate_unique_randoms(progressData.length, 1, 3);
      let positions = generate_unique_randoms(5, 1 , 3);
      // console.log(randoms);
      // console.log(progressData)
      document.getElementById("choice"+positions[0]).innerText = progressData[randoms[0]].back;
      document.getElementById("choice"+positions[1]).innerText = progressData[randoms[1]].back;
      document.getElementById("choice"+positions[2]).innerText = progressData[randoms[2]].back;
      let sum = positions.reduce(
        (accumulator, currentValue) => accumulator + currentValue,
        0,
      );

      let remaining_position = 10-sum;
      document.getElementById("choice"+remaining_position).innerText = progressData[0].back;
      //add onclick listeners...

      document.getElementById("choice"+positions[0]).onclick =
        document.getElementById("choice"+positions[1]).onclick =
          document.getElementById("choice"+positions[2]).onclick = function(e){
            progressData[0].wrongs++;
            //reset the onclicks
            document.getElementById("choice"+positions[0]).onclick =
              document.getElementById("choice"+positions[1]).onclick =
                document.getElementById("choice"+positions[2]).onclick = function(e){}

            let continueHint = setTimeout(() => {
              send_alert("yellow", "Click the correct answer to continue", "");
            }, 2000);
            //clicck the right answer to progress
            document.getElementById("choice"+remaining_position).onclick = function(e){
              clearTimeout(continueHint);
              show_next_card();
              document.getElementById("choice"+positions[0]).style.outline = 
                document.getElementById("choice"+remaining_position).style.outline =
                  document.getElementById("choice"+positions[1]).style.outline = 
                    document.getElementById("choice"+positions[2]).style.outline = "lightgray solid 2px";
            };


            document.getElementById("choice"+positions[0]).style.outline = 
              document.getElementById("choice"+positions[1]).style.outline = 
                document.getElementById("choice"+positions[2]).style.outline = "red solid 2px";

            document.getElementById("choice"+remaining_position).style.outline = "green solid 2px";

          };

      document.getElementById("choice"+remaining_position).onclick = function(e){
        document.getElementById("choice"+remaining_position).onclick = function(e){}
        document.getElementById("choice"+remaining_position).style.outline = "green solid 2px";
        progressData[0].rights++;
        if (progressData[0].wrongs>0){
          progressData[0].wrongs--;
        }
        setTimeout(function(){
          document.getElementById("choice"+remaining_position).style.outline = "lightgray solid 2px";
          show_next_card()
        }, 800);
      };

      show_multiple_choice();
    }
  }
  if(data.length>4){
    show_next_card();
  }



});
