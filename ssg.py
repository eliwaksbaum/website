import toml
from datetime import datetime
from pathlib import Path
from shutil import copyfile
from shutil import copytree

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

# -----------------------------------------------------------------------------------------------------

##meh
def buildElse():
    for d in [d for d in Path(read_path / "resources").iterdir()]:
        copytree(d, write_path / d.name, dirs_exist_ok = True)

    for d in [d for d in Path(read_path / "html").iterdir() if d.is_dir()]:
        if not (write_path / d.name).exists():
            (write_path / d.name).mkdir()
        createPage(d / "index.html", write_path / d.name / "index.html")
    
    #yikes the home page
    createPage("src/html/index.html", write_path / "index.html")

#creates a section home page list entry based on the current page's data, using the listentry.html template
def generateListEntry(lookup):
    template = readDoc(read_path / "listentry.html")
    edits = parseDoc(template, lookup)
    entry = editDoc(template, edits)
    listing = ""
    for line in entry:
        listing = listing + line + "\n"
    return listing

def buildSections():
    sections = [f for f in (read_path / "sections").iterdir() if f.is_dir()] #list of Paths, the folders in the sections folder
    for s in sections:
        global current_section_read
        current_section_read = s
        global current_section_write
        current_section_write = write_path / s.name

        #Every section needs a toml file because every section has an intro and a display name
        with open(s/"data.toml") as data:
            section_toml = toml.load(data)

        #Creates the section dir if it doesn't exist yet
        if not current_section_write.exists():
            current_section_write.mkdir()

        #This is where we're gonna store the <li> elements we generate from each page
        to_list = []

        #Holds the template so we don't need to read it for each page
        template = readDoc(s / "template.html")

        pages = [f for f in s.iterdir() if f.is_dir()] #list of Paths, the folders in the specifc section folder (the articles)
        for p in pages:
            global current_page_read
            current_page_read = p
            global current_page_write
            current_page_write = current_section_write / p.name

            #Creates the page dir if it doesn't exist yet
            if not current_page_write.exists():
                current_page_write.mkdir()

            #Every page needs a toml file because every page will be listed in a section home
            with open(p/"data.toml") as data:
                page_toml = toml.load(data)
                page_toml["section"] = section_toml
                cdate = datetime.fromtimestamp((p/"data.toml").stat().st_ctime).strftime("%x")
                page_toml["date"] = cdate #a bit janky, yes, but now each page knows its creation date

            # Generates this page's listing for the section home page, puts it into a Listing object with
            # the date, and adds it to the list
            li = generateListEntry(page_toml)
            date = datetime.strptime(page_toml["date"], "%x")
            to_list.append(Listing(li, date))

            # OK so we're gonna look for an index.html. If it's there, we'll parse it with no dictionary
            # If not, we'll use the default section template
            
            if (p / "index.html").exists():
                createPage(p / "index.html", current_page_write / "index.html")
            else:
                edits = parseDoc(template, page_toml)
                page = editDoc(template, edits)
                writeDoc(current_page_write / "index.html", page)

        #Order the Listing objects in to_list by date, then create a new list of just the strings
        to_list.sort(key = lambda i: i.date, reverse = True)
        to_list = [i.li for i in to_list]

        #Creates the section home page
        with open(read_path / "currententries.html", "w") as insert:
            if len(to_list) > 0:
                insert.writelines(to_list)
            else:
                insert.write("Nothing's here yet.")

        createPage(read_path / "sectionhome.html", current_section_write / "index.html", section_toml)

# -----------------------------------------------------------------------------------------------------


# CREATING HTML FROM TEMPLATE
# -----------------------------------------------

def generate(template_address, lookup = None):
    template = readDoc(template_address)
    edits = parseDoc(template, lookup)
    doc = editDoc(template, edits)
    return doc

def readDoc(address):
    doc = []
    with open(address, "r") as page:
        doc = page.readlines()
    return doc

def parseDoc(doc, lookup = None):
    line_edits = [] #list of tuples, where element 0 is the index of the line and element 1 is the new text

    i = 0
    for i, line in enumerate(doc, i):
        results = parseLine(line) #a list of Edit objects, with properties for command, argument, and the index of the closing paren
        if len(results) > 0:
            new_line = carryOut(results, line, lookup) #every command function should return a single string, the text to overwrite the line where the command was written
            if new_line == "bad commad":
                print("Invalid command at line " + str(i+1) + " in " + current_section_read.name + "; " + current_page_read.name + ".")
            if new_line == "bad key":
                print("Key does not exist at line " + str(i+1) + " in " + current_section_read.name + "; " + current_page_read.name + ".")
            else:
                line_edits.append((i, new_line))

    return line_edits

def parseLine(line):
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

def findEdit(line, start):
    iPercent = line.find("%", start)
    if iPercent != -1:
        iOParen = line.index("(", iPercent)
        iCParen = line.index(")", iOParen)
        command = line[iPercent + 1:iOParen]
        argument = line[iOParen+1:iCParen]
        return Edit(command, argument, iCParen)
    else:
        return 0

def editDoc(_doc, edits):
    doc = list(_doc)
    for edit in edits:
        doc[edit[0]] = edit[1]
    return doc

# COMMANDS
# -----------------------------------------------

def carryOut(edits, line, lookup):
    new_line = line
    for edit in edits:
        if edit.command == "globalInsert":
            new_line = globalInsert(edit.argument)
        # elif edit.command == "localInsert":
        #     new_line = localInsert(edit.argument)
        elif edit.command == "keyInsert":
            new_line = keyInsert(edit.argument, new_line, lookup)
        elif edit.command == "txt":
            new_line = txt(edit.argument, new_line, lookup)
        # elif edit.command == "sectiontxt":
        #     new_line = sectiontxt(edit.argument, new_line, lookup["section"])
        elif edit.command == "path":
            new_line = path(new_line, lookup["path"])
        # elif edit.command == "fetchFile":
        #     fetch(edit.argument, "file")
        # elif edit.command == "fetchFolder":
        #     fetch(edit.argument, "folder")
        else:
            new_line = "bad command"
    return new_line

#globalInsert should have one argument, the adress of the html snippet to be inserted in relation to /src
def globalInsert(html):
    snippet = ""
    with open(read_path / "inserts" / html, "r") as insert_text:
        for line in insert_text:
            snippet = snippet + line
    return snippet

#keyInsert should have one argument, the key whose value is an adress of the html to be inserted in relation to /src
def keyInsert(key, orig, lookup):
    adr = ""
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

#localInsert should have one argument, the adress of the html snippet to be inserted in relation to the specific article folder

# def localInsert(html):
#     snippet = ""
#     with open(current_page_read / html, "r") as insert_text:
#         for line in insert_text:
#             snippet = snippet + line
#     return (snippet)

#txt should have one argument, the key for the appropriate value in the corresponding toml file
def txt(key, orig, lookup):
    if key in lookup:
        value = lookup[key]
        new_line = orig.replace("%txt(" + key + ")", value)
        return new_line
    else:
        return "bad key"

# def sectiontxt(key, orig, lookup):
#     if key in lookup:
#         value = lookup[key]
#         new_line = orig.replace("%sectiontxt(" + key + ")", value)
#         return new_line
#     else:
#         return "bad key"

def path(orig, pathname):
    new = orig.replace("%path()", pathname)
    return new

# the fetches should have one or two arguments. the first is the path to the folder or file you want to
# include in the dst (starting from the current page's src directory). the second is the destination folder,
# which if not included will default to the page's folder.

# def fetch(argument, mode):
#     arguments = argument.split("|")
#     read = current_page_read / arguments[0]
#     if len(arguments) == 1:
#         write = current_page_write / arguments[0]
#     else:
#         write = arguments[1] / arguments[0]
#     if mode == "file":
#         copyfile(read, write)
#     elif mode == "folder":
#         copytree(read, write, dirs_exist_ok = True)

# BUILD
# -----------------------------------------------

def buildDirectory(table:dict, last_write:Path):
    write = last_write / table["meta"]["path"]

    if not write.exists():
        write.mkdir()

    if table["meta"]["has_subs"]:
        for sub in table["subs"]:
            buildDirectory(sub, write)
    
    table["data"]["path"] = table["meta"]["path"]

    if table["meta"]["template"] == "none":
        page = generate(read_path / table["data"]["index_dir"])
    else:
        page = generate(read_path / "templates" / (table["meta"]["template"] + ".html"),  table["data"])
    writePage(write / "index.html", page)

def writePage(address, doc):
    with open(address, "w") as file:
        for line in doc:
            file.write(line)

# RUN            
# -----------------------------------------------
            
sitemap = 0
with open ("sitemap.toml", "r") as read:
    sitemap = toml.load(read)

buildDirectory(sitemap, Path(".com"))