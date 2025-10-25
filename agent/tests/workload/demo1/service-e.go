package main

import (
    "fmt"
    "net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
    requestID := r.Header.Get("Request-ID")
    if requestID == "" {
        requestID = "unknown"
    }
    w.Header().Set("Request-ID", requestID)
    fmt.Fprintf(w, "Request-ID:%s", requestID)
}

func main() {
    http.HandleFunc("/api-e1", handler)
    fmt.Println("service-e running on :8080")
    http.ListenAndServe(":8080", nil)
}