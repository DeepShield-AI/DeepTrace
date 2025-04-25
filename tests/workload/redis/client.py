import redis

redis_client = redis.Redis(
    host='10.96.0.143',
    port=6379,
    db=0,                     # 数据库编号
    decode_responses=True     # 自动解码字节为字符串
)
for _ in range(1000):
    # 设置键值（带10秒过期时间）
    redis_client.set("user:1001", "Alice", ex=10)

    # 获取值
    value = redis_client.get("user:1001")
    print(value)  # 输出: Alice

    # 原子递增/递减
    redis_client.incr("counter")     # 自增1
    redis_client.incrby("counter",5) # 增加5

    # 存储用户信息
    redis_client.hset("user:profile:1001", mapping={
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com"
    })

    # 获取单个字段
    name = redis_client.hget("user:profile:1001", "name")

    # 获取全部字段
    profile = redis_client.hgetall("user:profile:1001")
    print(profile)  # 输出: {'name': 'Alice', 'age': '30', ...}

    # 左侧插入任务队列
    redis_client.lpush("task_queue", "task1", "task2")

    # 右侧弹出任务
    task = redis_client.rpop("task_queue")

    # 获取列表长度
    length = redis_client.llen("task_queue")