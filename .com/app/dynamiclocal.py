#!/usr/bin/env python

import json

top = """
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<script src="/res/js/theme.js"></script>
<script src="/res/js/mobile.js"></script>
<link id="base" rel="stylesheet" href="/res/stylesheets/base.css">
<link id="theme" rel="stylesheet">
<script>findCss();</script>
    </head>
    <body>
        <header id="header" class="desktop colorful">
    <div class="clearfix">
        <nav class="desktop">
            <a class="menu header-left" href="/">EW</a>
            <span>
                <a href="/projects" class="menu menu-span">Projects</a>
                <a class="menu menu-span">Say Hi</a>
            </span>
            <button onClick="toggleTheme()" class="chill desktop-right">
                <img id="theme-button-desktop">
            </button>
        </nav>
        <script>findButton();</script>
    </div>
</header>
<header class="mobile colorful">
    <nav class="mobile">
        <a class="menu header-left" href="/">EW</a>
        <div class="dropcont">
            <button onClick="menuDown()" class="chill">
                <img class="mobile-right themed" data-themedSrc="/res/svg/hamburger_theme.svg">
            </button>
            <div id="dropdown" class="dropdown">
                <a class="colorful menu" href="/projects">Projects</a>
                <a class="colorful menu">Say Hi</a>
            </div>
        </div>
    </nav>
</header>
<script>colorize();</script>
    </body>
</html>    
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