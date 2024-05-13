# Greeter

This service generates greeting messages with optional custom name.

## Endpoints

| Endpoint | Function |
| --- | --- |
| /v1/greeter/greet | Generate generic greeting |
| /v1/greeter/greet/:name | Generate custom greeting |

### Response
The response is in json and contains a single `message` string field.

Example:
```json
{ "message": "Hello, Axel" }
```

## Useful Commands
### Development Watch
To run in development use the `dev` command.

```sh
npm run dev
```

### Building
To build the project use the `build` command.
```sh
npm run build
```

### Running
To run the server **after building** use the `start` command.

```sh
npm start
```

## Open Telemetry
This section will explain the OpenTelemetry (OTLP) dependencies in this project. OTLP is used for collecting traces and metrics for instrumenting and observing the service.

### Install Dependencies
```sh
npm install @opentelemetry/api @opentelemetry/auto-instrumentations-node @opentelemetry/exporter-trace-otlp-http @opentelemetry/sdk-node @opentelemetry/sdk-trace-node
```

### Tracing Implementation
The tracing implementation is in the `src/tracing.ts` file. This file initializes automatic tracing of our node service.