from opentelemetry import trace
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter

from opentelemetry import metrics
from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import PeriodicExportingMetricReader
from opentelemetry.exporter.otlp.proto.grpc.metric_exporter import OTLPMetricExporter

from opentelemetry.instrumentation.celery import CeleryInstrumentor
from celery.signals import worker_process_init

@worker_process_init.connect()
def init_tracing(*args, **kwargs):
    resource = Resource(attributes={
        'service.name': 'translator'
    })

    trace.set_tracer_provider(TracerProvider(resource=resource))
    trace.get_tracer_provider().add_span_processor(BatchSpanProcessor(OTLPSpanExporter()))

    metrics.set_meter_provider(MeterProvider(
        resource=resource, 
        metric_readers=[PeriodicExportingMetricReader(OTLPMetricExporter())]
    ))

    CeleryInstrumentor().instrument()
