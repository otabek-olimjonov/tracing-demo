import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-grpc";
import { NodeSDK } from "@opentelemetry/sdk-node";
import { BasicTracerProvider, ConsoleSpanExporter, SimpleSpanProcessor } from "@opentelemetry/sdk-trace-node";
import { ExpressInstrumentation, ExpressLayerType } from "@opentelemetry/instrumentation-express";
import { HttpInstrumentation } from "@opentelemetry/instrumentation-http";
import { Resource } from '@opentelemetry/resources';
import { SEMRESATTRS_SERVICE_NAME, SEMRESATTRS_SERVICE_VERSION } from '@opentelemetry/semantic-conventions'
import { getNodeAutoInstrumentations } from "@opentelemetry/auto-instrumentations-node";
import { diag, DiagConsoleLogger, DiagLogLevel } from "@opentelemetry/api";

class Tracing {
    private sdk?: NodeSDK = undefined;
    private exporter = new OTLPTraceExporter();
    private provider = new BasicTracerProvider({
        resource: new Resource({
            [SEMRESATTRS_SERVICE_NAME]: process.env.npm_package_name,
            [SEMRESATTRS_SERVICE_VERSION]: process.env.npm_package_version
        })
    });

    private instrumentations = [
        new HttpInstrumentation(),
        new ExpressInstrumentation(),
        getNodeAutoInstrumentations(),
    ];

    public initialize() {
        diag.setLogger(new DiagConsoleLogger(), DiagLogLevel.DEBUG);

        try {
            this.provider.addSpanProcessor(new SimpleSpanProcessor(new ConsoleSpanExporter()));
            this.provider.addSpanProcessor(new SimpleSpanProcessor(this.exporter));
            this.provider.register();

            this.sdk = new NodeSDK({
                traceExporter: this.exporter,
                instrumentations: this.instrumentations,
            });

            this.sdk.start();

            console.info('📋 tracing initialized');
        } catch (err) {
            console.error('❗ could not initialize tracing', err);
        }
    }
}

export default new Tracing();
