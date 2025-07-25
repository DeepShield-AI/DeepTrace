FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service-a.go .

RUN go build -o server service-a.go

EXPOSE 8080

CMD ["./server"]