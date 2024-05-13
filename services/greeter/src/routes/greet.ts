import { Router, Request, Response } from 'express';

export const greet = Router();

greet.get('/', (_req: Request, res: Response) => {
    console.info('generic greet');

    const responseBody = {
        message: `Hello!`
    };

    return res.status(200).json(responseBody);
});

greet.get('/:name', (req: Request, res: Response) => {
    const name: string = req.params.name;
    console.info('custom greet', name);

    const responseBody = {
        message: `Hello, ${name}!`
    };

    return res.status(200).json(responseBody);
});
