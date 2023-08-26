# dg: a digital garden manager

`dg` is a digital garden tool with a narrow scope:

1. accept CRUD requests for wiki pages in markup format
2. convert those files to HTML (if needed) using a server-defined mapping of format to renderer (including templating.)
3. serve those files, along with a server-generated

`dg` respects the XDG spec, and as a result stores all it's data in a folder in `XDG_DATA_HOME`. This is also where it expects to find its configuration.

The roadmap includes:

- built-in support for backlinks
- polling of a configured git repo, complete with automatic re-rendering of changed pages.
- An in-browser edit page
    - supports the same user-defined templating that pages have
    - supports previewing of pages.
- (potential) multi-user support

# installation

tbd.
