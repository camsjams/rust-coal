# rust-coal 🚂
A static HTML development tool, built with Rust

## crate
https://crates.io/crates/coal

## Who is this for?
Those that want to generate a basic static HTML site, with perhaps 1-10 pages, and do not want to maintain layouts, navigation elements, and footers across multiple files.

If you need programmatic components, then it would be better to utilize a more hands-on system like [GatsbyJS](https://www.gatsbyjs.com/), [React Static](https://github.com/react-static/react-static), or a Content Management System (CMS) such as [WordPress](https://wordpress.org/).

Just like in a typical [Jamstack](https://jamstack.org/) solution, the static HTML output can be easily served on [GitHub Pages](https://pages.github.com/), [Firebase Hosting](https://firebase.google.com/docs/hosting), and a number of other places. Also see about upcoming Docker support below.

## Platforms / Technologies
* [rust](https://www.rust-lang.org/)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [Docker](https://www.docker.com/)

## Features
- Blazing Fast 🚀
- Zero config 📄
- Convention Over Configuration 💜
- Batteries Included 🔋
- Standard HTML 🌠
- HTML,CSS, JS minification 🗜️
- Asset Bundling 📦
- Local Dev Server ⚒️
- Basic Nested CSS ~SCSS :nail_care: [Using Grass](https://crates.io/crates/grass)
- Smart Reload During Local Dev ✨
- Leverages Locomotive Emoji 🚂

## Future Features
For feature requests, please see [this issue](https://github.com/camsjams/rust-coal/issues/1).

Some ideas (check issue for latest list):
- Docker build script 🐋
- Custom reusable components (Like React or Vue) ⚛️
- Uglification of JS 🧟
- Option to produce external files for CSS and JS 🎁

## Showcase
Sites using coal to generate simple static HTML websites:
* [cameronmanavian.com](https://cameronmanavian.com/)
* [Rust Lang Los Angeles](https://rustlang.la/)

## Current Version
v0.1.0 - see Cargo.toml file

# How To Use Coal

## Local Development: Serve HTML
This runs a local development server using the Rust crate actix as a server, on the desired port (defaults to `8041`).
```
USAGE:
    coal serve [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    the folder to use in Coal [default: src]
    <port>      the port to run the dev server on [default: 8041]

```

Examples
>      $ coal serve my_source
>      $ coal serve my_source 1337

**Warning**: _This is not intended to be a secure server for a production application!_

## Deploying: Build HTML
This compiles your project into static HTML, into the desired folder (defaults to `./dist`).
```
USAGE:
    coal build [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    the folder to use in Coal [default: src]
    <dest>      the folder to compiled public assets to [default: dist]
    <version>   the build to insert into html [default: current coal version + formatted date]

```

Examples
>      $ coal build my_source
>      $ coal build my_source ./dist


## Development: Folder Structure
Your HTML project should look like this
```
assets/
    [optional additional CSS/JS/image assets here]
pages/
    pages-here.html
layouts-here.html
```
See [examples](examples) for a bunch of common solutions.

### Layout
A layout file is required at the root of your project, this is used to create the basic HTML template to place your page content.

The name of the layout file becomes the name of the tag that you'd use within your page template files, ie `layout.html` is used as `<layout>`.

There are a number of special Mustache type tags that can be used in this file:
* {{version}} - the location of the version string, generated at build time via an argument (defaults to ISO-8061 string)
* {{meta}} - any <meta> tags collected from a page template
* {{title}} - A <title> tag collected from a page template (defaults to the page file's name uppercased, ie about.html becomes "About")
* {{style}} - any <style> tags collected from a page template
* {{content}} - the resolved content including any nested components
* {{script}} - any <script> tags collected from a page template
* {{page}} - a lowercase string to denote current page, useful for adding to the body class to mark active states on navigation (see [examples/mvp/layout.html](examples/mvp/layout.html))

### Pages
Pages live within your source folder under `pages/`, and should be `dashed-lowercase.html` files, which are used to create the final url of the page.

Declaring <meta>, <title>, <style>, or <script> tags in this page will pull in your content into the layout.

The discovered <style> or <script> tags will be minified (but not uglified).

Within the <style> tag, you can utilize SCSS features such as nesting.

#### 404 Page
If using a system like Firebase Hosting, the creation of a custom 404 page is ideal. To do so in coal, just add a file within pages called `404.html`. This file will also be utilized in local development.


##### Side Note:
Thanks to the robustness of modern browsers, you can load this page file directly in the browser to quickly iterate on the content. The browser will automatically fill in necessary tags like `html`, `head`, and `body` **See above to serve a compiled page using coal during local dev.**

## Getting Coal

You have two options to run and use coal: A) from a generated release, or B) build from source.

#### A) Get Coal from a release - recommended
See releases here

#### B) Create Coal from source - **Rust 2018+ stable**
>      $ git clone git@github.com:camsjams/rust-coal.git
>      $ cargo build

## Why Coal?
The name coal is just a shortened version of the word _coalesce_.

> **co·a·lesce**
> /ˌkōəˈles/
> *verb*
> _come together to form one mass or whole._