package main

import (
    "bufio"
    "fmt"
    "io"
    "math/rand"
    "net/http"
    "os"
    "strings"
    "time"
)

// 生成随机 64 位数字 ID
func generateRequestID() string {
    rand.Seed(time.Now().UnixNano()) // 设置随机种子
    high := rand.Uint32()            // 生成高 32 位
    low := rand.Uint32()             // 生成低 32 位
    return fmt.Sprintf("%010d%010d", high, low) // 拼接成 64 位数字字符串
}
func callAPI(pathType string, requestID string, serviceIdx string, outputFile *os.File) {
    url := ""
    if pathType == "1" || pathType == "2" {
        url = "http://service1:8080/api1"
    } else if pathType == "3" || pathType == "4" {
        url = "http://service2:8080/api2"
    } else if pathType == "5" || pathType == "6" {
        url = "http://service3:8080/api3"
    }
    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        fmt.Println("Failed to create request:", err)
        return
    }
    req.Header.Set("RequestID", requestID)
    req.Header.Set("PathType", pathType)
    if serviceIdx != "" {
        req.Header.Set("ServiceIdx", serviceIdx)
    }

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        fmt.Println("Failed to call API:", err)
        return
    }
    defer resp.Body.Close()

    body, err := io.ReadAll(resp.Body)
    if err != nil {
        fmt.Println("Failed to read response:", err)
        return
    }

    output := fmt.Sprintf("Request pathType %s serviceIdx %s with Request-ID: %s\nResponse: %s\n\n", pathType, serviceIdx, requestID, string(body))
    _, err = outputFile.WriteString(output)
    if err != nil {
        fmt.Println("Failed to write to output file:", err)
        return
    }

    fmt.Printf("Response from pathType %s serviceIdx %s: %s\n", pathType, serviceIdx, string(body))
}

func main() {
    // 打开输入文件
    file, err := os.Open("input.txt")
    if err != nil {
        fmt.Println("Failed to open input file:", err)
        return
    }
    defer file.Close()

    outputFile, err := os.Create("output.txt")
    if err != nil {
        fmt.Println("Failed to create output file:", err)
        return
    }
    defer outputFile.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        fields := strings.Fields(line)
        pathType := ""
        serviceIdx := ""
        if len(fields) > 0 {
            pathType = fields[0]
        }
        if len(fields) > 1 {
            serviceIdx = fields[1]
        }
        // fmt.Printf("Processing pathType: %s, serviceIdx: %s\n", pathType, serviceIdx)
        requestID := generateRequestID()
        callAPI(pathType, requestID, serviceIdx, outputFile)
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("Error reading input file:", err)
    }
}