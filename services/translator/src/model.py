from pydantic import BaseModel

class TaskTranslate(BaseModel):
    instance_id: str
    client_id: str
    text: str

class TaskTranslateResponse(BaseModel):
    client_id: str
    translated: str

class TaskResult(BaseModel):
    successful: bool
    