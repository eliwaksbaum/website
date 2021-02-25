from errors import SiteMapError

keys = {
    "meta": ["path", "template", "has_subs"],

    "templates": {
        "none": ["index_dir"],
        "blog": ["title", "doc_dir"],
        "game": ["title", "intro", "width", "height"],
        "music": ["title", "heading", "intro", "youtube"],
        "sectionhome": ["title", "heading", "intro"]
    },
    
    "listings": {
        "entry": ["summary"],
        "game": ["summary"]
    }
}

def validate(meta, data, to_list):
    #Check that the template exists
    if not (meta["template"] in keys["templates"]):
        raise SiteMapError(meta["template"] + " is not a defined template @" + meta["path"] + ".")

    #Check all the required meta keys are present
    for key in keys["meta"]:
        if not (key in meta):
            raise SiteMapError("The " + key + " key is missing @" + meta["path"] + ".meta")
    
    #If the table has subs, there should be a list_subs key
    if meta["has_subs"]:
        if not ("list_subs" in meta):
            raise SiteMapError("has_subs is true but no value is given for list_subs @" + meta["path"] + ".")
        
    #Check all the required data (template) keys are present
    template = meta["template"]
    for key in keys["templates"][template]:
        if not (key in data):
            raise SiteMapError("The " + key + " key is missing @" + meta["path"] + ".data")

    #If to_list, this page is generating a listing. We need a valid listing template and the appropriate keys
    if to_list:
        if not ("listing" in meta):
            raise SiteMapError("No listing template is given for the direct sub of a page where list_subs is true @" + meta["path"] + ".")
        if not (meta["listing"] in keys["listings"]):
            raise SiteMapError(meta["template"] + " is not a defined listing template @" + meta["path"] + ".")
        
        listing = meta["listing"]
        for key in keys["listings"][listing]:
            if not (key in data):
                raise SiteMapError("The " + key + " key is missing @" + meta["path"] + ".data") 
    
    return True
