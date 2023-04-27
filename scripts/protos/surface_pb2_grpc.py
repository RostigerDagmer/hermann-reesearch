# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import protos.surface_pb2 as surface__pb2


class PaperSearchStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetPaper = channel.unary_unary(
                '/surface.PaperSearch/GetPaper',
                request_serializer=surface__pb2.GetPaperRequest.SerializeToString,
                response_deserializer=surface__pb2.GetPaperResponse.FromString,
                )
        self.InsertPaper = channel.unary_unary(
                '/surface.PaperSearch/InsertPaper',
                request_serializer=surface__pb2.InsertPaperRequest.SerializeToString,
                response_deserializer=surface__pb2.InsertPaperResponse.FromString,
                )
        self.SearchPaper = channel.unary_unary(
                '/surface.PaperSearch/SearchPaper',
                request_serializer=surface__pb2.SearchPaperRequest.SerializeToString,
                response_deserializer=surface__pb2.SearchPaperResponse.FromString,
                )


class PaperSearchServicer(object):
    """Missing associated documentation comment in .proto file."""

    def GetPaper(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def InsertPaper(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SearchPaper(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_PaperSearchServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetPaper': grpc.unary_unary_rpc_method_handler(
                    servicer.GetPaper,
                    request_deserializer=surface__pb2.GetPaperRequest.FromString,
                    response_serializer=surface__pb2.GetPaperResponse.SerializeToString,
            ),
            'InsertPaper': grpc.unary_unary_rpc_method_handler(
                    servicer.InsertPaper,
                    request_deserializer=surface__pb2.InsertPaperRequest.FromString,
                    response_serializer=surface__pb2.InsertPaperResponse.SerializeToString,
            ),
            'SearchPaper': grpc.unary_unary_rpc_method_handler(
                    servicer.SearchPaper,
                    request_deserializer=surface__pb2.SearchPaperRequest.FromString,
                    response_serializer=surface__pb2.SearchPaperResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'surface.PaperSearch', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class PaperSearch(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def GetPaper(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/surface.PaperSearch/GetPaper',
            surface__pb2.GetPaperRequest.SerializeToString,
            surface__pb2.GetPaperResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def InsertPaper(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/surface.PaperSearch/InsertPaper',
            surface__pb2.InsertPaperRequest.SerializeToString,
            surface__pb2.InsertPaperResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SearchPaper(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/surface.PaperSearch/SearchPaper',
            surface__pb2.SearchPaperRequest.SerializeToString,
            surface__pb2.SearchPaperResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
