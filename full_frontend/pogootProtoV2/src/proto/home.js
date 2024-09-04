
document.addEventListener("astro:page-load", () => {
  if (document.URL != "https://sweep.rs/" && document.url != "https://sweep.rs") {
    return;
  }
  console.log("home")
  var alertBox = document.getElementById("exampleAlert");
  alertBox.style.display = "none";

});
