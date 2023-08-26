# Thu Aug 24

what is a wiki body?

basically just:
{
    "content": "markdown"
}

oh well, we can have things like author, date, etc etc parsed out from the yaml front-matter I think.

hmmmm I think author, date, etc can be parsed out from the yaml front-matter and placed in the json on the client-side. That way we have easy access to the keys forevermore, and the yaml to json happens before the data ever hits the server.

we need to publish a set of keys that are used, which so far is just "content", and "format".

do non-"format" keys just get plugged into the template?

That seems relatively straightforward.

How do we handle CSS? I think it should be inline - that means every page needs to have CSS, but that's honestly not a major problem I think.

How does deploying tailwind work? We can worry about that down the line, it's not the only option we have for CSS.

## what defines a wiki page and distinguishes it from others?

well, the title seems like a straightforward one (and is how other wikis handled this?)

but updating a page's title... ohhh

we can handle that in the clientside, where the client says "oh, we began editing this page, and we are now attempting to change the title. We should delete the old page and create a new page."

does that screw with editing history? eh I don't care about editing history. Besides, the end goal of this is to have a github repo + desktop client that manages the repo. then history will be stored there and we can roll back as needed.

# prior thoughts

wiki web server
    - consider more markup formats and how to extend the site to handle that.
        - log in to the server and set up a file to configure the available markup conversions and the scripts that run to actually perform a conversion.
    - a web editor?
- feeling _pretty_ wedded to the custom markup. that could be a pre-processor though? using a special tag type (perhaps using a special sexp with an & in front. ex: `#(durable-link google.com)`
        - # can be escaped with \ ofc.
        - pre-processor sounds pretty darn perfect.
        - wonder, are there any major features that will be impossible using a pre-processor?
            - I don't _think_ so. Oh, I guess there are things like backlinks, which require building the set of links in advance. I think it'd be possible to hack it together.
            - mmmm backlinks would require evaluating the entire set of page sources. Oh, I guess because this would be running on the server-side, then it'd be fine, because the serverside has the capability to view the entire site source.
        - This modular approach is hackable, which I really like. 
        - The conversion for each markup should be a simple executable script on the serverside (also perhaps modifiable using the API?)


    page_sources
    pages // converted from page_sources using the script that matches the file extension

```
GET /page_source/<id>/
GET /page/<id>/
GET /page/<id>/edit/
POST /new_page/ // creates a page_source object that gets converted to a page.
DELETE page_source // goes and cleans up page as well.

page_source and page_id are the same?
```

also, how does searching work? nah let's not worry about searching, we can build a search index later from the FS if needed.

hmmmm if we want this to be multithreaded then rust would be the ideal solution.

if we want this to be a quick project that we finish and can present on, then it's definitely not the right solution. But... would be a good way to learn lots of multithreaded code stuff, and could maybe be ok? I'm really not sure honestly.

I think I'm going to do it, and we'll see where we get out in the end.

mvp:

1. accept a markdown page POSTed to to `/new_page/`
2. assign it an id, save it in the filesystem (under /opt/) as the id under `page_source`
    - we're doing this now using hashes, so that the hash is based on the content of the page...
        - hmmm that means that old versions of the page will exist in the database... that's not bad per se, but we need some way to handle page updates nonetheless.
3. convert it to html, save it in `page` as html
4. serve either the page or the page source when requested at the appropriate url.
