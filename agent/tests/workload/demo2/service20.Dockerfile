FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service20.go .

RUN go build -o server service20.go

EXPOSE 8080

CMD ["./server"]
