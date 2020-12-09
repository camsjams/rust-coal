# rust-coal 🚂
A static HTML development tool, built with Rust

## crate
https://crates.io/crates/coal

## Who is this for?
Those that want to generate a basic static HTML site, with perhaps 1-5 pages, and do not want to maintain layouts, navigation elements, and footers across multiple files.

If you need programmatic components, then it would be better to utilize a more hands on system like GatsbyJS, Nozzle, or a Content Management System (CMS) such as WordPress.

Just like in a typical JAMStack solution, the static HTML output can be easily served on GitHub pages, Firebase Hosting, and a number of other places. Also see about Docker support below.

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
For feature requests, please create an issue.

Some ideas:
- Docker build script 🐋
- Custom reusable components (Like React or Vue) ⚛️

## Showcase
Sites using coal to generate simple static HTML websites:
* [cameronmanavian.com](https://cameronmanavian.com/)
* [Rust Lang Los Angeles](https://rustlang.la/)

## Current Version
v0.1.0 - see Cargo.toml file

# How To Use Coal

## Local Development: Serve HTML
This runs a local development server using the Rust crate actix as a server, on the desired port (defaults to `8041`).
>      $ coal serve {source_folder} [{port}=.8041]

Examples
>      $ coal serve my_source
>      $ coal serve my_source 1337

**Warning**: _This is not intended to be a secure server for a production application!_

## Deploying: Build HTML
This compiles your project into static HTML, into the desired folder (defaults to `./dist`).
>      $ coal build {source_folder} [{destination_folder}=./dist]

Examples
>      $ coal build my_source
>      $ coal build my_source ./dist

## Deploying: Containerize HTML (Coming Soon)
This compiles your project into static HTML, into a Docker image within the desired folder **on the image**.
>      $ coal image {source_folder} {dockerfile_location} {destination_folder}

Example
>      $ coal image my_source ./Dockerfile /var/httpdocs

**Note**: _Docker must be installed on build machine to run the `docker create` command_

## Development: Folder Structure
Your HTML project should look like this
```
components/
    components-here.html
pages/
    pages-here.html
layout-here.html
```
See [examples](examples) for a bunch of common solutions.

### Layout
A layout file is required at the root of your project, this is used to create the basic HTML template to place your page content.

The name of the layout file becomes the name of the tag that you'd use within your page template files, ie `layout.html` is used as `<layout>`.

There are a number of special tags that can be used in this file:
* {{version}} - the location of the version string, generated at build time via an argument (defaults to ISO-8061 string)
* {{meta}} - any <meta> tags collected from a page template
* {{title}} - A <title> tag collected from a page template (defaults to the page file's name uppercased, ie about.html becomes "About")
* {{style}} - any <style> tags collected from a page template
* {{content}} - the resolved content including any nested components
* {{script}} - any <script> tags collected from a page template

### Pages
Pages live within your source folder under `pages/`, and should be `dashed-lowercase.html` files, which are used to create the final url of the page.

Declaring <meta>, <title>, <style>, or <script> tags in this page will pull in your content into the layout.

Thanks to the robustness of modern browsers, you can load this page file directly in the browser to quickly iterate on the content. The browser will automatically fill in necessary tags like `html`, `head`, and `body` **See above to serve a compiled page using coal during local dev.**

## Getting Coal

You have two options to run and use coal: A) from a generated release, or B) build from source.

#### A) Get Coal from a release - recommended
See releases here

#### B) Create Coal from source - **Rust 2018+ stable**
>      $ git clone git@github.com:camsjams/rust-coal.git
>      $ cargo build

## Why Coal?
The name coal is just a shortened version of the word coalesce.

> **co·a·lesce**
> /ˌkōəˈles/
> *verb*
> _come together to form one mass or whole._