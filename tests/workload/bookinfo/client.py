import requests

# Bookinfo productpage 服务地址
url = "http://productpage:9080/productpage?u=normal"

for index in range(100):
    try:
        response = requests.get(url, timeout=5)
        print(f"请求 {index + 1} 成功:")
        print("状态码:", response.status_code)
    except Exception as e:
        print("请求失败:", e)