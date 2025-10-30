FROM docker.1ms.run/golang:1.20 AS builder

WORKDIR /app
COPY ./service-d.go .

RUN go build -o server service-d.go

EXPOSE 8080

CMD ["./server"]