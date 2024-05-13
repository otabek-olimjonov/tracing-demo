import { Router } from 'express';
import { greet } from './routes';

const router = Router();

router.use('/greet', greet);

export default router;