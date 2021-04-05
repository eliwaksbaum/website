This is the weird and wonderful way I make my website, eli.waksbaum.com. I don't really recommend anyone else use this, so 
the following is just for me to remember how it all works.

## sitemap.toml
This is what ssg.py reads to build the site. Every directory is on here as a seperate table. It's keys 
are where you indicate what template it uses, and where you fill in all the data that template needs. 
If a directory has sub-directories, then they go in a subtable called s. So if you string them together,
you'll get something like page1.s.page2.s.page3. Think of the .s as a slash.

## validate.py
This is where we make sure all the syntax in the sitemap file is correct. It checks to see if every 
key that's supposed to be there is actually there for each page, both the meta keys for all pages 
and the template- and listing-specific ones for the pages that use them. So if you ever add a new template 
or change what keys one of them needs, make sure to reflect those changes in those lists at the top.

## The src Folder
blog-docs - the html files that get inserted as the body of pages using the blog template.
inserts - header, footer, head. Stuff that gets inserted.
listings - the templates for the listings.
templates - the templates for the pages. (the tags template is in here too, but that's a bit of a mess)
Any pages with the none template need their own html files, and those go here too.

## The .com Folder
This is where all the generated stuff goes, and to make your life easier, it's also where we keep 
the res, app, and html folders. res is for images, unity files, javascript, downloads, etc. html for 
is just the error pages. app has a few things for testing, but the only important part is dynamic.py, 
the uwsgi server.

### dynamic.py, the Projects page, and ajax
So most of this is static, but the project page isn't. It makes an ajax request to /dyanmic/blah, where 
the blah is determined by the url. /dynamic isn't a real directory, but the nginx server has a location 
block for it, and it does a uwsgi_pass to the dyanmic.py mini-server. That then parses the blah 
in order to serve up the correct project listings.

## Template Commands
%txt(key)
    looks up the key in the page's toml table and inserts the corresponding value string

%path()
    inserts the current page's path

%globalInsert(address)
    inserts the whole text of the html document found at src/inserts/address

%keyInsert(key)
    looks up the key in the page's toml table and inserts the whole text of the html found at src/value

%txtList(key)
    looks up the key in the page's toml table and inserts every element of the list found at that value
