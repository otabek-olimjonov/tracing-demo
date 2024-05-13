FROM python:3.10-alpine

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

COPY ./requirements.txt requirements.txt

RUN pip install -r requirements.txt

COPY ./src .

ENV REDIS_SERVICE_URL=redis://redis-server:6379

ENV OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-endpoint:4317
ENV OTEL_EXPORTER_OTLP_INSECURE=true
ENV OTEL_EXPORTER_OTLP_PROTOCOL=grpc

CMD [ "python", "__main__.py" ]
