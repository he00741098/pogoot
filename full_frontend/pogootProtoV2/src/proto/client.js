// const {Notecard}
const {
  NotecardListUploadRequest,
  NotecardUploadResponse,
  Notecard,
  NotecardList,
} = require("./pogoots_pb.js");
const { NotecardServiceClient } = require("./pogoots_grpc_web_pb.js");

var client = new NotecardServiceClient("http://localhost:80");

// uploader();

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

let immediate_line_add = false;
let last_value_length = 0;
let entered_rows = 0;
let last_line_count = 1;
front_input.oninput = function(ev) {
  let input1 = front_input.value.split("\n");
  let rows = 0;
  for (var index = input1.length - 1; index >= 0; index--) {
    rows += 1;
    let length_temp = input1[index].length;
    while (length_temp > 34) {
      rows += 1;
      length_temp -= 34;
    }
  }

  front_input.rows = rows;
};
back_input.oninput = function(ev) {
  let input1 = back_input.value.split("\n");
  let rows = 0;
  for (var index = input1.length - 1; index >= 0; index--) {
    rows += 1;
    let length_temp = input1[index].length;
    while (length_temp > 34) {
      rows += 1;
      length_temp -= 34;
    }
  }

  back_input.rows = rows;
};
end_cap.onclick = function(ev) {
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
  cloned_right_header.childNodes[1].onclick = function(ev) {
    console.log(ev);
    let node = document.getElementById("created:" + current_clone);
    cards.removeChild(node);
    reorder_all_entries();
  };
  cloned_left_header.childNodes[1].value = clone_count + 1;
  cloned_left_header.style.display = cloned_right_header.style.display = "grid";
  front_input.value = "";
  back_input.value = "";
  cloned_front_input.id = "front:" + clone_count;
  cloned_back_input.id = "back:" + clone_count;
  clone_count++;
  cloned_created.appendChild(cloned_left_header);
  cloned_created.appendChild(cloned_right_header);
  cloned_created.appendChild(cloned_front_input);
  cloned_created.appendChild(cloned_back_input);
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
    node_entry.childNodes[0].childNodes[1].value = nodes.length - 2 - node + 1;
    let current_clone = node - 1;
    node_entry.childNodes[1].childNodes[1].onclick = function(ev) {
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

save_button.onclick = function(ev) {
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

  var list = [];
  for (var i = 1; i < fronts.length; i++) {
    var notecard = new Notecard();
    notecard.setFrontList(fronts[i].value);
    notecard.setBackList(backs[i].value);
    list.push(notecard);
  }
  var notecardList = new NotecardList();
  notecardList.setNotecardsList(list);

  var request = new NotecardListUploadRequest();
  request.setNotecards(notecardList);
  request.setAuthToken("1238946");
  uploader(request);
};

function uploader(request) {
  client.upload(request, {}, (err, response) => {
    console.log(response.getMessage());
  });
}

function send_alert(color, header, text) {
  let box = alertBox.cloneNode(true);
  box.style.outline = color + " solid 1px";
  box.style.backgroundColor = "white";
  // box.style
  console.log(box.childNodes);
  box.childNodes[1].innerText = header;
  box.childNodes[3].innerText = text;
  box.style.display = "grid";
  alerts.appendChild(box);
  setTimeout(() => {
    alerts.removeChild(box);
  }, 2000);
}