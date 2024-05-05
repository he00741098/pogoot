document.addEventListener("astro:page-load", () => {
  if (document.URL.indexOf("account") < 1) {
    return;
  }

  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";
  const {
    LoginResponse,
    UserLoginRequest,
    UserPasswordUpdateRequest,
    UserRegisterWithEmailRequest,
  } = require("./pogoots_pb.js");
  const {
    LoginServerClient,
  } = require("./pogoots_grpc_web_pb.js");

  let register_button = document.getElementById("RegisterButton");
  let login_button = document.getElementById("LoginButton");

  register_button.addEventListener("click", function (event) {
    event.preventDefault();
    let email = document.getElementById("emailRegister").value;
    let emailConfirm = document.getElementById("emailRegisterConfirm").value;
    let password = document.getElementById("passwordRegister").value;
    let passwordConfirm = document.getElementById(
      "passwordRegisterConfirm",
    ).value;

    if (email != emailConfirm) {
      send_alert("red", "Emails do not match", "Please try again");
      document
        .getElementById("emailRegister")
        .setCustomValidity("Emails do not match");

      // document.getElementById("emailRegister").innerHTML =
      //   "Emails do not match";
      document
        .getElementById("emailRegisterConfirm")
        .setCustomValidity("Emails do not match");

      return;
    } else {
      document.getElementById("emailRegister").setCustomValidity("");

      document.getElementById("emailRegisterConfirm").setCustomValidity("");
    }
    if (
      document.getElementById("emailRegister").validity.typeMismatch ||
      document.getElementById("emailRegister").validity.valueMissing
    ) {
      send_alert("red", "Invalid email", "Please enter a valid email");
      console.log("Invalid email");
      document
        .getElementById("emailRegister")
        .setCustomValidity("Please Enter a valid email");
      document
        .getElementById("emailRegisterConfirm")
        .setCustomValidity("Please Enter a valid email");
      return;
    } else {
      document.getElementById("emailRegister").setCustomValidity("");
      document.getElementById("emailRegisterConfirm").setCustomValidity("");
    }

    if (password != passwordConfirm) {
      send_alert("red", "Passwords do not match", "Re-input if needed!");
      document
        .getElementById("passwordRegister")
        .setCustomValidity("Passwords do not match");
      document
        .getElementById("passwordRegisterConfirm")
        .setCustomValidity("Passwords do not match");
      return;
    } else {
      document.getElementById("passwordRegister").setCustomValidity("");
      document.getElementById("passwordRegisterConfirm").setCustomValidity("");
    }
    if (document.getElementById("passwordRegister").validity.valueMissing) {
      send_alert(
        "red",
        "Please enter a password",
        "Use a new password for every account!",
      );
      return;
    }

    let client = new LoginServerClient("https://bigpogoot.sweep.rs");
    let regReq = new UserRegisterWithEmailRequest();
    regReq.setEmail(email);
    regReq.setPassword(password);
    if (document.getElementById("RegisterButton").disabled) {
      return;
    }
    document.getElementById("RegisterButton").disabled = true;
    setTimeout(
      'document.getElementById("RegisterButton").disabled=false;',
      5000,
    );

    client.register(regReq, {}, (err, response) => {
      console.log(response);
      if (response.array[1] == "User Logged In Already") {
        send_alert("red", "User already exists", "Try logging in");
      } else if (response.array[0]) {
        send_alert("green", "Login Success", "Redirecting...");
        cookie_set("auth", response.array[1]);
        cookie_set("username", email);
        redirect();
      }
    });
  });

  login_button.addEventListener("click", function (event) {
    event.preventDefault();

    let email = document.getElementById("emailLogin").value;
    let password = document.getElementById("passwordLogin").value;

    let client = new LoginServerClient("https://bigpogoot.sweep.rs");
    let regReq = new UserLoginRequest();
    regReq.setEmail(email);
    regReq.setPassword(password);
    client.login(regReq, {}, (err, response) => {
      console.log(response);
      if (response.array[0]) {
        send_alert("green", "Login Success", "Redirecting...");
        cookie_set("auth", response.array[1]);
        cookie_set("username", email);
        redirect();
      } else {
        send_alert("red", "Incorrect credentials", "");
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
});
