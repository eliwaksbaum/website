#!/usr/bin/env python

def tagHandler(tag):
    return "these are the posts with the " + tag + " tag."

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
