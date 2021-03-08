#!/usr/bin/env python
# test.py
def application(env, start_response):
    start_response('200 OK', [('Content-Type','text/html')])
    return [str(env).encode()] # python3
