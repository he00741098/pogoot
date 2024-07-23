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

  let data = JSON.parse(document.getElementById("rawData").innerText);
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
      if (document.URL.indexOf("notecards") > 0) {
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
});
