document.addEventListener("astro:page-load", () => {
  // console.log(window.turnstile);
  let turn = null;
  let loginTime = cookie_get("auth");
  if (loginTime == null || loginTime.length <= 5) {
    console.log("Callback...")
    console.log(window.onloadTurnstileCallback);
    console.log(window.turnstile);
    if(window.onloadTurnstileCallback==null){
      window.onloadTurnstileCallback = function () {
        window.id = turnstile.render('.captcha', {
          sitekey: '0x4AAAAAAAg-XBCL8WUE5rPr',
          callback: function(token) {
            console.log(`Challenge Success ${token}`);
            turn = token;
          },
        });
      };
    }else{
      turnstile.remove(window.id);
      window.onloadTurnstileCallback();
    }

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

  let register_function = function (event) {
    event.preventDefault();

    let usernameReg = document.getElementById("usernameReg");
    let usernameConfirm = document.getElementById("usernameRegConfirm");
    let passReg = document.getElementById("passReg");
    let passConfirm = document.getElementById("passRegConfirm");
    if(turn==null){
      usernameReg.innerText = 
      usernameConfirm.innerText = 
      passReg.innerText = 
      passConfirm.innerText = "CAPTCHA Invalid";
    }else{
      usernameReg.innerText = 
      usernameConfirm.innerText = 
      passReg.innerText = 
      passConfirm.innerText = "";
    }
    let email = document.getElementById("emailRegister").value;
    let emailConfirm = document.getElementById("emailRegisterConfirm").value;
    let password = document.getElementById("passwordRegister").value;
    let passwordConfirm = document.getElementById(
      "passwordRegisterConfirm",
    ).value;

    if (email != emailConfirm) {
      // send_alert("red", "Emails do not match", "Please try again");
      document
        .getElementById("emailRegister")
        .setCustomValidity("Emails do not match");
      usernameReg.innerText = "Emails Do Not Match"

      document
        .getElementById("emailRegisterConfirm")
        .setCustomValidity("Emails do not match");
      usernameConfirm.innerText = "Emails Do Not Match";

      return;
    } else {
      document.getElementById("emailRegister").setCustomValidity("");
      document.getElementById("emailRegisterConfirm").setCustomValidity("");
      usernameReg.innerText = "";
      usernameConfirm.innerText = "";
    }

    if (email.length<=5){
      document
        .getElementById("emailRegister")
        .setCustomValidity("Email too short");
      usernameReg.innerText = "Email too short";
      return;
    }
    if (emailConfirm.length<=1){
      document
        .getElementById("emailRegisterConfirm")
        .setCustomValidity("Email too short");
      usernameConfirm.innerText = "Email too short";
      return;
    }
    // if (
    //   document.getElementById("emailRegister").validity.typeMismatch ||
    //   document.getElementById("emailRegister").validity.valueMissing
    // ) {
    //   send_alert("red", "Invalid email", "Please enter a valid email");
    //   console.log("Invalid email");
    //   document
    //     .getElementById("emailRegister")
    //     .setCustomValidity("Please Enter a valid email");
    //   document
    //     .getElementById("emailRegisterConfirm")
    //     .setCustomValidity("Please Enter a valid email");
    //   return;
    // } else {
    //   document.getElementById("emailRegister").setCustomValidity("");
    //   document.getElementById("emailRegisterConfirm").setCustomValidity("");
    // }

    if (password != passwordConfirm) {
      // send_alert("red", "Passwords do not match", "Re-input if needed!");
      document
        .getElementById("passwordRegister")
        .setCustomValidity("Passwords do not match");
      passReg.innerText = "Passwords do not match";
      document
        .getElementById("passwordRegisterConfirm")
        .setCustomValidity("Passwords do not match");
      passConfirm.innerText = "Passwords do not match";
      return;
    } else {
      document.getElementById("passwordRegister").setCustomValidity("");
      document.getElementById("passwordRegisterConfirm").setCustomValidity("");
      passReg.innerText = "";
      passConfirm.innerText = "";
    }
    if (password.length<6) {
      // send_alert(
      //   "red",
      //   "Please enter a password",
      //   "Use a new password for every account!",
      // );
      document
        .getElementById("passwordRegister")
        .setCustomValidity("Password must be 6+ characters");
      passReg.innerText = "Password must be 6+ characters";
      return;
    }
    if (passwordConfirm.length<6) {
      // send_alert(
      //   "red",
      //   "Please enter a password",
      //   "Use a new password for every account!",
      // );
      document
        .getElementById("passwordRegisterConfirm")
        .setCustomValidity("Password must be 6+ characters");
      passConfirm.innerText = "Password must be 6+ characters";
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
        // send_alert("red", "User already exists", "Try logging in");
        passConfirm.innerText = passReg.innerText = usernameReg.innerText = usernameConfirm.innerText = "User Already Exists. Try Logging in";
      }else if(response.array[1] == "Invalid Email"){
      document
        .getElementById("emailRegister")
        .setCustomValidity("Please enter an email address.");
      document
        .getElementById("emailRegisterConfirm")
        .setCustomValidity("Please enter an email address.");
        usernameReg.innerText = usernameConfirm.innerText = "Invalid Email";
      } else if (response.array[0]) {
        localStorage.setItem("library_cache","");
        send_alert("green", "Login Success", "Redirecting...");
        cookie_set("auth", response.array[1]);
        cookie_set("username", email);
        // localStorage.setItem("loginTime") = Date.now();
        redirect();
      }
    });
  };
  let login_function = function (event) {
    event.preventDefault();
        let passLog = document.getElementById("usernameLogConfirm");
        let userLog = document.getElementById("passLogConfirm");
    if(turn==null){
        passLog.innerText = "CAPTCHA Invalid";
        userLog.innerText = "CAPTCHA Invalid";
      return;
    }else{
        passLog.innerText = "";
        userLog.innerText = "";
    }
    let email = document.getElementById("emailLogin").value;
    let password = document.getElementById("passwordLogin").value;

    let client = new LoginServerClient("https://bigpogoot.sweep.rs");
    let regReq = new UserLoginRequest();
    regReq.setEmail(email);
    regReq.setPassword(password);
    regReq.setTurn(turn);
    client.login(regReq, {}, (err, response) => {
      console.log(response);
      if (response.array[0]) {
        send_alert("green", "Login Success", "Redirecting...");
        cookie_set("auth", response.array[1]);
        localStorage.setItem("updated","true");
        cookie_set("username", email);
        // localStorage.setItem("loginTime") = Date.now();
        redirect();
      } else {
        passLog.innerText = "Incorrect credentials";
        userLog.innerText = "Incorrect credentials";
        // send_alert("red", "Incorrect credentials", "");
      }
    });
  }


  register_button.addEventListener("click", register_function);
  login_button.addEventListener("click", login_function);
  document.getElementById("regform").onsubmit = register_function;
  document.getElementById("logform").onsubmit = login_function;

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


});
