#!/usr/bin/env python3
import socket
import json


def send_request(interest_rate):

    host, port = "localhost", 9527
    data = dict(interest_rate=interest_rate)

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        sock.connect((host, port))
        sock.send(bytes(json.dumps(data), "UTF-8"))
        received = json.loads(sock.recv(1024).decode("UTF-8"))
    finally:
        sock.close()

    return float(received["result"])


INTEREST_RATE = 0.02

print(f"Requesting NPV @ {INTEREST_RATE}")
print(f"NPV is {send_request(INTEREST_RATE):.2f} @ {INTEREST_RATE}")
