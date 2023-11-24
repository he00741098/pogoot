# Pogoot Protocol

1. First step is for the game creator to connect to the wss://play.sweep.rs/commandSocket websocket
2. The commander will then send over the Questions in a pogootRequest
3. The game will respond with (pogootResponse { response: responseType::gameCreatedSuccessResponse, data: Data::GameCreationSuccessData(game_id, password) })
4. The password will be used to reconnect if the commander disconnects (*Not implemented yet*)
5. The game_id should be displayed or somehow conveyed to players
6. Player client will then send a temp request to get a token and set their username (username is tied with token) *Token will not be automatically deleted (Not implemented yet)*
7. Players connect to the wss://play.sweep.rs/pogootSocket websocket
8. Players will then send their login token
9. The player should now send a SubscribeToGame pogoot request

                requestType::SubscribeToGame=>{
                    //check if data is the right data type
                    match request.data{
                        Data::SubscribeToGameData(target)=>{

10. The game will start after a pogoot request of the StartGame variety
11. The game will then serve questions and info about the game
12. The commander can skip the 30 second (or all players answer the question) countdown with a Next request

# Notes

- Currently the regular player client has no idea how many questions there are
- The player will recieve the question and answer choices
- The commander has no way of reconnecting
- the player can reconnect but they must use the same username and save the previous token
