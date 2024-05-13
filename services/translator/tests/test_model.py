
import unittest
import src.model as model
from pydantic import ValidationError

class MessageTests(unittest.TestCase):
    def test_valid_message(self):
        data = {
            'instance_id': 'instance-01',
            'client_id': 'client-01',
            'text': 'Hello I\'m just a test case.'
        }

        message = model.Message(**data)

        self.assertEquals(message.instance_id, 'instance-01')

    def test_invalid_message(self):
        data = {
            'instnce_id': 'instance-01',
            'client_id': 'client-01',
            'texxt': 'Invalid test field'
        }

        self.assertRaises(ValidationError, model.Message, **data)
