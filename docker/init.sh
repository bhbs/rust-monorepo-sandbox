#!/bin/bash

set -e

echo "Starting LocalStack initialization..."

echo "Creating SNS topic..."
awslocal sns create-topic --name service_a_topic

echo "Creating SQS queue..."
awslocal sqs create-queue --queue-name service_b_queue

echo "Retrieving SNS topic ARN..."
TOPIC_ARN=$(awslocal sns list-topics --query "Topics[0].TopicArn" --output text)

echo "Retrieving SQS queue URL..."
QUEUE_URL=$(awslocal sqs get-queue-url --queue-name service_b_queue --query 'QueueUrl' --output text)

echo "Retrieving SQS queue ARN..."
QUEUE_ARN=$(awslocal sqs get-queue-attributes --queue-url $QUEUE_URL --attribute-names QueueArn --query 'Attributes.QueueArn' --output text)

echo "Subscribing SQS queue to SNS topic..."
SUBSCRIPTION_ARN=$(awslocal sns subscribe \
  --topic-arn $TOPIC_ARN \
  --protocol sqs \
  --notification-endpoint $QUEUE_ARN \
  --query 'SubscriptionArn' \
  --output text)

echo "Setting SQS queue policy..."
SQS_POLICY='{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"*"},"Action":"sqs:SendMessage","Resource":"'"$QUEUE_ARN"'","Condition":{"ArnLike":{"aws:SourceArn":"'"$TOPIC_ARN"'"}}}]}'
SQS_POLICY_ESCAPED=$(echo "$SQS_POLICY" | sed 's/"/\\"/g')
SQS_POLICY_ATTRIBUTES='{"Policy":"'"$SQS_POLICY_ESCAPED"'"}'
awslocal sqs set-queue-attributes \
  --queue-url "$QUEUE_URL" \
  --attributes "$SQS_POLICY_ATTRIBUTES"

echo "LocalStack initialization completed successfully."
