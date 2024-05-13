FROM node:20.10-alpine as builder
ENV NODE_ENV=development

ARG SERVICE_PORT=8080

RUN npm i -g typescript tsc

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY package.json /usr/src/app/
RUN npm install

COPY . /usr/src/app
RUN npm run build

EXPOSE ${SERVICE_PORT}

CMD NODE_ENV=production npm start
