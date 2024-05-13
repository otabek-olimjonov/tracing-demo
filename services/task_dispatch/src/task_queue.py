import aioredis
import uuid
import logging

from utils.redis_utils import decode_redis_values

from model import TaskRequest

TASK_QUEUE_READ_BLOCK=500
TASK_QUEUE_READ_COUNT=20

logger = logging.getLogger(__name__)

class TaskQueue:
    _con: aioredis.Redis
    _queue_stream_name: str
    _queue_stream_group_name: str
    _consumer_id: str
    _consumer_group_ready: bool

    def __init__(self, redis_url, queue_name):
        self._con = aioredis.from_url(redis_url)

        self._queue_stream_name = queue_name
        self._queue_stream_group_name = f'{queue_name}__group_tq'
        self._consumer_id = str(uuid.uuid4())
        self._consumer_group_ready = False
    
    async def _create_consumer_group(self):
        try:
            await self._con.xgroup_create(
                self._queue_stream_name,
                self._queue_stream_group_name,
                '0', True,
            )
        except aioredis.exceptions.ResponseError as exc:
            if not str(exc).startswith("BUSYGROUP"):
                raise exc
    
        logger.info(f'✅ group consumer ready, stream={self._queue_stream_name}, consumer_id={self._consumer_id}')

        self._consumer_group_ready = True

    async def read_pending_tasks(self):
        if not self._consumer_group_ready:
            await self._create_consumer_group()

        res = await self._con.xreadgroup(
            self._queue_stream_group_name,
            consumername=self._consumer_id,
            streams={self._queue_stream_name: '>'},
            count=TASK_QUEUE_READ_COUNT,
            block=TASK_QUEUE_READ_BLOCK,
        )

        if len(res) == 0:
            return

        stream_messages = res[0][1]

        for (_, msg) in stream_messages:
            decoded_msg = decode_redis_values(msg)

            task_json = decoded_msg['task']

            logger.debug(f'read pending task `{task_json}`')
            
            try:
                yield TaskRequest.model_validate_json(task_json)
            except ValueError as exc:
                logger.error(f'invalid task definition `{task_json}`')
                logger.exception(exc)

        for (id, _) in stream_messages:
            await self._con.xack(self._queue_stream_name, self._queue_stream_group_name, id)

    async def unregister_consumer(self):
        logger.info(f'🧹 deleting group consumer, consumer_id={self._consumer_id}')
        await self._con.xgroup_delconsumer(self._queue_stream_name, self._queue_stream_group_name, self._consumer_id)
