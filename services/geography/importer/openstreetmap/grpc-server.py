#!/usr/bin/env python3
import grpc
from concurrent import futures
import subprocess
import logging
import import_service_pb2
import import_service_pb2_grpc

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class ImportServicer(import_service_pb2_grpc.ImportServiceServicer):
    def RunImport(self, request, context):
        logger.info(f"Received import request: {request.command}")
        
        try:
            if request.command == import_service_pb2.ImportRequest.IMPORT:
                # Run the OpenStreetMap import
                cmd = ["/code/pelias/openstreetmap/bin/import"]
                cmd.extend(request.args)
                
            elif request.command == import_service_pb2.ImportRequest.REINDEX:
                # Run reindex command
                cmd = ["/code/pelias/openstreetmap/bin/reindex"]
                cmd.extend(request.args)
            else:
                return import_service_pb2.ImportResponse(
                    stdout="",
                    stderr="Unknown command",
                    exit_code=1
                )
            
            # Execute the command
            logger.info(f"Executing: {' '.join(cmd)}")
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=3600  # 1 hour timeout
            )
            
            return import_service_pb2.ImportResponse(
                stdout=result.stdout,
                stderr=result.stderr,
                exit_code=result.returncode
            )
            
        except subprocess.TimeoutExpired:
            return import_service_pb2.ImportResponse(
                stdout="",
                stderr="Import command timed out",
                exit_code=124
            )
        except Exception as e:
            logger.error(f"Import failed: {e}")
            return import_service_pb2.ImportResponse(
                stdout="",
                stderr=str(e),
                exit_code=1
            )

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=2))
    import_service_pb2_grpc.add_ImportServiceServicer_to_server(
        ImportServicer(), server
    )
    
    listen_addr = '0.0.0.0:50051'
    server.add_insecure_port(listen_addr)
    
    logger.info(f"Starting gRPC server on {listen_addr}")
    server.start()
    server.wait_for_termination()

if __name__ == '__main__':
    serve()