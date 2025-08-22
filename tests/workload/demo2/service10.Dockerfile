FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service10.go .

RUN go build -o server service10.go

EXPOSE 8080

CMD ["./server"]
