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

func callAPI(api string, requestID string, outputFile *os.File) {
    url := fmt.Sprintf("http://service-a:8080/%s", api) // 假设服务运行在本地
    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        fmt.Println("Failed to create request:", err)
        return
    }

    // 设置 Request-ID
    req.Header.Set("Request-ID", requestID)

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        fmt.Println("Failed to call API:", err)
        return
    }
    defer resp.Body.Close()

    // 读取响应
    body, err := io.ReadAll(resp.Body)
    if err != nil {
        fmt.Println("Failed to read response:", err)
        return
    }

    // 写入到 output.txt
    output := fmt.Sprintf("Request to %s with Request-ID: %s\nResponse: %s\n\n", api, requestID, string(body))
    _, err = outputFile.WriteString(output)
    if err != nil {
        fmt.Println("Failed to write to output file:", err)
        return
    }

    fmt.Printf("Response from %s: %s\n", api, string(body))
}

func main() {
    // 打开输入文件
    file, err := os.Open("input.txt") // 假设文件名为 input.txt
    if err != nil {
        fmt.Println("Failed to open input file:", err)
        return
    }
    defer file.Close()

    // 打开输出文件
    outputFile, err := os.Create("output.txt") // 创建或覆盖 output.txt
    if err != nil {
        fmt.Println("Failed to create output file:", err)
        return
    }
    defer outputFile.Close()

    // 逐行读取文件内容
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text()) // 去除空格和换行符

        // 生成一个 Request-ID（这里简单用行号模拟）
        requestID := generateRequestID()

        // 根据文件内容调用不同的 API
        if line == "0" {
            callAPI("api-a1", requestID, outputFile)
        } else if line == "1" {
            callAPI("api-a2", requestID, outputFile)
        } else {
            fmt.Printf("Invalid input: %s (skipping)\n", line)
        }
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("Error reading input file:", err)
    }
}