
const {NotecardUploadListRequest, NotecardUploadResponse, Notecard, Notecards} = require('../proto/pogoots_pb.js');
const {NotecardServiceClient} = require('../proto/pogoots_grpc_web_pb.js');

var notecardService = new NotecardServiceClient("http://localhost:8080");
var request = new NotecardUploadListRequest();

