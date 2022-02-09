import socketserver, sys
import json
import npv

HOST = "localhost"
PORT = 9527


class SingleTCPHandler(socketserver.BaseRequestHandler):
    def handle(self):
        data = self.request.recv(1024)  # clip input at 1Kb
        text = data.decode("utf-8")
        data = json.loads(text)
        result = npv.main(data["interest_rate"])
        self.request.send(bytes(json.dumps({"result": str(result)}), "UTF-8"))
        self.request.close()


class SimpleServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    daemon_threads = True
    allow_reuse_address = True

    def __init__(self, server_address, RequestHandlerClass):
        socketserver.TCPServer.__init__(self, server_address, RequestHandlerClass)


if __name__ == "__main__":
    server = SimpleServer((HOST, PORT), SingleTCPHandler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        sys.exit(0)
