from errors import SiteMapError

keys = {
    "meta": ["path", "template", "has_subs"],

    "none": ["index_dir"],
    "blog": ["title", "doc_dir"],
    "game": ["title", "intro", "width", "height"],
    "music": ["title", "heading", "intro", "youtube"],
    "sectionhome": ["title", "heading", "intro"]
}

def validate(meta, data):
    #Check that the template exists
    if not (meta["template"] in keys):
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
    for key in keys[template]:
        if not (key in data):
            raise SiteMapError("The " + key + " key is missing @" + meta["path"] + ".data")
    
    return True
