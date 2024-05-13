
def decode_redis_values(value):
    return { p.decode('utf-8'): value.get(p).decode('utf-8') for p in value.keys() }
