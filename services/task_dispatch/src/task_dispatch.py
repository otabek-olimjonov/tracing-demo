import logging

from celery import Celery

from model import (
    TaskTranslate,
    CommandType
)

from task_queue import TaskQueue
from signal_handler import SignalHandler

logger = logging.getLogger(__name__)

_task_command_map = {
    CommandType.translate: lambda task: ('translator.translate_text', TaskTranslate(instance_id=task.instance_id, client_id=task.client_id, text=task.command.payload))
}

def task_dispatch(task, celery):
    gen_celery_task = _task_command_map.get(task.command.type, None)

    if gen_celery_task == None:
        logger.error(f'unrecognized task, type={task.command.type}')
        return
    
    (task_name, task_payload) = gen_celery_task(task)

    logger.debug(f'send celery task, name={task_name}, payload={task_payload}')
    celery.send_task(task_name, [task_payload.model_dump_json()])

async def run(redis_url, queue_name):
    celery = Celery(backend=redis_url, broker=redis_url)
    task_queue = TaskQueue(redis_url, queue_name)

    signal_handler = SignalHandler()

    logger.info('📃 task_dispatch started')

    while not signal_handler.should_exit:
        async for task in task_queue.read_pending_tasks():
            task_dispatch(task, celery)

    await task_queue.unregister_consumer()

    logger.info('🌷 graceful exit')
