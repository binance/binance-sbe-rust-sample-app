#!/usr/bin/env python3

import argparse
import sys
from websocket import create_connection

parser = argparse.ArgumentParser(
    description='Send a request to a WebSocket server')
parser.add_argument('url', help='WebSocket URL to connect to')
args = parser.parse_args()
config = vars(args)

# Send the request
json_request = sys.stdin.read()

ws = create_connection(config['url'])
ws.send(json_request)
result =  ws.recv()
ws.close()

# Note: This outputs binary data to stdout
sys.stdout.buffer.write(result)
