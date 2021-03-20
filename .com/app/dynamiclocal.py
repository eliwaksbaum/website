#!/usr/bin/env python

import json

top = """
<!DOCTYPE html>
<html lang="en">
    <head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<script src="/res/js/theme.js"></script>
<link id="base" rel="stylesheet" href="/res/stylesheets/base.css">
<link id="theme" rel="stylesheet">
<script>findCss();</script>        <title>Coming Soon</title>
    </head>
    <body>
<header id="header">
    <div class="clearfix">
        <nav>
            <div></div>
            <a class="menu" href="/">EW</a>
            <div>
                <a href="/projects" class="menu">Projects</a>
            </div>
            <div>
                <img id="theme-button">
            </div>
        </nav>
        <script>findButton();</script>
        <script>colorHeader(theme_preference);</script>
    </div>
</header>        <div class="content">      
"""
bottom = """
        </div> 
    </body>
</html>
"""

def getProjs(tag = ""):
    out = ""

    with open(".com/app/projects.json", "r") as file:
        projects = json.load(file)
    
    for proj in projects:
        if tag != "":
            if tag in proj["tags"]:
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

def application(env, start_response=None):
    #start_response('200 OK', [('Content-Type','text/html')])

    path = env["PATH_INFO"].replace("/dynamic/", "", 1)
    pieces = path.split("/")
    if pieces[0] in handlers:
        handler = handlers[pieces[0]]
        out = handler(pieces[1])
    else:
        out = pieces[0] + " doesn't have a handler"
        
    #return [out.encode()]
    return out

with open(".com/app/test.html", "w") as file:
    file.write(top)
    file.write("<ul>")
    file.write(application({"PATH_INFO": "/dynamic/all-projects/"}))
    file.write("</ul>")
    file.write(bottom)