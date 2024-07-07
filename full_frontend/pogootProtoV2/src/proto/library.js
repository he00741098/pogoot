document.addEventListener("astro:page-load", () => {
  if (document.URL.indexOf("library") < 1) {
    return;
  }
  
  if(localStorage.getItem("updated")==null){
    localStorage.setItem("updated", false);
  }

  let refresher = document.getElementById("refresh");
  let main = document.getElementsByClassName("LibraryMain")[0];
  let header = document.getElementsByClassName("placeholderDate")[0].cloneNode(true);
  let search_bar = document.getElementsByClassName("accountSearchBar")[0];
  search_bar.value = "";
  


  refresher.addEventListener("click", function(e){
    localStorage.setItem("updated", "true");
    let nodes = main.childNodes;
    for (var i = nodes.length-1; i>=7;i--){
      let z = nodes[i];
      // console.log(z);
      main.removeChild(z);
    }
  let auth_cookie = cookie_get("auth");
  let username_cookie = cookie_get("username");
  if (auth_cookie==null || username_cookie==null || auth_cookie.length < 2 || username_cookie.length < 2) {
    send_alert("red", "Login", "Please Login To View Your Library");
  }else{
    let client = new NotecardServiceClient("https://bigpogoot.sweep.rs");
    let fetch_request = new NotecardLibraryRequest();
    fetch_request.setUsername(username_cookie);
    fetch_request.setAuthToken(auth_cookie);
    send_alert("green", "Loading...","");
    client.fetch(fetch_request, {}, (err, response) => {
      console.log(response);
      if (response==null){
        console.log("Load failed");
        send_alert("red", "Loading Failed", "Please reload");
        return;
      }
      localStorage.setItem("library_cache", JSON.stringify(response));
      localStorage.setItem("updated", "false");
      proccess_response(response);
    });
    //end of if statment
  }

  });

  console.log(search_bar);
  search_bar.addEventListener("input", (ev) => {
    let val = search_bar.value;
    if (val==null||val=='undefined'){
      val = "";
    }
    val = val.trim().toLowerCase();

    console.log("Searching... for "+val);
    let nodes = main.childNodes;
    for (var i = 7; i<nodes.length;i++){
      let z = nodes[i];
      console.log(z);
      if(z.classList[0]=="placeholderLibraryEntry"){
        console.log("matched:"+z.childNodes[3].childNodes[1].innerText +". / :"+z.childNodes[5].childNodes[1].innerText);
        if (!z.childNodes[3].childNodes[1].innerText.trim().toLowerCase().includes(val)&&!z.childNodes[5].childNodes[1].innerText.trim().toLowerCase().includes(val)){
          console.log("hidden");
          z.classList.add("hidden");
        }else{
          console.log("removed");
          z.classList.remove("hidden");
        }
      }else if(z.classList[0]=="placeholderLibraryEntryNoDesc"){
        if (!z.childNodes[3].childNodes[1].innerText.includes(val)){
          z.classList.add("hidden");
        }else{
          z.classList.remove("hidden");
        }
      }
    }
    let dates = document.getElementsByClassName("placeholderDate");
    let first = true;
    for (var d of dates){
      if(first){
        first = false;
        continue;
      }
      d.classList.remove("hidden");
    }

    let filtered_nodes = [];
    for (var i = 7; i<nodes.length;i++){
      if(!nodes[i].classList.contains("hidden")){
        filtered_nodes.push(nodes[i]);
      }
    }
    let cur_dex = 0;
    let last_dex = 0;
    for (var x of filtered_nodes){
      if(x.classList[0]=="placeholderDate"){
        if(cur_dex-last_dex==1){
          filtered_nodes[last_dex].classList.add("hidden");
        }
        if(cur_dex==filtered_nodes.length-1){
          filtered_nodes[cur_dex].classList.add("hidden");
        }
        last_dex = cur_dex;
      }
      cur_dex++;
    }
    console.log("Visible: "+filtered_nodes.length+" Dates:" + dates.length);
    if(filtered_nodes.length==dates.length-1){
      for(var d of dates){
        d.classList.add("hidden");
      }
    }

  });
  header.id="";
  let two = document.getElementById("libraryEntryOne");
  let three = document.getElementById("libraryEntryTwo");
  let clonedLibraryEntryOne = two.cloneNode(true);
  let clonedLibraryEntryTwo = three.cloneNode(true);
  clonedLibraryEntryOne.id="";
  clonedLibraryEntryTwo.id="";

  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";
  const {
    Notecard,
    NotecardFetchRequest,
    NotecardList,
    NotecardListUploadRequest,
    NotecardModifyRequest,
    NotecardUploadResponse,
    NotecardFetchResponse,
    NotecardLibraryRequest,
    NotecardLibraryList
  } = require("./pogoots_pb.js");
  const {
    NotecardServiceClient
  } = require("./pogoots_grpc_web_pb.js");


  function cookie_set(key, value) {
    var date = new Date();
    date.setTime(date.getTime() + 3 * 24 * 60 * 60 * 1000);
    let cookies = document.cookie;
    let split = cookies.split(";");
    let validCookies = false;
    for (var cookie of split) {
      if (cookie.trim().split("=")[0] == "validCookies") {
        validCookies = true;
        break;
      }
    }

    if (!validCookies) {
      console.log("no cookies");
      document.cookie =
        "auth=; SameSite=None; Secure; expires=" +
          date.toUTCString() +
          "; path=/";
      document.cookie =
        "username=; SameSite=None; Secure; expires=" +
          date.toUTCString() +
          "; path=/";
      document.cookie =
        "validCookies=; SameSite=None; Secure; expires=" +
          date.toUTCString() +
          "; path=/";
    }
    cookies = document.cookie;
    document.cookie =
      key +
        "=" +
        value +
        "; SameSite=None; Secure; expires=" +
        date.toUTCString() +
        "; path=/";
  }




  function cookie_get(key) {
    let cookies = document.cookie;
    let split = cookies.split(";");
    for (var cookie of split) {
      let cook = cookie.trim().split("=");
      if (cook[0] == key) {
        return cook[1];
      }
    }
  }

  function send_alert(color, header, text) {
    var alertBox = document.getElementById("exampleAlert");
    var alerts = document.getElementById("Alerts");
    let box = alertBox.cloneNode(true);
    box.style.outline = color + " solid 3px";
    console.log(box.childNodes);
    box.childNodes[1].innerText = header;
    box.childNodes[3].innerText = text;
    box.style.display = "grid";
    alerts.appendChild(box);
    setTimeout(() => {
      alerts.removeChild(box);
    }, 5000);
  }
  // function redirect() {
  //   window.location.href = "/library";
  // }


  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";
  if (document.URL.indexOf("library") < 1) {
    return;
  }

  let response="";
  let cached = false;
 if(localStorage.getItem("updated")=="false"){
    try{
      let library_data = localStorage.getItem("library_cache");
      if(library_data.length<5){
        cached = false;
      }else{
      response = JSON.parse(library_data);
      console.log("JSON parse successfully!");
      cached = true;
      }
    }catch(err){
      console.error("JSON parse failed...\n"+err);
    }
  }

  let auth_cookie = cookie_get("auth");
  let username_cookie = cookie_get("username");
  if (auth_cookie==null || username_cookie==null || auth_cookie.length < 2 || username_cookie.length < 2) {
    send_alert("red", "Login", "Please Login To View Your Library");
  }else if(cached){
      proccess_response(response)
  }else{
    let client = new NotecardServiceClient("https://bigpogoot.sweep.rs");
    let fetch_request = new NotecardLibraryRequest();
    fetch_request.setUsername(username_cookie);
    fetch_request.setAuthToken(auth_cookie);
    send_alert("green", "Loading...","");
    client.fetch(fetch_request, {}, (err, response) => {
      console.log(response);
      if (response==null){
        console.log("Load failed");
        send_alert("red", "Loading Failed", "Please reload");
        return;
      }
      localStorage.setItem("library_cache", JSON.stringify(response));
      localStorage.setItem("updated", "false");
      proccess_response(response);
    });
    //end of if statment
  }

  function proccess_response(response){
      let element_map = new Map();
      if(response.array[0].length<1){
        //there are no elements...
        send_alert("orange", "No Sets Found", "Create a new set and you will see it here!")

      }
      for(var b of response.array[0]){
        let title = b[0];
        let tag = b[2];
        let desc = b[3];
        let id = b[4];
        let date = b[5];
        let term_count = b[6];
        date = new Date(date);
        // date = date.toJSON().split("-");
        // let year = date[0];
        // let month = date[1];
        // let day = date[2].split("T")[0];
        if(desc==null||desc.length<1){
          let newChildNode = clonedLibraryEntryTwo.cloneNode(true);
          let termCountHolder = newChildNode.childNodes[1].childNodes[1];
          let notecardTitleHolder = newChildNode.childNodes[3].childNodes[1];
          // let descHolder = newChildNode.childNodes[5].childNodes[1];
          notecardTitleHolder.innerText = title;
          // descHolder.innerText = desc;
          if (term_count==1){
            termCountHolder.innerText = term_count+" Term";
          }else if (term_count<1){
            termCountHolder.innerText = "Empty";
          }else{
            termCountHolder.innerText = term_count+" Terms";
          }

          newChildNode.onclick = function (ev){
            window.location.href = "/notecards/"+id;
          };
          element_map.set(date, newChildNode);
        }else{
          let newChildNode = clonedLibraryEntryOne.cloneNode(true);
          let termCountHolder = newChildNode.childNodes[1].childNodes[1];
          let notecardTitleHolder = newChildNode.childNodes[3].childNodes[1];
          let descHolder = newChildNode.childNodes[5].childNodes[1];
          notecardTitleHolder.innerText = title;
          descHolder.innerText = desc;
          if (term_count==1){
            termCountHolder.innerText = term_count+" Term";
          }else if (term_count<1){
            termCountHolder.innerText = "Empty";
          }else{
            termCountHolder.innerText = term_count+" Terms";
          }


          newChildNode.onclick = function (ev){
            redirect_to("/notecards/"+id);
          };
          element_map.set(date, newChildNode);


        }

      }
      let sorted = element_map.keys();
      let sorting = [];
      for (var g of sorted){
        sorting.push(g);
      }
      sorted = sorting.sort(function(a,b){
        return new Date(b.date) - new Date(a.date);
      });
      sorted = sorted.reverse();
      let last_date = "";
      for (var c of sorted){
        let dater = c.toJSON().split("-");
        let year = dater[0];
        let month = dater[1];
        let day = dater[2].split("T")[0];
        if (last_date != month+"/"+day+"/"+year){
          let headernew = header.cloneNode(true);
          headernew.childNodes[1].innerHTML=month+"/"+day+"/"+year;
          main.appendChild(headernew);
        }
        last_date = month+"/"+day+"/"+year;
        main.appendChild(element_map.get(c));
      }
  }

});
