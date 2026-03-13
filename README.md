notes.sweep.rs

A WIP learning tool.

Goals -
- Notecards
- Kahoot thing
- Forum
- Question answer system
- Educational games

# File Structure
- /full_frontend contains all the frontend stuff - written with the AstroJS framework
- /pogootRefactoredRefactored contains the backend code - written with Rust

# Infrastructure
- Backend hosted on AWS ec2, frontend website hosted on cloudflare
- Note card data is compressed and stored in cloudflare R2
- User information stored in an sqlite database on Turso, this includes password hashes, username/email and notecard set ownership information
- Using protobuf/grpc for data transmission instead of just sending json - probably a mistake

# Current functionality
- Users can create accounts, and create notecard sets. They can also edit them.
- Other random front end things

# Goals
- Redesign frontend, create set suggestion algorithm, other QOL features.
- Incorporate some sort of version of kahoot (basic prototype backend included in ./pogoot)
- Reduce size of website + faster loading times in general

