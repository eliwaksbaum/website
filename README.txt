THE SRC FOLDER
----------------------------------------------------
resources - stylesheets, images, downloads, etc
html - every page that doesn't belong in a section. this includes the home page. every other page needs to go
        in its own folder.
sections - every article that goes into a section
currententries.html - a document that is used in writing the section home pages. will be generated each time
listentry.html - the format for an article listing on a section home pages
sectionhome.html - the template for the section home pages
head.html, header.html, footer.html - documents that will be inserted into every pages

THE SECTION FOLDER
--------------------------------------------------------
Every section has a folder. Within every section folder, there needs be a data.toml file, a template.html
file, and a folder for every page. Within every page folder there must be a data.toml and preview.jpg.
Every data.toml will be parsed and used to fill in the default template for the page's section. If you
don't want to use the template, include an index.html file with its custom layout.

THE DATA.TOML FILE (section)
--------------------------------------------------------
Every file needs the following three keys:
    title
    heading
    intro

THE DATA.TOML FILE (page)
--------------------------------------------------------
Every file needs the following two keys:
    intro - the paragraph that introduces the article
    summary - the paragraph that is displayed on the section home page
The rest can vary from section to section, just make sure they match all the %txt() calls in their template

THE ELSE FOLDER
----------------------------------------------------------
The stylesheet folder is for the stylesheets. The html folder holds all the pages that don't belong in a
section. The index.html is the homepage, all other pages need to go in their own folders and be named index.html.

COMMANDS
--------------------------------------------------------
%txt(key)
    looks up the key in the given toml file and inserts the corresponding value string

    -- there are two secret keys that are generated for each data.toml file that belongs in a section. section and date --

%path()
    inserts the current page's path

%localInsert(address)
    inserts the whole text of the html document found at (current page's src directory)/address

%globalInsert(address)
    inserts the whole text of the html document found at src/address

%fetchFile(address|destination(optional))
    copies the file found at (current page's src directory)/address into the destination directory. If no
    directory is included the default destination is the page's base destination directory.

%fetchFolder(address|destination(optional))
    copies the folder found at (current page's src directory)/address into the destination directory. If no
    directory is included the default destination is the page's base destination directory.

(It is recommended to put the %fetch calls in a comment at the top of the template)