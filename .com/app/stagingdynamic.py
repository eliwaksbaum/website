#!/usr/bin/env python

import json

def getProjs(tag = ""):
    out = ""

    with open("/var/www/staging/app/projects.json", "r") as file:
        projects = json.load(file)
    
    tag = tag.lower()

    for proj in projects:
        if tag != "":
            tags = [x.lower() for x in proj["tags"]]
            if tag in tags:
                out = out + proj["html"]
        else:
            out = out + proj["html"]
    
    return out

def tagHandler(tag):
    return getProjs(tag)

def allHandler(blank):
    return getProjs()

def printHandler(txt):
    return txt

handlers = {
    "tag": tagHandler,
    "print": printHandler,
    "all-projects": allHandler
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