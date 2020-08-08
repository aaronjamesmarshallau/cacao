#!/bin/bash
while getopts b:e:s: flag
do
    case "${flag}" in
        b) bucketname=${OPTARG};; # name of the bucket to create
        e) environment=${OPTARG};; # environment label to apply
        s) stackname=${OPTARG};; # name of stack to use
    esac
done

# Argument check. Abort if any not specified.
if [ -z "$bucketname" ] || [ -z "$environment" ] || [ -z "$stackname" ]
then
    echo "Usage: deploy <-b BUCKET_NAME> <-e ENVIRONMENT> <-s STACK_NAME>"
    exit 1
fi

echo "🚀 Ensuring stack $stackname exists and is up to date..."
error="$(aws cloudformation deploy --stack-name $stackname --template-body file://./bucket.yml --parameters ParameterKey=BucketName,ParameterValue=$bucketname ParameterKey=Environment,ParameterValue=$environment 2>&1)"

# Check the previous commands exit code using `$?`
if [ $? -ne 0 ]
then
    # If not 0, grep and print the error message and exit
    errorMessage="$(echo "$error" | grep -oP "An error occurred .*:\s\K(.*)")"
    echo "🔥 $errorMessage"
    exit 1
fi

while ! aws cloudformation describe-stacks --stack-name $stackname --query "Stacks[0].StackStatus" --output text | grep "_COMPLETE$" > /dev/null
do
    echo -n "."
    sleep 3
done
echo "✅"

echo ""
echo "Stack deployed successfully."

echo ""
echo "Ensuring the bucket contains the latest version of the initial files..."
aws s3 sync ./initial-bucket-contents/ s3://$bucketname/
