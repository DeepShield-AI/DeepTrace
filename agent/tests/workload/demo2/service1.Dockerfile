FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service1.go .

RUN go build -o server service1.go

EXPOSE 8080

CMD ["./server"]
