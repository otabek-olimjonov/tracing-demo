FROM node:22.1.0

WORKDIR /app

COPY ./package.json ./package.json

RUN npm install --force

COPY . .

CMD ["npm", "run", "dev"]