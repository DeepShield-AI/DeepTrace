FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service18.go .

RUN go build -o server service18.go

EXPOSE 8080

CMD ["./server"]
