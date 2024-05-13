from enum import Enum
from pydantic import BaseModel

# ==== Task Requests ==============

class CommandType(str, Enum):
    translate = 'translate'

class Command(BaseModel):
    type: CommandType
    payload: str

class TaskRequest(BaseModel):
    instance_id: str
    client_id: str
    command: Command

# ==== Celery Tasks ==============

class TaskTranslate(BaseModel):
    instance_id: str
    client_id: str
    text: str