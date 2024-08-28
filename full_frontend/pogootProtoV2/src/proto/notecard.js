// let context_present = false;
document.addEventListener("astro:page-load", function () {
  // if (context_present) {
  //   console.log("context present!");
  //   return;
  // }
  // context_present = true;

  if (document.URL.indexOf("notecards") < 1) {
    return;
  }

  let url = window.location.href;
  let username = cookie_get("username");
  // console.log("Not logged in")

  let stars = [];
  for(var i=0;i<document.getElementsByClassName("termstar").length;i++){
    stars.push(false);
  }
  document.getElementById("full").onclick = function(e){
    e.stopPropagation()
    if (!document.fullscreenElement) {
      document.querySelector("main").requestFullscreen();
    }else{
      document.exitFullscreen();
    }
  }
  document.onfullscreenchange = (e)=>{
    if (!document.fullscreenElement) {
      document.getElementById("full").selected = false;
    }else{
      document.getElementById("full").selected = true;
    }
  };


  // let updateStarsInterval;
  get_progress();


  // console.log("activating!!!!!");
  let maxlen = document.getElementById("maxlen").innerHTML;
  let main = document.getElementById("main");
  let showing_front = true;
  main.onclick = function (e) {
    if (showing_front) {
      //show back
      document.getElementById("back" + current_index).style.display = "grid";
      document.getElementById("front" + current_index).style.display = "none";
      showing_front = false;
    } else {
      document.getElementById("back" + current_index).style.display = "none";
      document.getElementById("front" + current_index).style.display = "grid";
      showing_front = true;
    }
  };

  let current_index = 1;
  document.getElementById("notecardContainer" + current_index).style.display =
    "grid";
  document.getElementById("front" + current_index).style.display = "grid";
  document.getElementById("notecardContainer" + current_index).style.height =
    "100%";
  document.getElementById("notecardContainer" + current_index).style.width =
    "100%";
  let curdex = document.getElementById("currentindex");
  let rightarrow = document.getElementById("rightarrow");
  let leftarrow = document.getElementById("leftarrow");
  leftarrow.onclick = function (e) {
    document.getElementById("notecardContainer" + current_index).style.display =
      "none";
    document.getElementById("front" + current_index).style.display = "none";
    document.getElementById("back" + current_index).style.display = "none";
    document.getElementById("notecardContainer" + current_index).style.height =
      "0%";
    document.getElementById("notecardContainer" + current_index).style.width =
      "0%";
    current_index--;
    if (current_index < 1) {
      current_index = maxlen;
    }
    document.getElementById("notecardContainer" + current_index).style.display =
      "grid";
    document.getElementById("front" + current_index).style.display = "grid";
    document.getElementById("notecardContainer" + current_index).style.height =
      "100%";
    document.getElementById("notecardContainer" + current_index).style.width =
      "100%";
    curdex.innerHTML = current_index + "/";
    showing_front = true;
  };
  rightarrow.onclick = function (e) {
    document.getElementById("notecardContainer" + current_index).style.display =
      "none";
    document.getElementById("front" + current_index).style.display = "none";
    document.getElementById("back" + current_index).style.display = "none";
    document.getElementById("notecardContainer" + current_index).style.height =
      "0%";
    document.getElementById("notecardContainer" + current_index).style.width =
      "0%";
    current_index++;
    if (current_index > maxlen) {
      current_index = 1;
    }
    document.getElementById("notecardContainer" + current_index).style.display =
      "grid";
    document.getElementById("front" + current_index).style.display = "grid";
    document.getElementById("notecardContainer" + current_index).style.height =
      "100%";
    document.getElementById("notecardContainer" + current_index).style.width =
      "100%";
    curdex.innerHTML = current_index + "/";
    showing_front = true;
  };

  document.onkeydown = function (e) {
    if (e.key == "ArrowRight") {
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.display = "none";
      document.getElementById("front" + current_index).style.display = "none";
      document.getElementById("back" + current_index).style.display = "none";
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.height = "0%";
      document.getElementById("notecardContainer" + current_index).style.width =
        "0%";
      current_index++;
      if (current_index > maxlen) {
        current_index = 1;
      }
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.display = "grid";
      document.getElementById("front" + current_index).style.display = "grid";
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.height = "100%";
      document.getElementById("notecardContainer" + current_index).style.width =
        "100%";
      curdex.innerHTML = current_index + "/";
      showing_front = true;
    } else if (e.key == "ArrowLeft") {
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.display = "none";
      document.getElementById("front" + current_index).style.display = "none";
      document.getElementById("back" + current_index).style.display = "none";
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.height = "0%";
      document.getElementById("notecardContainer" + current_index).style.width =
        "0%";
      current_index--;
      if (current_index < 1) {
        current_index = maxlen;
      }
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.display = "grid";
      document.getElementById("front" + current_index).style.display = "grid";
      document.getElementById(
        "notecardContainer" + current_index,
      ).style.height = "100%";
      document.getElementById("notecardContainer" + current_index).style.width =
        "100%";
      curdex.innerHTML = current_index + "/";
      showing_front = true;
    } else if (e.key == " ") {
      if (document.URL.indexOf("notecards") > 0 && document.getElementById("learnView").style.display!="grid") {
        e.preventDefault();
      }

      if (showing_front) {
        //show back
        document.getElementById("back" + current_index).style.display = "grid";
        document.getElementById("front" + current_index).style.display = "none";
        showing_front = false;
      } else {
        document.getElementById("back" + current_index).style.display = "none";
        document.getElementById("front" + current_index).style.display = "grid";
        showing_front = true;
      }
    }
  };
  // document.removeEventListener("keydown", keycap);
  // document.addEventListener("keydown", keycap);

  function encode(array){
    let total_num = 0;
    for (var i = 0; i<array.length;i++){
      if (array[i]){
        total_num+=Math.pow(2, i);
      }
    }
    return total_num
  }
  function decode(num){
    let decoded = num.toString(2).split("").reverse().join("");
    decoded = decoded.split("");
    decoded = decoded.map((f)=>{if(f=="1"){return true}else{return false}});
    return decoded;
  }

  async function digestMessage(message) {
    const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
    const hashBuffer = await window.crypto.subtle.digest("SHA-256", msgUint8); // hash the message
    const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
    const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join(""); // convert bytes to hex string
    return hashHex;
  }


  let index = 0;
  for(var b of document.getElementsByClassName("termstar")){
    let star = b.childNodes[1];
    let ins = index;
    star.onclick = function(e){
      if(username==null){
        send_alert("red", "Not Logged In", "Login to save stars");
      }
      if (star.selected){
        //starred
        stars[ins] = true;
      }else{
        //unstarred
        stars[ins] = false;
      }
      star.loading=true;
      update_progress_store(star);
      // console.log(stars)
      // console.log(ins)
    };
    index++;
  }


  async function update_progress_store(star){
    if(username==null){
      console.log("Not logged in");
      // clearInterval(updateStarsInterval);
      return;
    }
    let important = url.split("/");
    important = important[important.length-1];
    let encodable = important+username;
    digestMessage(encodable).then((digestHex) => {
      fetcher(digestHex, star);
    });
  }
  async function fetcher(digestHex, star){
    localStorage.setItem("progressUpdate"+digestHex, Date.now());
    localStorage.setItem("progress"+digestHex, encode(stars));
    try {
      let response = await fetch("https://api.counterapi.dev/v1/"+digestHex+"/progress/set?count="+encode(stars));
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const json = await response.json();
      console.log(json);
      star.loading=false;
    } catch (error) {
      star.loading=false;
      console.error(error.message);
      send_alert("red", "Save Failed", "Try again later");
    }
  }
  function update_stars(){
    for(var i =0; i<stars.length;i++){
      if (stars[i]){
        document.getElementsByClassName("termstar")[i].childNodes[1].selected = true;
      }
    }
  }

  async function fetcher2(digestHex){
    if(localStorage.getItem("progressUpdate"+digestHex)!=null&&Date.now()-parseInt(localStorage.getItem("progressUpdate"+digestHex))>60000){
    try {
      let response = await fetch("https://api.counterapi.dev/v1/"+digestHex+"/progress");
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }
      const json = await response.json();
      console.log(json);
      stars = decode(json.count);
      update_stars();

      // updateStarsInterval= setInterval( update_progress_store, 10000)
    } catch (error) {
      console.error(error.message);
    }
    }else{
      let progress = localStorage.getItem("progress"+digestHex);
      progress = parseInt(progress);
      stars = decode(progress);
      update_stars();
    }
  }
  async function get_progress(){
    if(username==null){
      console.log("Not logged in");
      // clearInterval(updateStarsInterval);
      return;
    }
    let important = url.split("/");
    important = important[important.length-1];
    let encodable = important+username;
    digestMessage(encodable).then((digestHex) => {
      fetcher2(digestHex);
    });
  }



});
