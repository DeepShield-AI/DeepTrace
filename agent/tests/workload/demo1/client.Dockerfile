FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./client.go .

RUN go build -o client client.go

EXPOSE 8080

CMD ["tail", "-f", "/dev/null"]