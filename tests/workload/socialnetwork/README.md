# Deployment

Run the following command to deploy the Social Network microservices.  
The script installs Docker and Docker Compose, pulls the required images, and launches the stack with Docker Compose.

```bash
sudo bash deploy.sh
```

# Send Requests

The command below starts an interactive shell inside the `wrk2` container and then issues frontend requests against the Social Network application.

```bash
sudo bash client.sh
```

# Cleanup

Execute the following command to tear down the Social Network microservices.

```bash
sudo bash clear.sh
```
