FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service9.go .

RUN go build -o server service9.go

EXPOSE 8080

CMD ["./server"]
