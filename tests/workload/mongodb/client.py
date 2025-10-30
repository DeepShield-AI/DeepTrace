from decimal import Decimal
from pymongo import IndexModel, MongoClient, WriteConcern, ReadPreference
from pymongo.errors import PyMongoError
from bson import (
    Binary,
    Code,
    DBRef,
    Decimal128,
    Int64,
    ObjectId,
    Regex,
    Timestamp,
)
from faker import Faker
import random
import datetime
import struct
import math
import hashlib

fake = Faker()
MONGODB_URI = "mongodb://test:password@localhost:27017/"
DB_NAME = "admin"
COLLECTION_NAME = "large_documents"

def generate_geo_data():
    return {
        "type": "Point",
        "coordinates": [Decimal128(str(Decimal('146.725660'))),
        Decimal128(str(Decimal('11.543379')))]
    }

def generate_binary_data(size=1024):
    subtypes = {
        0: lambda: struct.pack(f'{size}H', *random.sample(range(65536), size)),
        1: lambda: Code(fake.pystr()).encode(),
        # 4: lambda: UUID().binary,
        5: lambda: hashlib.md5(fake.pystr().encode()).digest()
    }
    subtype = random.choice(list(subtypes.keys()))
    return Binary(subtypes[subtype](), subtype=subtype)

def generate_full_bson_doc():
    return {
        # 基础类型
        "double": random.uniform(-1e5, 1e5),
        "string": fake.text(),
        "object": {"nested": fake.pydict(3, True, [int, str])},
        "array": [fake.pyint(), fake.pystr(), None],
        
        # 二进制相关类型
        "binary": generate_binary_data(),
        "object_id": ObjectId(),
        "boolean": random.choice([True, False]),
        "date": datetime.datetime.now(datetime.timezone.utc),
        "null": None,
        "regex": Regex(fake.pystr()[:5], "i"),
        "db_ref": DBRef(COLLECTION_NAME, ObjectId()),
        
        # 特殊数值类型
        # "decimal128": Decimal128(str(random.uniform(-1e5, 1e5))),
        "int64": Int64(fake.pyint()),
        "timestamp": Timestamp(int(datetime.datetime.now(datetime.timezone.utc).timestamp()), 1),
        
        # 地理空间类型
        "geo_point": generate_geo_data(),
        
        # JavaScript类型
        "js_code": Code(f"function() {{ return {fake.pyint()}; }}"),
    }

def execute_all_operations(db):
    collection = db[COLLECTION_NAME].with_options(
        write_concern=WriteConcern(w="majority"),
        read_preference=ReadPreference.PRIMARY
    )
    
    try:
        # 插入操作（Insert OPCODE 2002）
        insert_result = collection.insert_many([generate_full_bson_doc() for _ in range(100)])
        print(f"Inserted {len(insert_result.inserted_ids)} docs")
        
        # 更新操作（Update OPCODE 2001）
        update_ops = [
            {"$set": {"counter": random.randint(1, 100)}},
            {"$inc": {"counter": 1}},
            {"$push": {"array": fake.pystr()}},
            {"$bit": {"flags": {"xor": random.randint(1, 255)}}}
        ]
        update_result = collection.update_many(
            {"boolean": True}, 
            update_ops[random.randint(0, len(update_ops)-1)]
        )
        print(f"Updated {update_result.modified_count} docs")
        
        # 查询操作（Query OPCODE 2004）
        cursor = collection.find({
            "$or": [
                {"geo_point": {"$near": generate_geo_data()}},
                {"decimal128": {"$lt": Decimal128("1000.5")}}
            ]
        }).limit(10)
        # print(f"Queried {cursor} docs")
        
        # 删除操作（Delete OPCODE 2006）
        delete_result = collection.delete_many({"counter": {"$gt": 50}})
        print(f"Deleted {delete_result.deleted_count} docs")
        
        # 聚合操作（Aggregate OPCODE 2011）
        pipeline = [
            {"$match": {"int64": {"$exists": True}}},
            {"$group": {"_id": "$boolean", "count": {"$sum": 1}}},
            {"$project": {"ratio": {"$divide": ["$count", Int64(100)]}}}
        ]
        agg_result = list(collection.aggregate(pipeline))
        print(f"Aggregated {len(agg_result)} docs")
        
        # 索引操作（CreateIndexes OPCODE 2010）
        index_result = collection.create_indexes([
            IndexModel([("geo_point", "2dsphere")], name="geo_idx"),
            IndexModel([("date", 1)], partialFilterExpression={"counter": {"$gt": 0}}),
            IndexModel([("object_id", 1)], expireAfterSeconds=3600)
        ])
        print(f"Created indexes: {index_result}")
        
        # KillCursors OPCODE 2007（通过游标自动回收触发）
        [doc for doc in collection.find().batch_size(2)]
        print("Cursor killed automatically after batch size limit reached")
        
    except PyMongoError as e:
        print(f"MongoDB operation failed: {str(e)}")

def main():
    for i in range(100):
        client = MongoClient(MONGODB_URI)
        try:
            db = client[DB_NAME]
            execute_all_operations(db)
        finally:
            client.close()

if __name__ == "__main__":
    main()