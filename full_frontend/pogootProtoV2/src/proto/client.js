// const {Notecard}
const {
  NotecardListUploadRequest,
  NotecardUploadResponse,
  Notecard,
  NotecardList,
} = require("./pogoots_pb.js");
const { NotecardServiceClient } = require("./pogoots_grpc_web_pb.js");

var client = new NotecardServiceClient("http://localhost:80");
var list = [];
var notecard = new Notecard();
notecard.setFrontList(["pog"]);
notecard.setBackList(["brog"]);
list.push(notecard);

var notecardList = new NotecardList();
notecardList.setNotecardsList(list);

var request = new NotecardListUploadRequest();
request.setNotecards(notecardList);
request.setAuthToken("1238946");

function uploader() {
  client.upload(request, {}, (err, response) => {
    console.log(response.getMessage());
  });
}
