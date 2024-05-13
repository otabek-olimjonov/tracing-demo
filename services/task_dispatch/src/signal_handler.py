import signal
import logging

logger = logging.getLogger(__name__)

class SignalHandler:
    should_exit = False

    def __init__(self):
        signal.signal(signal.SIGTERM, self._sigterm_handler)
        signal.signal(signal.SIGINT, self._sigint_handler)

    def _sigterm_handler(self, signum, _frame):
        logger.info(f'🚦 received SIGTERM ({signum})')
        self.should_exit = True

    def _sigint_handler(self, signum, _frame):
        logger.info(f'🚦 received SIGINT ({signum})')
        self.should_exit = True
