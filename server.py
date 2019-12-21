#!/usr/bin/env python3
import http.server
from http.server import HTTPServer, CGIHTTPRequestHandler
import cgitb
from pathlib import PurePath
from urllib.parse import urlparse, parse_qs
import ssl
cgitb.enable()

class CGIHandler(CGIHTTPRequestHandler):
    def send_response(self, code, message=None):
        print("RESPONSE: " + str(code) + " | " + str(message))
        print("  client_address: " + str(self.client_address))
        # print("          server: " + str(self.server))
        print("close_connection: " + str(self.close_connection))
        print("     requestline: " + str(self.requestline))
        print("         command: " + str(self.command))
        print("            path: " + str(self.path))
        print(" request_version: " + str(self.request_version))
        # print("       responses: " + str(self.responses))
        super().send_response(code, message)
    def do_GET(self):
        url = urlparse(self.path)
        path = PurePath(url.path)
        if (path.parent.name == 'properties'):
            self.path = '/cgi-bin/properties?_cp=' + path.name + '&' + url.query
            super().do_GET()
        else:
            pass

CGIHandler.have_fork = False

server = HTTPServer
handler = CGIHandler
server_address = ("0.0.0.0", 443)
handler.cgi_directories = ["/cgi-bin", "/api"]

httpd = server(server_address, handler)
httpd.socket = ssl.wrap_socket(httpd.socket, server_side=True,
    certfile='/etc/letsencrypt/live/api.libseshat.tk/fullchain.pem',
    keyfile='/etc/letsencrypt/live/api.libseshat.tk/privkey.pem')
httpd.serve_forever()

