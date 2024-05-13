// NOTE(axel): It's important tracing is imported as early as possible to patch for auto tracing
import tracing from './tracing';

import dotenv from 'dotenv';
import router from './router';

import { app } from './app';
import { sigintHandler } from './exit';

dotenv.config();

tracing.initialize();

app.use('/greeter/v1', router);

const HOST = process.env.SERVICE_HOST || '0.0.0.0';
const PORT = parseInt(process.env.SERVICE_PORT || '') || 8080;

process.on('SIGINT', sigintHandler);

app.listen(PORT, HOST, () => {
    console.info(`🟢 server listening at ${HOST}:${PORT}`);
})
