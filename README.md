<p align="center"><img width="150" src="xiexie.png" alt="xiexie logo"></p>

# `xiexie 谢谢`
Static site generator 个的静态网站生成器

## Quick start
If you like learning by example, you can check out a sample [xiexie blog template](https://github.com/jmaczan/xiexie-blog-template) and play with it to get an intuition. Just remember that values are rendered by prefixing with `xiexie::` and each html needs a JSON config file, most of them are more or less copy-paste of another.

## Install
Download xiexie from [releases](https://github.com/jmaczan/xiexie/releases)

To use it globally, copy xiexie to `/usr/bin`

## Run
```
xiexie --source path/to/source/directory --target path/to/target/directory
```

## Documentation

For each page named i.e. `about`, you create 3 files: 
- HTML with content and `xiexie::`-prefixed tags (`about.html`)
- CSS with page specific styles (`about.css`)
- JSON (`about.json`)

JSON file needs a specific schema, which is defined below.

### Body tag
This is where content of a `template` is rendered.
```HTML
xiexie::body
```

### CSS tag
It is replaced with a `<link>` pointing to a page's CSS file.
```HTML
xiexie::css
```

### Aggregation tag
Use aggregations when you want to make copies of some HTML structure multiple times. For example, when you want to render a collection of posts on a blog.
```HTML
<xiexie::aggregation::aggregationName>
    xiexie::aggregation:aggregationName::fieldName
</xiexie::aggregation::aggregationName>
```

### JSON configuration schema
This is how your JSON file is structured.

A file has a `purpose`. If it's a template for other pages, set it to `template`. If it renders a collection of pages, set it to `aggregator`. If it's a page (like `about.html`), set it to `"page"`.

If a file is a regular page, it needs a `template`. Put a name of a template as a value, i.e. `"blog-post"`.

```py
{
    "purpose": "template" | "aggregator" | "page",
    "template": templateFileName,
    "fields": [
        {
            fieldFileName: "value of field name"
        },
        {
            anotherFieldFileName: ""
        }
    ],
    "aggregations": [
        {
            aggregationName: [
                {
                    fieldFileName: ""
                },
                {
                    anotherFieldFileName: ""
                }
            ]
        }
    ]
}
```

## Build
If you don't want to use a released binary version, build xiexie on your own. [Clone](https://github.com/jmaczan/xiexie.git) this repository

Run `cargo build --release` in a project root directory. You need Rust and Cargo to build it

A binary file will be available in `target/release/xiexie`

## License
Free for personal use. [Email me](mailto:jedrzejpawel@maczan.pl) for a commercial license.

© Copyright [Jędrzej Paweł Maczan](https://maczan.pl/). Made in [Poland](https://en.wikipedia.org/wiki/Poland), 2022
