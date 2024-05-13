import os
from time import sleep
from random import randint

from celery import Celery
from redis import Redis

from model import TaskTranslate, TaskTranslateResponse, TaskResult
import translate
import tracing as _

module_name = 'translator'

redis_service_url = os.environ.get('REDIS_SERVICE_URL', 'redis://localhost:6379')

queue = Redis.from_url(redis_service_url)
app = Celery(module_name, backend=redis_service_url, broker=redis_service_url)

def streaming_task_response(instance_id, client_id, translated):
    stream_message = TaskTranslateResponse(client_id=client_id, translated=translated)
    queue.xadd(f'streaming_tasks_queue:responses:{instance_id}', { 'json': stream_message.model_dump_json() })

@app.task()
def translate_text(task_json):
    task = TaskTranslate.model_validate_json(task_json)

    for word in str.split(task.text, None):
        translated = translate.translate_word(word)

        # NOTE: Simulate task taking some time to complete
        sleep(randint(10, 200) / 1000)

        print(f'translated {word} -> {translated}')
        streaming_task_response(task.instance_id, task.client_id, translated=translated)

    return TaskResult(successful=True).model_dump_json()

if __name__ == "__main__":
    app.worker_main([ 'worker', '--loglevel=DEBUG', '-E' ])
