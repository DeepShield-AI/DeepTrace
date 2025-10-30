FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service-b.go .

RUN go build -o server service-b.go

EXPOSE 8080

CMD ["./server"]