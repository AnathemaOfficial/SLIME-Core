#!/usr/bin/env python3
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
import html
import time

LOG_PATH = Path("/data/repos/SLIME/enterprise/actuator/logs/events.log")
HOST = "127.0.0.1"
PORT = 8081
MAX_LINES = 50

def read_tail(path: Path, n: int) -> list[str]:
    if not path.exists():
        return []
    try:
        with path.open("rb") as f:
            f.seek(0, 2)
            size = f.tell()
            block = 4096
            data = b""
            while size > 0 and data.count(b"\n") <= n:
                step = min(block, size)
                size -= step
                f.seek(size)
                data = f.read(step) + data
        lines = data.decode("utf-8", errors="replace").splitlines()
        return lines[-n:]
    except Exception as e:
        return [f"[error] {e!r}"]

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path not in ("/", "/health"):
            self.send_response(404)
            self.end_headers()
            return

        if self.path == "/health":
            self.send_response(200)
            self.send_header("Content-Type", "text/plain; charset=utf-8")
            self.end_headers()
            self.wfile.write(b"OK\n")
            return

        lines = read_tail(LOG_PATH, MAX_LINES)
        now = time.strftime("%Y-%m-%d %H:%M:%S %Z", time.localtime())

        body = []
        body.append("<!doctype html><html><head><meta charset='utf-8'>")
        body.append("<title>SLIME Dashboard (Read-Only)</title>")
        body.append("<style>body{font-family:monospace;padding:16px} .box{border:1px solid #444;padding:12px;margin:12px 0}</style>")
        body.append("</head><body>")
        body.append("<h1>SLIME Dashboard (Read-Only)</h1>")
        body.append(f"<div class='box'>Now: {html.escape(now)}<br>Log: {html.escape(str(LOG_PATH))}<br>Exists: {LOG_PATH.exists()}</div>")
        body.append("<div class='box'><b>Last events</b><pre>")
        if not lines:
            body.append("(no events)")
        else:
            for ln in lines:
                body.append(html.escape(ln))
        body.append("</pre></div>")
        body.append("</body></html>")

        out = "\n".join(body).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(out)))
        self.end_headers()
        self.wfile.write(out)

    def log_message(self, fmt, *args):
        return  # silence

def main():
    httpd = HTTPServer((HOST, PORT), Handler)
    print(f"[dashboard] listening on http://{HOST}:{PORT}")
    httpd.serve_forever()

if __name__ == "__main__":
    main()
