import sys

from .longport import openapi
from typing import Optional

sys.modules['longport.openapi'] = openapi


class OpenApiException(Exception):
    def __init__(self, kind, code, trace_id, message):
        self.kind = kind
        self.code = code
        self.trace_id = trace_id
        self.message = message

    def __str__(self):
        if self.code != None:
            return "OpenApiException: (kind=%s, code=%d, trace_id=%s) %s" % (self.kind, self.code, self.trace_id, self.message)
        else:
            return "OpenApiException: %s" % self.message


openapi.OpenApiException = OpenApiException
