FROM docker.1ms.run/python:3.10-slim

WORKDIR /app

COPY client.py .

RUN pip install requests

CMD ["tail", "-f", "/dev/null"]