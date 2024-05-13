import os
import asyncio

import utils.logging as logging
import task_dispatch

logging.init()

TASK_QUEUE_STREAM_NAME = 'streaming_tasks_queue:requests'

REDIS_URL = os.environ.get('REDIS_URL', 'redis://localhost:6379')

if __name__ == "__main__":
    asyncio.run(task_dispatch.run(REDIS_URL, TASK_QUEUE_STREAM_NAME))
