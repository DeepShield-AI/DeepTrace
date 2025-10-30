-- 随机生成 Request-ID
init = function()
    math.randomseed(os.time() + tonumber(tostring({}):sub(8), 16))
end

function gen_request_id()
    -- 生成一个不超过2^64-1的数字字符串
    local high = math.random(0, 4294967295)
    local low = math.random(0, 4294967295)
    -- 拼接成64位数字（字符串形式）
    return string.format("%010u%010u", high, low)
end

-- 随机选择 API 路径
function random_api()
    if math.random() < 0.5 then
        return "/api-a1"
    else
        return "/api-a2"
    end
end

-- 每个请求前调用
request = function()
    local req_id = gen_request_id()
    local path = random_api()
    wrk.headers["Request-ID"] = req_id
    print("Request API:", path, "Request-ID:", req_id)
    return wrk.format("GET", path)
end

-- 打印响应体
response = function(status, headers, body)
    print("Response:", body)
end