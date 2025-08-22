package main

import (
    "fmt"
    "io"
    "net/http"
    "time" // 添加此行
)

func withRequestID(w http.ResponseWriter, r *http.Request) (string, string, string) {
    requestID := r.Header.Get("RequestID")
    if requestID == "" {
        requestID = "unknown"
    }
    w.Header().Set("RequestID", requestID)

    pathType := r.Header.Get("PathType")
    if pathType == "" {
        pathType = "unknown"
    }
    w.Header().Set("PathType", pathType)

    serviceIdx := r.Header.Get("ServiceIdx")
    if serviceIdx == "" {
        serviceIdx = "unknown"
    }
    w.Header().Set("ServiceIdx", serviceIdx)

    return requestID, pathType, serviceIdx
}

func inject_delay(serviceIdx string) {
    if serviceIdx == "7" {
        // 模拟延迟
        time.Sleep(5 * time.Millisecond)
    }
}

func callService(url, requestID string, pathType string, serviceIdx string) (string, error) {
    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        return "", err
    }
    req.Header.Set("RequestID", requestID)
    req.Header.Set("PathType", pathType)
    req.Header.Set("ServiceIdx", serviceIdx)
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

func apiHandler(w http.ResponseWriter, r *http.Request) {
    requestID, pathType, serviceIdx := withRequestID(w, r)
    inject_delay(serviceIdx)

    if pathType == "4" {
        w.Header().Set("RequestID", requestID)
        fmt.Fprintf(w, "%s", "service7")
    }

    if pathType == "6" {
        service14Resp, err := callService("http://service14:8080/api14", requestID, pathType, serviceIdx)
        if err != nil {
            http.Error(w, "Failed to call service-c: "+err.Error(), http.StatusInternalServerError)
            return
        }
        w.Header().Set("RequestID", requestID)
        fmt.Fprintf(w, "%s %s", "service7", service14Resp)
    }
}

func main() {
    http.HandleFunc("/api7", apiHandler)
    fmt.Println("Server running on :8080")
    http.ListenAndServe(":8080", nil)
}
