import bmemcached

mc = bmemcached.Client(['10.96.1.147:11211'])
for _ in range(1000):
    # 1. 键值操作
    mc.set('str_key', 'hello')             # 设置字符串
    mc.set('dict_key', {'a': 1, 'b': 2})   # 自动序列化复杂对象
    mc.set('expire_key', 'data', time=60)  # 60秒过期

    print("Get:", mc.get('str_key'))       # 获取值
    print("Get multi:", mc.get_multi(['str_key', 'dict_key']))

    # 2. 原子计数器
    mc.set('counter', 10)
    mc.incr('counter', 5)    # +5 → 15
    mc.decr('counter', 3)    # -3 → 12
    print("Counter:", mc.get('counter'))

    # 3. 存在性操作
    mc.add('new_key', 'value')   # 仅当键不存在时添加
    mc.get('str_key')
    
    # 4. replace
    mc.replace('str_key', 'new_value')  # 仅当键存在时替换

    mc.get('new_value')
    print("Appended:", mc.get('str_key'))

    # 5. 删除操作
    mc.delete('str_key')
    mc.delete_multi(['key1', 'key2'])

    # 6. CAS检查与设置（并发安全）
    mc.cas('cas_key', 'initial', 16)
    _ = mc.gets('cas_key')