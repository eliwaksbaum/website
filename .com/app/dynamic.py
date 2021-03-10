#!/usr/bin/env python

import json

def tagHandler(tag):
    out = ""

    with open("/var/www/eli.waksbaum.com/app/projects.json", "r") as file:
        projects = json.load(file)
    
    for proj in projects:
        if tag in proj["tags"]:
            out = out + proj["html"]
    
    return out

def printHandler(txt):
    return txt

handlers = {
    "tag": tagHandler,
    "print": printHandler
}

def application(env, start_response):
    start_response('200 OK', [('Content-Type','text/html')])

    path = env["PATH_INFO"].replace("/dynamic/", "", 1)
    pieces = path.split("/")
    if pieces[0] in handlers:
        handler = handlers[pieces[0]]
        out = handler(pieces[1])
    else:
        out = pieces[0] + " doesn't have a handler"
        
    return [out.encode()]
