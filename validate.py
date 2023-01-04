from errors import SiteMapError

keys = {
    "meta": ["template", "has_subs"],

    "templates": {
        "none": ["index_dir"],
        "blog": ["title", "doc_dir"],
        "unity": ["title", "intro", "width", "height", "controls"],
        "godot": ["title", "intro", "width", "height", "controls"],
        "algiers": ["title", "intro"],
        "game-notes": ["title", "doc_dir"],
        "music": ["title", "heading", "intro", "pagesJSON", "numPages"],
        "sectionhome": ["title", "heading", "intro"]
    },
    
    "listings": {
        "entry": ["summary", "tags", "date"],
        "game": ["summary", "tags", "date"],
        "hub": ["summary", "tags", "date"]
    }
}

def validate(_table:tuple) -> bool:
    # Seperate the key and the dict
    path = _table[0]
    table = _table[1]
    
    meta = table["meta"]
    data = table["data"]

    #Check that the template exists
    if not (meta["template"] in keys["templates"]):
        raise SiteMapError(meta["template"] + " is not a defined template @" + path + ".")

    #Check all the required meta keys are present
    for key in keys["meta"]:
        if not (key in meta):
            raise SiteMapError("The " + key + " key is missing @" + path + ".meta")
        
    #Check all the required data (template) keys are present
    template = meta["template"]
    for key in keys["templates"][template]:
        if not (key in data):
            raise SiteMapError("The " + key + " key is missing @" + path + ".data")

    #If this page is generating a listing. We need a valid listing template and the appropriate keys
    if "listing" in meta:
        if not ("listing" in meta):
            raise SiteMapError("No listing template is given for the direct sub of a page where list_subs is true @" + meta["path"] + ".")
        if not (meta["listing"] in keys["listings"]):
            raise SiteMapError(meta["template"] + " is not a defined listing template @" + path + ".")
        
        listing = meta["listing"]
        for key in keys["listings"][listing]:
            if not (key in data):
                raise SiteMapError("The " + key + " key is missing @" + path + ".data") 
    
    return True
