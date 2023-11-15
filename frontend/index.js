/** @type {HTMLDivElement} */
const content = document.getElementById('content')

/** @type {WebSocket} */
let socket;
let intervalID; 

function update(msg) {
    let data = JSON.parse(msg.data)
    if (!includesFields(data, ["requestType"])) return;

    switch(data["requestType"]) {
        case ""
    }

    socket.send("")
}

function submitAnswer(optionNum) {
    let option = document.querySelector("#option" + optionNum)
    option.innerHTML
}

function includesFields(laObject, reqFields) {
    for(field in reqFields) {
        if (!laObject.includes(field)) return false
    }
    return true
}

function init() {
    socket = new WebSocket((location.protocol == "https:" ? "wss:" : "ws:") + "//" + location.host + "/ws")
    socket.onmessage = update;
}




