import toml
from datetime import datetime
from pathlib import Path
from shutil import copyfile
from shutil import copytree
from errors import CommandError
from validate import validate

read_path = Path("src")
write_path = Path(".com")
current_section_read = Path("fuck.txt")
current_section_write = Path("fuck.txt")
current_page_read = Path("fuck.txt")
current_page_write = Path("fuck.txt")

class Listing():
    def __init__(self, li, date):
        self.li = li
        self.date = date

class Edit():
    def __init__(self, command, argument, end):
        self.command = command
        self.argument = argument
        self.end = end

# CREATING HTML FROM TEMPLATE
# -----------------------------------------------

def generate(template_address:Path, lookup:dict = None):
    template = readDoc(template_address)
    edits = parseDoc(template, lookup["data"], lookup["meta"])
    doc = editDoc(template, edits)
    return doc

def readDoc(address:Path):
    doc = []
    with open(address, "r") as page:
        doc = page.readlines()
    return doc

def parseDoc(doc:list, lookup:dict = None, meta:dict = None):
    line_edits = [] #list of tuples, where element 0 is the index of the line and element 1 is the new text

    i = 0
    for i, line in enumerate(doc, i):
        results = parseLine(line) #a list of Edit objects, with properties for command, argument, and the index of the closing paren
        if len(results) > 0:
            new_line = carryOut(results, line, lookup) #every command function should return a single string, the text to overwrite the line where the command was written
            if new_line == "bad command":
                raise CommandError("Invalid command at line " + str(i+1) + " in the " + meta["template"] + " template while building " + meta["path"] + ".")
            if new_line == "bad key":
                raise CommandError("Key does not exist at line " + str(i+1) + " in the " + meta["template"] + " template while building " + meta["path"] + ".")
            else:
                line_edits.append((i, new_line))

    return line_edits

def parseLine(line:str):
    edits = []
    i = 0
    while i < len(line):
        edit = findEdit(line, i)
        if edit != 0:
            i = edit.end
            edits.append(edit)
        else:
            i = len(line)
    return edits

def findEdit(line:str, start:int):
    iPercent = line.find("%", start)
    if iPercent != -1:
        iOParen = line.index("(", iPercent)
        iCParen = line.index(")", iOParen)
        command = line[iPercent + 1:iOParen]
        argument = line[iOParen+1:iCParen]
        return Edit(command, argument, iCParen)
    else:
        return 0

def editDoc(_doc:list, edits:list):
    doc = list(_doc)
    for edit in edits:
        doc[edit[0]] = edit[1]
    return doc

# COMMANDS
# -----------------------------------------------

def carryOut(edits:list, line:str, lookup:dict):
    new_line = line
    for edit in edits:
        if edit.command == "globalInsert":
            new_line = globalInsert(edit.argument)
        elif edit.command == "keyInsert":
            new_line = keyInsert(edit.argument, new_line, lookup)
        elif edit.command == "txt":
            new_line = txt(edit.argument, new_line, lookup)
        elif edit.command == "path":
            new_line = path(new_line, lookup["path"])
        else:
            new_line = "bad command"
    return new_line

#globalInsert should have one argument, the adress of the html snippet to be inserted in relation to /src
def globalInsert(html:str):
    snippet = ""
    with open(read_path / "inserts" / html, "r") as insert_text:
        for line in insert_text:
            snippet = snippet + line
    return snippet

#keyInsert should have one argument, the key whose value is an adress of the html to be inserted in relation to /src
def keyInsert(key:str, orig:str, lookup:dict):
    snippet = ""

    if key in lookup:
        adr = lookup[key]
        adr = read_path / adr
    else:
        return "bad key"

    with open(adr, "r") as insert:
        for line in insert:
            snippet = snippet + line
    return snippet

#txt should have one argument, the key for the appropriate value in the corresponding toml file
def txt(key:str, orig:str, lookup:dict):
    if key in lookup:
        value = lookup[key]
        new_line = orig.replace("%txt(" + key + ")", value)
        return new_line
    else:
        return "bad key"

#path takes no arguments
def path(orig:str, pathname:str):
    new = orig.replace("%path()", pathname)
    return new

# BUILD
# -----------------------------------------------

def buildDirectory(table:dict, last_write:Path, to_list:bool):
    # If we make it through validate without any errors, then everything's here! No if in's necessary
    if validate(table["meta"], table["data"], to_list):
        meta = table["meta"]
        data = table["data"]
        write = last_write / meta["path"]

        # If a page has subs, we need to know if we're gonna have to keep track of them. If yes, we'll store them in listings
        is_list = meta["list_subs"] if meta["has_subs"] else False
        if is_list:
            listings = []

        if not write.exists():
            write.mkdir()

        if meta["has_subs"]:
            if is_list:
                for sub in table["subs"]:
                    listings.append(buildDirectory(sub, write, True))
            else:
                for sub in table["subs"]:
                    buildDirectory(sub, write, False)

        data["path"] = meta["path"]
        if is_list:
            with open(read_path / "inserts/currententries.html", "w") as insert:
                if len(listings) > 0:
                    insert.writelines(listings)
                else:
                    insert.write("Nothing's here yet.")

        if meta["template"] == "none":
            page = generate(read_path / data["index_dir"], table)
        else:
            page = generate(read_path / "templates" / (meta["template"] + ".html"),  table)
        writePage(write / "index.html", page)

        if to_list:
            listing = generate(read_path / "listings" / (meta["listing"] + ".html"), table)
            listing_string = ""
            for line in listing:
                listing_string = listing_string + line
            return listing_string
        else:
            return None

def writePage(address:Path, doc:list):
    with open(address, "w") as file:
        for line in doc:
            file.write(line)

# RUN            
# -----------------------------------------------
            
sitemap = 0
with open ("sitemap.toml", "r") as read:
    sitemap = toml.load(read)

buildDirectory(sitemap, Path(".com"), False)