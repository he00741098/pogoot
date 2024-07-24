sh ./crossbuild.sh
aws s3 cp ./target/aarch64-unknown-linux-gnu/release/pogootRefactoredRefactored s3://instanceaccess/pogoot
cd
sudo ssh -i "key/Awskey.pem" ec2-user@2600:1f14:1638:b401:2dbc:c41:10e2:c36f < /home/he00741098/Documents/GitHub/pogoot/pogootRefactoredRefactored/commands.txt
