document.addEventListener("astro:page-load", () => {
  if (document.URL.indexOf("library") < 1) {
    return;
  }

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
  function redirect() {
    window.location.href = "/library";
  }


  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";
  if (document.URL.indexOf("library") < 1) {
    return;
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
    client.fetch(fetch_request, {}, (err, response) => {
      console.log(response);
    });
  }
});