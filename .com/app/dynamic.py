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
<header>
    <div class="clearfix">
        <nav>
            <span class="menu" style="margin-left: 2vw; text-align: left;"><a href="/">EW</a></span>
            <a href="/games" class="menu" id="games">Game Development</a>
            <a href="/math" class="menu" id="math">Math</a>
            <a href="/music" class="menu" id="music">Music</a>
            <a href="/other" class="menu" id="other">Other Projects</a>
            <script>highlightNav(theme_preference);</script>
        </nav>
        <img id="theme-button">
        <script>findButton();</script>
    </div>
</header>        <div class="content">        
"""
bottom = """
        </div> 
    </body>
</html>
"""

def tagHandler(tag):
    out = ""

    #with open("/var/www/eli.waksbaum.com/app/projects.json", "r") as file:
    with open(".com/app/projects.json", "r") as file:
        projects = json.load(file)
    
    for proj in projects:
        if tag in proj["tags"]:
            out = out + proj["html"]
    
    return "<ul>" + out + "</ul>"

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

with open(".com/app/test.html", "w") as file:
    file.write(top)
    file.write(tagHandler("Unity"))
    file.write(bottom)