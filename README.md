The way [eli.waksbaum.com](https://eli.waksbaum.com) gets made.
This is mostly for future Eli when he forgets how this all works.

# SSG

The rust project in `ssg` takes heml files in the `ssg/docs` directory and converts them into html files that it sends to the `.com` directory. 
This means that the file structure of the website lives in `ssg/docs`. E.g. `ssg/docs/public/clarinet/dancing.heml` will become `.com/public/clarinet/dancing.html`.

## HEML
The static site generator generates html files from heml files (a hyper Eli markup language). It's pretty much just html with superpowers. And where h**t**ml has **T**ags, 
h**e**ml has **E**ags.

### Eags
Eags look like normal html tags but with double angle brackets and are essentially place holders for some dynamic text defined in another file.  
The list of eags that the parser will recognize has to be defined by the user, in `ssg/eags/eags.toml`. Each eag belongs in its own table like this:
```
[foo]
path = "foo.heml"
...
```
The key for the table defines the name of the eag, so here when the parser sees `<<foo>>` it knows we're talking about the eag defined in the above toml table. 
The path value should be the path to the file that holds the text of the eag, relative to `ssg/eags/`. The rest of the keys are for the eag's parameters.

#### Parameters
I like to think of the eags as functions that take certain parameters and use them to return a string.

The function signature lives in `eags.toml`:
```
[foo]
path = "foo.heml"
text_params = ["first", "last"]
```

The function body is the file at the specified path, in this case, `ssg/eags/foo.heml`:
```
<p>
  Hello! My name is {{first}} {{last}}.
</p>
```

The function call goes in another heml file that wants to use the text defined in the eag file. The call needs to pass an argument for every parameter defined in the signature. 
You pass arguments with a little toml snippet right after the opening eag:
```
<<foo>>
  first = "John"
  last = "Doe"
  
  <div>
    ...
```
The parser knows to stop reading toml when it sees a `<`, signifying the start of some html or heml content.

The resulting html will look like this:
```
<p>
  Hello! My name is John Doe.
</p>

<div>
    ...
```

There are three kinds of parameters an eag can take, and they all work a little differently.

##### `text_params`
Text parameters are the simplest. They're defined in a toml array of strings:

```
[foo]
path = "foo.heml"
text_params = ["food"]
```

You use them in the eag body with double curly braces, and they get replaced by the value of the corresponding argument when you use the  
`<<foo>>` eag. So `I like to eat {{food}}` in the body with:
```
<foo>>
  food = "cheese"
  ...
```
in the call yields `I like to eat cheese`.

##### `file_params`
File parameters are also defined in an array of strings:
```
[foo]
path = "foo.heml"
file_params = ["css"]
```
However, they get replaced not by the value of their keys in the eag call, but by the text of the file the value points to, relative to `ssg/dumps`. 
Also, instead of brackets they use @ signs. So if foo.heml has:
```
<style>
  @@css@@
<</style
```
and our eag call has `css = foo.css`, `@@css@@` will be replaced be whatever text is in `ssg/dumps/foo.css`

#### `list_params`
List parameters require some more information, so they live an array of tables:
```
[foo]
path = "foo.heml"

[[foo.list_params]]
name = "tasks"
wrapper = "<li>{{tasks}}</li>"
join = "\n"
```
When we call them, we pass an array of strings instead of a single string: `tasks = ["clean", "cook", "taxes"]`. The `name` key corresponds to the name of the parameter, 
the toml key the parser will recognize this parameter by in the eag call. `wrapper` is used to wrap each item in our list in some other text. Use double curly braces 
to tell the parser where in the string each item should be inserted. `join` says what string should go inbetween each wrapped item.

In the eag body, list parameters use double brackets. So in foo.heml:
```
<ul>
  [[tasks]]
</ul>
```
will result in an html output of:
```
<ul>
  <li>clean</li>
  <li>cook</li>
  <li>taxes</li>
</ul>
```

#### The `{{inside}}` parameter
There's one special paramter that every eag already has defined. `{{inside}}` will be replaced by any text between the opening and closing eags 
(except for the toml). So if foo.heml looks like this:
```
<h1>{{title}}</h1>
<div>
    {{inside}}
</div>
```
and is called like this:
```
<<foo>>
    title = "Moby Dick"

    <p>Call me Ishmael</p>
<</foo>>
```
the output will be:
```
<h1>Moby Dick</h1>
<div>
    <p>Call me Ishmael</p>
</div>
```

## Nesting and Chaining
Unlock the full power of eags with nesting and chaining.

### Chaining
Consider the `<<base>>` eag. It's signature looks like this:
```
[base]
path = "base.heml"
text_params = ["title", "css", "meta-desc"]
```

and it's body like this:
```
<html lang="en">
    <<head>>
        title = "{{title}}"
        meta-desc = "{{meta-desc}}"
        css = "{{css}}"
    <</head>>
...
```

We can see it makes a call to the `<<head>>` eag, but passes the values of its own arguments. Neat!

### Nesting
Check out this heml snippet:
```
<<base>>
    title = "Projects"
    meta-desc = "Projects"
    css = "css/projects.css"
    
    <div>
        <div class="card-holder" id="holder">
            <<card>>
                proj-name = "ania"
                display-name = "A Night in Algiers"
                background = "rgb(5, 5, 5)"
                color = "rgb(74, 216, 60)"

                <<play>> proj = "a-night-in-algiers" <</play>>
                <<hub>>
                    repo = "a-night-in-algiers"
                    theme = "dark"
                <</hub>>
                <<read>>
                    link = "/blog/ania-notes.html"
                    theme = "dark"
                <</read>>
            <</card>>
...
```
Here, the implicit `inisde` argument of `<<base>>` contains a call to `<<card>>`, which in turn contains calls to `<<play>>`, `<<hub>>`, 
and `<<read>>`. The possibilities are endless!

Nesting and chaining can be used to build web pages from just a few lines of heml.

## Blog
The SSG also generates blog previews and all the info rocket needs to deal with blog tags. It uses the same heml files though, so just 
make sure all the blog post heml files are in the same directory and you should be good.

# Rocket
I use rocket to serve static files from `.com/public`, as well as do some dynamic stuff with the projects page and the blog. Stuff that 
the SSG generates for the server to use should end up in `.com/assets`/. [Rocket](https://rocket.rs) has excellent documentation, so go over there to learn how it works.

# .com/public/res
Javascript, image files, blazor and unity files etc. go here to be served.