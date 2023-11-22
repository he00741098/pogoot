let socket1 = new WebSocket("wss://play.sweep.rs/pogootSocket");
let socket2 = new WebSocket("wss://play.sweep.rs/pogootSocket"); 
let gameCreate = '{"requestType":"CreateGame","data":{"QuestionUpload":{"questions":[{"question":"What is this question: 0","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"0"]]},{"question":"What is this question: 1","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"1"]]},{"question":"What is this question: 2","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"2"]]},{"question":"What is this question: 3","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"3"]]},{"question":"What is this question: 4","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"4"]]},{"question":"What is this question: 5","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"5"]]},{"question":"What is this question: 6","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"6"]]},{"question":"What is this question: 7","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"7"]]},{"question":"What is this question: 8","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"8"]]},{"question":"What is this question: 9","answers":[[false,"Pog"],[false,"JFK"],[false,"Plog"],[true,"9"]]}]}}}';
socket1.onmessage = function(pog){console.log(pog)};
socket2.onmessage = function(pog){console.log(pog)};
function game_test()
let subscribe_one = '{"requestType":{"SubscribeToGame":"'+440476+'"},"data":{"Username":"Poggooooo"}}';
let startGame = '{"requestType":{"StartGame":"'+ting+'"},"data":"None"}';
