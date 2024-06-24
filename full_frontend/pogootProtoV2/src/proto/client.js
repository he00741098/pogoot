document.addEventListener("astro:page-load", () => {
  if (document.URL.indexOf("create") < 0) {
    return;
  }
  const {
    Notecard,
    NotecardFetchRequest,
    NotecardList,
    NotecardListUploadRequest,
    NotecardModifyRequest,
    NotecardUploadResponse,
  } = require("./pogoots_pb.js");
  const {
    NotecardServiceClient,
  } = require("./pogoots_grpc_web_pb.js");

  var client = new NotecardServiceClient("https://bigpogoot.sweep.rs");

  let clone_count = 0;
  let cards = document.getElementById("cards");
  let example_created = document.getElementById("created");
  example_created.style.display = "none";
  notecardHeaderLeft.style.display = "none";
  notecardHeaderRight.style.display = "none";
  var creates = document.getElementById("create");
  var alertBox = document.getElementById("exampleAlert");
  var titleInput = document.getElementById("titleInput");
  alertBox.style.display = "none";
  var alerts = document.getElementById("Alerts");
  var front_input = document.getElementById("createFrontInput");
  var back_input = document.getElementById("createBackInput");
  var end_cap = document.getElementById("end_cap");
  var save_button = document.getElementById("save");
  var skip_back_input = false;
  var skip_front_input = false;
  let rows = 1;
  let refresh_inputs = [];
  let immediate_line_add = false;
  let last_value_length = 0;
  let entered_rows = 0;
  let last_line_count = 1;
  let char_count = Math.round(front_input.clientWidth/15.15);
  window.addEventListener("resize", function (ev){
    char_count = Math.round(front_input.clientWidth/15.15);
    for(input of refresh_inputs){
      flex_input(input);
    }
  });

  function flex_input(front_input){
    let input1 = front_input.value.split("\n");
    let rows = 0;
    for (var index = input1.length - 1; index >= 0; index--) {
      rows += 1;
      let length_temp = input1[index].length;
      while (length_temp > char_count) {
        rows += 1;
        length_temp -= char_count;
      }
    }

    front_input.rows = rows;
  }
  

  refresh_inputs.push(front_input);
  front_input.oninput = function(ev){
    flex_input(front_input)
  };

  refresh_inputs.push(back_input);
  back_input.oninput = function (ev) {
    flex_input(back_input)
  };



  end_cap.onclick = function (ev) {
    skip_front_input = true;
    skip_back_input = true;
    new_card(ev);
  };
  front_input.onblur = back_input.onblur = new_card;

  function new_card(ev) {
    if (
      (front_input.value == "" && !skip_front_input) ||
      (back_input.value == "" && !skip_back_input)
    ) {
      return;
    }
    let cloned_created = example_created.cloneNode();
    cloned_created.id = "created:" + clone_count;
    let cloned_front_input = front_input.cloneNode();
    let cloned_back_input = back_input.cloneNode();
    let cloned_left_header = document
      .getElementById("notecardHeaderLeft")
      .cloneNode(true);
    cloned_left_header.value = cloned_left_header.value + ":" + clone_count;
    let cloned_right_header = document
      .getElementById("notecardHeaderRight")
      .cloneNode(true);

    cloned_right_header.value = cloned_right_header.value + ":" + clone_count;
    let current_clone = clone_count;
    cloned_right_header.childNodes[1].onclick = function (ev) {
      console.log(ev);
      let node = document.getElementById("created:" + current_clone);
      cards.removeChild(node);
      reorder_all_entries();
    };
    cloned_left_header.childNodes[1].value = clone_count + 1;
    cloned_left_header.style.display = cloned_right_header.style.display =
      "grid";
    front_input.value = "";
    back_input.value = "";
    cloned_front_input.id = "front:" + clone_count;
    cloned_back_input.id = "back:" + clone_count;
    clone_count++;
    cloned_created.appendChild(cloned_left_header);
    cloned_created.appendChild(cloned_right_header);
    cloned_created.appendChild(cloned_front_input);
    cloned_created.appendChild(cloned_back_input);
    refresh_inputs.push(cloned_front_input)
    cloned_front_input.oninput = function (ev){
      flex_input(cloned_front_input)
    }
    refresh_inputs.push(cloned_back_input)
    cloned_back_input.oninput = function(ev){
      flex_input(cloned_back_input)
    }
    cloned_created.style.display = "grid";
    cards.insertBefore(cloned_created, creates.nextSibling);
    skip_back_input = false;
    skip_front_input = false;
    front_input.rows = 1;
    back_input.rows = 1;
    rows = 1;
    front_input.focus();
    console.log(cards.childNodes);
  }

  function reorder_all_entries() {
    let nodes = document.getElementsByClassName("notecardContainer");
    for (var node = nodes.length - 2; node > 0; node--) {
      let node_entry = nodes[node];
      node_entry.id = "temp_node:" + (node - 1);
      node_entry.childNodes[2].id = "temp_front:" + (node - 1);
      node_entry.childNodes[3].id = "temp_back:" + (node - 1);
    }

    for (var node = nodes.length - 2; node > 0; node--) {
      let node_entry = nodes[node];
      console.log(node);
      node_entry.id = "created:" + (node - 1);
      node_entry.childNodes[0].childNodes[1].value =
        nodes.length - 2 - node + 1;
      let current_clone = node - 1;
      node_entry.childNodes[1].childNodes[1].onclick = function (ev) {
        let noder = document.getElementById("created:" + current_clone);
        cards.removeChild(noder);
        reorder_all_entries();
      };
      node_entry.childNodes[2].id = "front:" + (node - 1);
      node_entry.childNodes[3].id = "back:" + (node - 1);
    }
    if (nodes.length == 2) {
      clone_count = 0;
    } else {
      clone_count = nodes.length - 2;
    }
  }

  save_button.onclick = function (ev) {
    var fronts = document.getElementsByClassName("frontNotecardInput");
    var backs = document.getElementsByClassName("backNotecardInput");
    if (titleInput.value.length < 1) {
      send_alert("red", "No Title", "Please add a title");
      return;
    }
    if (fronts.length < 2) {
      // console.log("no content");
      send_alert("red", "No Content", "Please add at least 1 card");
      return;
    }
    if (cookie_get("auth").length < 2 || cookie_get("username").length < 2) {
      send_alert("red", "Login", "Please Login before uploading");
    }

    var list = [];
    for (var i = 1; i < fronts.length; i++) {
      var notecard = new Notecard();
      // console.log(fronts[i].value);
      notecard.setFrontList([fronts[i].value]);
      notecard.setBackList([backs[i].value]);
      // console.log(notecard);
      list.push(notecard);
    }
    var notecardList = new NotecardList();
    notecardList.setNotecardsList(list);

    console.log(notecardList);

    var request = new NotecardListUploadRequest();
    request.setNotecards(notecardList);
    request.setAuthToken(cookie_get("auth"));
    request.setTitle(document.getElementById("titleInput").value);
    request.setDescription(document.getElementById("description").value);
    request.setTags(document.getElementById("tags").value);
    request.setSchool(document.getElementById("school"));
    request.setUsername(cookie_get("username"));
    console.log(request);
    uploader(request);
  };

  function uploader(request) {
    client.upload(request, {}, (err, response) => {
      if(err==null && response.array[0]){
        //the request was a success
        redirect_to("/notecards/"+response.array[1])
      }else{
        send_alert("red", "Upload Failed", "Please Try Again");
      }
    });
  }

  function send_alert(color, header, text) {
    let box = alertBox.cloneNode(true);
    box.style.outline = color + " solid 3px";
    // console.log(box.childNodes);
    box.childNodes[1].innerText = header;
    box.childNodes[3].innerText = text;
    box.style.display = "grid";
    alerts.appendChild(box);
    setTimeout(() => {
      alerts.removeChild(box);
    }, 5000);
  }
  function redirect() {
    window.location.href = "/library";
  }
  function redirect_to(url){
    window.location.href = url;
  }

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
        "auth=; SameSite=None; Secure; expires=" + date.toUTCString() + ";";
      document.cookie =
        "username=; SameSite=None; Secure; expires=" + date.toUTCString() + ";";
      document.cookie =
        "validCookies=; SameSite=None; Secure; expires=" +
        date.toUTCString() +
        ";";
    }
    cookies = document.cookie;
    document.cookie =
      key +
      "=" +
      value +
      "; SameSite=None; Secure; expires=" +
      date.toUTCString() +
      ";";
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
});
