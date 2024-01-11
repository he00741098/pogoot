# Pogoot

A WIP Notecard learning tool.

The architecture plan
- One central server intended to manage all logins - probably will need to be changed if there are a lot of users. Also maybe orchestrate scaling of total servers if more are up than necessary or more are needed
- Multiple regular servers intended to deal with the users. They will have to send requests to the central server to validate things like logins
- A list of server ips will be maintained by central server for the user client to decide to connect to
- Servers will be able to redirect users if preformance is degraded