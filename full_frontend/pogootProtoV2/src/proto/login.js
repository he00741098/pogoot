const {
  GameStartInfoResponse,
  LoginResponse,
  ManagerPlayerRequest,
  Notecard,
  NotecardFetchRequest,
  NotecardList,
  NotecardListUploadRequest,
  NotecardModifyRequest,
  NotecardUploadResponse,
  PogootAnswerRequest,
  PogootCreationRequest,
  PogootCreationResponse,
  PogootJoinCode,
  PogootQuestion,
  PogootQuestionList,
  PogootRequest,
  PogootResultsResponse,
  Progress,
  RoundResultResponse,
  UserLoginRequest,
  UserPasswordUpdateRequest,
  UserRegisterWithEmailRequest,
} = require("./pogoots_pb.js");
const {
  NotecardServiceClient,
  LoginServerClient,
  PogootPlayerServerClient,
} = require("./pogoots_grpc_web_pb.js");

let register_button = document.getElementById("RegisterButton");
let login_button = document.getElementById("LoginButton");

register_button.addEventListener("click", function(event) {
  event.preventDefault();
  let email = document.getElementById("emailRegister").value;
  let emailConfirm = document.getElementById("emailRegisterConfirm").value;
  let password = document.getElementById("passwordRegister").value;
  let passwordConfirm = document.getElementById(
    "passwordRegisterConfirm",
  ).value;

  if (email != emailConfirm) {
    alert("emails do not match");
    return;
  }
  if (password != passwordConfirm) {
    alert("passwords do not match");
    return;
  }

  let client = new LoginServerClient("http://localhost:80");
  let regReq = new UserRegisterWithEmailRequest();
  regReq.setEmail(email);
  regReq.setPassword(password);
  client.register(regReq, {}, (err, response) => {
    console.log(response);
  });
});

login_button.addEventListener("click", function(event) {
  event.preventDefault();

  let email = document.getElementById("emailLogin").value;
  let password = document.getElementById("passwordLogin").value;

  let client = new LoginServerClient("http://localhost:80");
  let regReq = new UserLoginRequest();
  regReq.setEmail(email);
  regReq.setPassword(password);
  client.login(regReq, {}, (err, response) => {
    console.log(response);
    if (response.array[0]) {
      cookie_set("auth", response.array[1]);
      cookie_set("username", email);
    } else {
      alert("Incorrect credentials");
    }
  });
});

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
