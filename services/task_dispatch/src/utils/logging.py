import logging
import sys

def init():
    file_handler = logging.FileHandler(filename='debug.log')
    stdout_handler = logging.StreamHandler(stream=sys.stdout)

    handlers = [file_handler, stdout_handler]

    logging.basicConfig(
        level=logging.DEBUG,
        format='[%(asctime)s][%(filename)s:%(lineno)d][%(levelname)s] %(message)s',
        handlers=handlers
    )
