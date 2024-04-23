# Rust Transformer Lambda Function

This project contains a Rust AWS Lambda function that uses the `rust-bert` library for sequence classification. It utilizes the Lambda Runtime and Simple Logger to facilitate interaction with AWS Lambda and logging capabilities, respectively.

## Project Overview

The Lambda function processes input text for sequence classification using a pre-trained BERT model from the `rust-bert` library. The input is expected to be a JSON object containing a text field.

## Prerequisites

- Rust and Cargo (latest stable version)
- Docker
- AWS CLI configured with appropriate permissions

## Installation

Clone the repository and navigate into the project directory:

```bash
git clone [URL_TO_REPOSITORY]
cd [PROJECT_DIRECTORY]
```

## Building the Project

### Local Build

To build the project locally, run:

```bash
cargo build --release
```

### Building with Docker

To build the Lambda function for deployment, use the provided Dockerfile:

```bash
docker build -t rust-transformer-lambda .
```

This Dockerfile is configured to build the Rust project in a containerized environment, ensuring compatibility with the AWS Lambda execution environment.

## Deployment

### Creating a Lambda Function

Create an AWS Lambda function using the AWS CLI:

```bash
aws lambda create-function --function-name rust-transformer-lambda \
--handler doesnt.matter \
--zip-file fileb://./target/lambda/release/bootstrap.zip \
--runtime provided.al2 \
--role arn:aws:iam::[ACCOUNT_ID]:role/[LAMBDA_EXEC_ROLE] \
--environment Variables={RUST_BACKTRACE=1} \
--memory-size 1024 \
--timeout 15
```

### Updating the Lambda Function

To update the function code, build the Docker image and extract the binary to upload to AWS Lambda:

```bash
docker run --rm --entrypoint cat rust-transformer-lambda /home/rust/src/target/x86_64-unknown-linux-musl/release/bootstrap > bootstrap
zip lambda.zip bootstrap

aws lambda update-function-code --function-name rust-transformer-lambda \
--zip-file fileb://lambda.zip
```

## Usage

To invoke the Lambda function, you can use the AWS CLI:

```bash
aws lambda invoke --function-name rust-transformer-lambda \
--payload '{"text":"example"}' response.json
```

Check `response.json` for the output.

## Function Behavior

The Lambda function `my_handler` expects an event with a JSON object containing a "text" key. It uses this text for sequence classification and returns the model's predictions as JSON.
