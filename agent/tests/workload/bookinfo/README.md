# Deployment

Run the following command to deploy the bookinfo microservices.  
The script installs Docker and Docker Compose, pulls the required images, and launches the stack with Docker Compose.

```bash
sudo bash deploy.sh
```

# Send Requests

The command below starts an interactive shell inside the `client` container and then issues frontend requests against the bookinfo application.

```bash
sudo bash client.sh
```

# Cleanup

Execute the following command to tear down the bookinfo microservices.

```bash
sudo bash clear.sh
```
