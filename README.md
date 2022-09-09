<p align="center"><img width="150" src="xiexie.png" alt="xiexie logo"></p>

# `xiexie 谢谢`
Static site generator 个的静态网站生成器

## Documentation

### Body tag
```HTML
xiexie::body
```

### CSS tag
```HTML
xiexie::css
```

### Aggregation tag
```HTML
<xiexie::aggregation::aggregationName>
    xiexie::aggregation:aggregationName::fieldName
</xiexie::aggregation::aggregationName>
```

### JSON configuration schema
```JSON
{
    "purpose": "template" | "aggregator",
    "template": templateFileName,
    "fields": [
        {
            fieldFileName: ""
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

## Installation
Download xiexie from [releases](https://github.com/jmaczan/xiexie/releases)

To use it globally, copy xiexie to `/usr/bin`

## Run
```
xiexie --source path/to/source/directory --target path/to/target/directory
```

## Build
If you don't want to use a released binary version, build xiexie on your own. [Clone](https://github.com/jmaczan/xiexie.git) this repository

Run `cargo build --release` in a project root directory. You need Rust and Cargo to build it

A binary file will be available in `target/release/xiexie`

## License
Free for personal use. [Email me](mailto:jedrzejpawel@maczan.pl) for a commercial license.

© Copyright [Jędrzej Paweł Maczan](https://maczan.pl/). Made in [Poland](https://en.wikipedia.org/wiki/Poland), 2022
