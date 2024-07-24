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
  //add the list

  let edit_button = document.getElementById("edit");
  edit_button.addEventListener("click", function (e) {
    console.log("Sending request....");
    let client = new NotecardServiceClient("https://bigpogoot.sweep.rs");
    var request = new NotecardModifyRequest();
    let notecardList = new NotecardList();
    request.setNotecards(notecardList);
    request.setAuthToken(cookie_get("auth"));
    request.setTitle(document.getElementById("titleInput").value);
    request.setDescription(document.getElementById("description").value);
    request.setTags(document.getElementById("tags").value);
    request.setSchool(document.getElementById("school"));
    request.setUsername(cookie_get("username"));
    request.setCfid(document.URL.split("notecards")[1].split("/")[1]);
    client.modify(edit_request);
  });

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
