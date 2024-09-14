document.addEventListener("astro:page-load", () => {

  const {
    LoginResponse,
    UserLoginRequest,
    UserPasswordUpdateRequest,
    UserRegisterWithEmailRequest,
    Empty,
    date
  } = require("./pogoots_pb.js");
  const {
    LoginServerClient,
  } = require("./pogoots_grpc_web_pb.js");

  if(window.lastChecked == null && cookie_get("auth")!=null&&cookie_get("auth").length>2){
    window.lastChecked = new Date();
    check_boot_time();
  }else if(new Date() - window.lastChecked>300000){
    window.lastChecked = new Date();
    check_boot_time();
  }

function check_boot_time(){
    let client = new LoginServerClient("https://bigpogoot.sweep.rs");
    let req = new Empty();
    client.boot(req, {}, (err, response) => {
      console.log(response);
      let date = response.array[0];
      let sign_in_date = localStorage.getItem("signInDate");
      if(sign_in_date==null){
        reset_auth()
        return;
      }else{
        sign_in_date = new Date(sign_in_date);
        date = new Date(date);
        if(date-sign_in_date>0){
          //the server date is newer than the sign in date, auth is reset
          reset_auth()
          return;
        }else{
          //valid
        }
      }
    });
  }
  function reset_auth(){
    document.getElementById("account_button").icon = "account_circle";
    cookie_set("auth", "");
    send_alert("red", "Server Updated", "Suggested: reload and sign in again");
    if(window.onloadTurnstileCallback==null){
      window.onloadTurnstileCallback = function () {
        window.id = turnstile.render('.captcha', {
          sitekey: '0x4AAAAAAAg-XBCL8WUE5rPr',
          callback: function(token) {
            // console.log(`Challenge Success ${token}`);
            turn = token;
            window.turn = token;

            // console.log(`Challenge Success ${turn}`);
            // console.log("Turn3: "+window.turn);
          },
        });
      };
    }else{
      // console.log("Turn1: "+turn);
      turnstile.remove(window.id);
      window.onloadTurnstileCallback();
      // console.log("Turn2: "+turn);
    }
  }



  // console.log(window.turnstile);
  var turn = null;
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
            // console.log(`Challenge Success ${token}`);
            turn = token;
            window.turn = token;

            // console.log(`Challenge Success ${turn}`);
            // console.log("Turn3: "+window.turn);
          },
        });
      };
    }else{
      // console.log("Turn1: "+turn);
      turnstile.remove(window.id);
      window.onloadTurnstileCallback();
      // console.log("Turn2: "+turn);
    }

  }

  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";

  let register_button = document.getElementById("RegisterButton");
  let login_button = document.getElementById("LoginButton");

  let register_function = function (event) {
    event.preventDefault();

    let usernameReg = document.getElementById("usernameReg");
    let usernameConfirm = document.getElementById("usernameRegConfirm");
    let passReg = document.getElementById("passReg");
    let passConfirm = document.getElementById("passRegConfirm");
    if(window.turn==null){
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
    regReq.setTurn(window.turn);
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

        document.getElementById("account_button").icon = "settings";
        localStorage.setItem("library_cache","");
        cookie_set("auth", response.array[1]);
        cookie_set("username", email);
        let now = new Date();
        localStorage.setItem("signInDate", now.toUTCString());
        window.lastChecked = new Date();

        if(!document.URL.includes("create")){
          send_alert("green", "Login Success", "Redirecting...");
          redirect();
        }else{
          document.getElementById("login_popup").close();
          send_alert("green", "Login Success", "");
        }

      }
    });
  };
  let login_function = function (event) {
    event.preventDefault();
        let passLog = document.getElementById("usernameLogConfirm");
        let userLog = document.getElementById("passLogConfirm");
    if(window.turn==null){
      // console.log("Login Turn" + turn);
      // console.log("Window Turn" + window.turn);
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
    regReq.setTurn(window.turn);
    client.login(regReq, {}, (err, response) => {
      console.log(response);
      if (response.array[0]) {
        document.getElementById("account_button").icon = "settings";
        cookie_set("auth", response.array[1]);
        localStorage.setItem("updated","true");
        cookie_set("username", email);
        let now = new Date();
        localStorage.setItem("signInDate", now.toUTCString());
        window.lastChecked = new Date();

        if(!document.URL.includes("create")){
          redirect();
          send_alert("green", "Login Success", "Redirecting...");
        }else{
          document.getElementById("login_popup").close();
          send_alert("green", "Login Success", "");
        }
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


});
