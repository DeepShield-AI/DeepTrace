package main

import (
    "fmt"
    "io"
    "net/http"
)

func withRequestID(w http.ResponseWriter, r *http.Request) string {
    requestID := r.Header.Get("Request-ID")
    if requestID == "" {
        requestID = "unknown"
    }
    w.Header().Set("Request-ID", requestID)
    return requestID
}

func callService(url, requestID string) (string, error) {
    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        return "", err
    }
    req.Header.Set("Request-ID", requestID)
    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()
    body, err := io.ReadAll(resp.Body)
    if err != nil {
        return "", err
    }
    return string(body), nil
}

func api1Handler(w http.ResponseWriter, r *http.Request) {
    requestID := withRequestID(w, r)

    // 调用 service-b
    serviceBResp, err := callService("http://service-b:8080/api-b1", requestID)
    if err != nil {
        http.Error(w, "Failed to call service-b: "+err.Error(), http.StatusInternalServerError)
        return
    }

    // 调用 service-c
    serviceCResp, err := callService("http://service-c:8080/api-c1", requestID)
    if err != nil {
        http.Error(w, "Failed to call service-c: "+err.Error(), http.StatusInternalServerError)
        return
    }

    // 返回响应，包含 Request-ID
    w.Header().Set("Request-ID", requestID)
    fmt.Fprintf(w, "/api1 Request-ID:%s\nservice-b response: %s\nservice-c response: %s\n", requestID, serviceBResp, serviceCResp)
}

func api2Handler(w http.ResponseWriter, r *http.Request) {
    requestID := withRequestID(w, r)

    // 调用 service-d
    serviceDResp, err := callService("http://service-d:8080/api-d1", requestID)
    if err != nil {
        http.Error(w, "Failed to call service-d: "+err.Error(), http.StatusInternalServerError)
        return
    }

    // 返回响应，包含 Request-ID
    w.Header().Set("Request-ID", requestID)
    fmt.Fprintf(w, "/api2 Request-ID:%s\n%s\n", requestID, serviceDResp)
}

func main() {
    http.HandleFunc("/api-a1", api1Handler)
    http.HandleFunc("/api-a2", api2Handler)
    fmt.Println("Server running on :8080")
    http.ListenAndServe(":8080", nil)
}