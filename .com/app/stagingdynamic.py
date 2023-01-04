#!/usr/bin/env python

import json, ssl, smtplib

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

GET_handlers = {
    "tag": tagHandler,
    "print": printHandler,
    "all-projects": allHandler
}

def mailHandler(body):
    with open("/home/eli/password.secret", "r") as secret:
        password = secret.read()
    password = password[:-1]
    smtp_server = "smtp.migadu.com"
    port = 465  # For starttls
    sender_email = "eli@waksbaum.com"
    receiver_email = "eli@waksbaum.com"
    message = "Subject: " + body["subject"] + "\n\n" + body["message"] + "\n\n" + "Sent from : " + body["email"]

    # Create a secure SSL context
    context = ssl.create_default_context()
    with smtplib.SMTP_SSL(smtp_server, port, context=context) as server:
        server.login(sender_email, password)
        server.sendmail(sender_email, receiver_email, message)
        server.quit()

    return message

POST_handlers = {
    "mail": mailHandler
}

def application(env, start_response):
    start_response('200 OK', [('Content-Type','text/html')])

    path = env["PATH_INFO"].replace("/dynamic/", "", 1)
    pieces = path.split("/")
    method = env["REQUEST_METHOD"].upper()

    if (method == "GET"):
        if pieces[0] in GET_handlers:
            handler = GET_handlers[pieces[0]]
            out = handler(pieces[1])
        else:
            out = pieces[0] + " doesn't have a GET handler"
            
        return [out.encode()]

    elif (method == "POST"):
        dic = json.load(env['wsgi.input'])

        if pieces[0] in POST_handlers:
            handler = POST_handlers[pieces[0]]
            out = handler(dic)
        else:
            out = pieces[0] + " doesn't have a POST handler"

        return [out.encode()]