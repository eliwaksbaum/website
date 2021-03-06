import toml
import json
from datetime import datetime
from pathlib import Path
from errors import CommandError
from validate import validate

read_path = Path("src")
write_path = Path(".com")
projects = []


# CREATING HTML FROM TEMPLATE
# -----------------------------------------------

def generate(template_address:Path, lookup:dict = None) -> list:
    template = readDoc(template_address)
    edits = parseDoc(template, lookup["data"], lookup["meta"])
    doc = editDoc(template, edits)
    return doc

def readDoc(address:Path) -> list:
    with open(address, "r", encoding="utf-8") as page:
        doc = page.readlines()
    return doc

def parseDoc(doc:list, lookup:dict = None, meta:dict = None) -> list:
    line_edits = [] #list of tuples, where element 0 is the index of the line and element 1 is the new text

    i = 0
    for i, line in enumerate(doc, i):
        results = parseLine(line) #a list of Edit dicts, with keys for command, argument, and the index of the closing paren
        if len(results) > 0:
            new_line = carryOut(results, line, lookup) #every command function should return a single string, the text to overwrite the line where the command was written
            if new_line == "bad command":
                raise CommandError("Invalid command at line " + str(i+1) + " in the " + meta["template"] + " template while building " + meta["path"] + ".")
            if new_line == "bad key":
                raise CommandError("Key does not exist at line " + str(i+1) + " in the " + meta["template"] + " template while building " + meta["path"] + ".")
            else:
                line_edits.append((i, new_line))

    return line_edits

def parseLine(line:str) -> list:
    edits = []
    i = 0
    while i < len(line):
        edit = findEdit(line, i)
        if edit != {}:
            i = edit["end"]
            edits.append(edit)
        else:
            i = len(line)
    return edits

def findEdit(line:str, start:int) -> dict:
    iPercent = line.find("%", start)
    if iPercent != -1:
        iOParen = line.index("(", iPercent)
        iCParen = line.index(")", iOParen)
        command = line[iPercent + 1:iOParen]
        argument = line[iOParen+1:iCParen]
        return {"command": command, "argument": argument, "end": iCParen}
    else:
        return {}

def editDoc(_doc:list, edits:list) -> list:
    doc = list(_doc)
    for edit in edits:
        doc[edit[0]] = edit[1]
    return doc

# COMMANDS
# -----------------------------------------------

def carryOut(edits:list, line:str, lookup:dict) -> str:
    new_line = line
    for edit in edits:
        if edit["command"] == "globalInsert":
            new_line = globalInsert(edit["argument"])
        elif edit["command"] == "keyInsert":
            new_line = keyInsert(edit["argument"], new_line, lookup)
        elif edit["command"] == "txt":
            new_line = txt(edit["argument"], new_line, lookup)
        elif edit["command"] == "path":
            new_line = path(new_line, lookup["path"])
        elif edit["command"] == "txtList":
            new_line = txtList(edit["argument"], new_line, lookup)
        else:
            new_line = "bad command"
    return new_line

#globalInsert should have one argument, the adress of the html snippet to be inserted in relation to /src
def globalInsert(html:str) -> str:
    snippet = ""
    with open(read_path / "inserts" / html, "r", encoding="utf-8") as insert_text:
        for line in insert_text:
            snippet = snippet + line
    return snippet

#keyInsert should have one argument, the key whose value is an adress of the html to be inserted in relation to /src
def keyInsert(key:str, orig:str, lookup:dict) -> str:
    snippet = ""

    if key in lookup:
        adr = lookup[key]
        adr = read_path / adr
    else:
        return "bad key"

    with open(adr, "r", encoding="utf-8") as insert:
        for line in insert:
            snippet = snippet + line
    return snippet

#txt should have one argument, the key for the appropriate value in the corresponding toml table
def txt(key:str, orig:str, lookup:dict) -> str:
    if key in lookup:
        value = lookup[key]
        new_line = orig.replace("%txt(" + key + ")", value)
        return new_line
    else:
        return "bad key"

#path takes no arguments
def path(orig:str, pathname:str) -> str:
    new = orig.replace("%path()", pathname)
    return new

#txtList should have one argument, the key whose value is a list
def txtList(key:str, orig:str, lookup:dict) -> str:
    insert = ""
    with open(read_path / ("templates/" + key + ".html"), "r", encoding="utf-8") as file:
        template = file.readlines()

    for i in lookup[key]:
        out = ""
        for line in template:
            out = out + line.format(i)
        insert = insert + out
    
    new = orig.replace("%txtList(" + key + ")", insert)
    return new

# BUILD
# -----------------------------------------------

def buildDirectory(_table:tuple, last_write:Path) -> None:
    # If we make it through validate without any errors, then everything's here! No if in's necessary
    if validate(_table):
        # Seperate the key and the dict
        key = _table[0]
        table = _table[1]
        
        meta = table["meta"]
        data = table["data"]
        write = last_write / key

        if not write.exists():
            write.mkdir()

        if meta["has_subs"]:
            for sub in table["s"].items():
                buildDirectory(sub, write)

        data["path"] = key

        if meta["template"] == "none":
            page = generate(read_path / data["index_dir"], table)
        else:
            page = generate(read_path / "templates" / (meta["template"] + ".html"),  table)
        writePage(write / "index.html", page)

        # If it has a listing, generate it and store it in an object with its keys in app/projects.json
        if "listing" in meta:
            addListing(meta["listing"], data["tags"], data["date"], table)

def writePage(address:Path, doc:list) -> None:
    with open(address, "w", encoding="utf-8") as file:
        for line in doc:
            file.write(line)

def addListing(listing:str, tags:list, date:str, table:dict) -> None:
    listing = generate((read_path / "listings" / (listing + ".html")), table)
            
    listing_string = ""
    for line in listing:
        listing_string = listing_string + line

    listing_obj = {"html": listing_string, "tags": tags, "date": date}
    projects.append(listing_obj)


# RUN            
# -----------------------------------------------
            
sitemap:list
with open ("sitemap.toml", "r", encoding="utf-8") as read:
    sitemap = toml.load(read)

buildDirectory(("", sitemap), Path(".com"))

#ahhhhhhhhhhh
page = generate(read_path / "404.html", {"data":{}, "meta":{}})
writePage(Path(".com/html/404.html"), page)
page = generate(read_path/"403.html", {"data":{}, "meta":{}})
writePage(Path(".com/html/403.html"), page)
#ahhhhhhhhhhhh

projects = sorted(projects, key = lambda x: datetime.strptime(x["date"], "%m/%d/%Y"), reverse=True)

with open (write_path/"app/projects.json", "w", encoding="utf-8") as projects_file:
    json.dump(projects, projects_file)