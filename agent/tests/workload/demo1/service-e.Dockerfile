FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service-e.go .

RUN go build -o server service-e.go

EXPOSE 8080

CMD ["./server"]