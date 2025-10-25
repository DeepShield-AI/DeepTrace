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

func handler(w http.ResponseWriter, r *http.Request) {
    requestID := withRequestID(w, r)

    // 调用 service-d
    serviceEResp, err := callService("http://service-e:8080/api-e1", requestID)
    if err != nil {
        http.Error(w, "Failed to call service-e: "+err.Error(), http.StatusInternalServerError)
        return
    }

    // 返回响应，包含 Request-ID
    fmt.Fprintf(w, "service-d response: %s\nservice-e response: %s\n", requestID, serviceEResp)
}

func main() {
    http.HandleFunc("/api-d1", handler)
    fmt.Println("service-d running on :8080")
    http.ListenAndServe(":8080", nil)
}