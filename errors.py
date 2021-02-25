class Error(Exception):
    def __init__(self, message):
        self.message = message

class SiteMapError(Error):
    pass

class CommandError(Error):
    pass