This is a simple tool for generating a presentation site using markdown.
The tool generates htmls from markdown files that are displayed in a presentation form using [RemarkJs](https://github.com/gnab/remark)

## Getting started

To make this tool work you need to create an HTML file that will be using as the base of your page, below is an example html file

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Document</title>
    <style>
      @import url(https://fonts.googleapis.com/css?family=Yanone+Kaffeesatz);
      @import url(https://fonts.googleapis.com/css?family=Droid+Serif:400,700,400italic);
      @import url(https://fonts.googleapis.com/css?family=Ubuntu+Mono:400,700,400italic);

      body {
        font-family: "Droid Serif";
      }

      h1,
      h2,
      h3 {
        font-family: "Yanone Kaffeesatz";
        font-weight: normal;
      }

      .remark-code,
      .remark-inline-code {
        font-family: "Ubuntu Mono";
      }
    </style>
  </head>

  <body>
    <textarea id="source">
{{content}}
    </textarea>
    <script src="https://remarkjs.com/downloads/remark-latest.min.js"></script>
    <script>
      var slideshow = remark.create();
    </script>
  </body>
</html>
```

Make sure that the HTML template has a `{{content}}` template string so that we should be able to replace the markdown content.

After this you can add markdown files for your slides. The markdown needs to be formatted using the [RemarkJs](https://github.com/gnab/remark/wiki/Markdown) format.

After that you can build your site by running the tool

```bash
lata --path ./example
```

This will generate static html in the `build` folder. You can host these file on the hosting platform of choice

## Building

This project is using rust, make sure you have rust configued on you system. For the instructions [here](https://www.rust-lang.org/tools/install) to setup rust

To build run

```bash
cargo build
```

To run the projuct run

```Bash
cargo run
```

To build a release build run

```Bash
cargo build --release
```

## TODO

- Add watch mode to build files on change
- Add landing page
- Build normal markdown file into page
- Add custom page support
- make the html template optional
- Add sitemap/table of contents
