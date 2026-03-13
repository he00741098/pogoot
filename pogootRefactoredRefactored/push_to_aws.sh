sh ./crossbuild.sh
aws s3 cp ./target/aarch64-unknown-linux-gnu/release/pogootRefactoredRefactored s3://instanceaccess/pogoot
ssh -i "~/key/Awskey2.pem" ec2-user@2600:1f14:1638:b401:999a:bb8c:4d04:f845 < ~/Documents/Github/pogoot/pogootRefactoredRefactored/commands.txt
