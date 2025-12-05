#!/bin/bash

SERVICE_NAME="productpage"
NAMESPACE="default"

CLUSTER_IP=$(kubectl get svc $SERVICE_NAME -n $NAMESPACE -o=jsonpath='{.spec.clusterIP}')

PORT=$(kubectl get svc $SERVICE_NAME -n $NAMESPACE -o=jsonpath='{.spec.ports[0].port}')

if [ -z "$CLUSTER_IP" ] || [ -z "$PORT" ]; then
    echo "Failed to get the ClusterIP or port of the service: $SERVICE_NAME"
    exit 1
fi

URL="http://${CLUSTER_IP}:${PORT}/productpage"
echo "URL: $URL"

echo "Example response:"
curl "$URL"
echo "Example response end."

for i in $(seq 1 100); do
  response=$(curl -s "$URL")
  if ! echo "$response" | grep -q "reviews"; then
    echo "Response without 'reviews': $response"
    break
  fi
done
