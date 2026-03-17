# peasytext

[![crates.io](https://img.shields.io/crates/v/peasytext)](https://crates.io/crates/peasytext)
[![docs.rs](https://docs.rs/peasytext/badge.svg)](https://docs.rs/peasytext)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://agentgif.com/badge/crates/peasytext/version.svg)](https://crates.io/crates/peasytext)
[![GitHub stars](https://agentgif.com/badge/github/peasytools/peasytext-rs/stars.svg)](https://github.com/peasytools/peasytext-rs)

Async Rust client for the [PeasyText](https://peasytext.com) API -- text case conversion, slug generation, word counting, and encoding utilities with tools for camelCase, snake_case, kebab-case, PascalCase, and more. Built with reqwest, serde, and tokio.

Built from [PeasyText](https://peasytext.com), a comprehensive text processing toolkit offering free online tools for case conversion, slug generation, word counting, and text analysis. The glossary covers text concepts from character encodings to Unicode normalization, while guides explain text processing strategies and encoding best practices.

> **Try the interactive tools at [peasytext.com](https://peasytext.com)** -- [Text Counter](https://peasytext.com/text/text-counter/), [Case Converter](https://peasytext.com/text/text-case-converter/), [Slug Generator](https://peasytext.com/text/slug-generator/), and more.

<p align="center">
  <img src="demo.gif" alt="peasytext demo -- text case conversion, slug generation, and word count tools in Rust terminal" width="800">
</p>

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [What You Can Do](#what-you-can-do)
  - [Text Processing Tools](#text-processing-tools)
  - [Browse Text Reference Content](#browse-text-reference-content)
  - [Search and Discovery](#search-and-discovery)
- [API Client](#api-client)
  - [Available Methods](#available-methods)
- [Learn More About Text Processing](#learn-more-about-text-processing)
- [Also Available](#also-available)
- [Peasy Developer Tools](#peasy-developer-tools)
- [License](#license)

## Install

```toml
[dependencies]
peasytext = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

Or via cargo:

```bash
cargo add peasytext
```

## Quick Start

```rust
use peasytext::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // List available text tools
    let tools = client.list_tools(&Default::default()).await?;
    for tool in &tools.results {
        println!("{}: {}", tool.name, tool.description);
    }

    Ok(())
}
```

## What You Can Do

### Text Processing Tools

Text transformation is fundamental to software development -- from generating URL-safe slugs for blog posts to converting variable names between camelCase and snake_case during code generation. Case conversion tools handle the mechanical work of transforming text between naming conventions used across programming languages (camelCase in JavaScript, snake_case in Python, kebab-case in CSS). Word counting and character analysis help content authors meet length requirements for SEO titles, meta descriptions, and social media posts.

| Tool | Description | Use Case |
|------|-------------|----------|
| Text Counter | Count words, characters, sentences, and paragraphs | Content length validation, SEO metadata |
| Case Converter | Convert between camelCase, snake_case, kebab-case, PascalCase | Code generation, API field mapping |
| Slug Generator | Create URL-safe slugs from any text input | Blog posts, CMS content, URL routing |

```rust
use peasytext::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Fetch the case converter tool details
    let tool = client.get_tool("text-case-converter").await?;
    println!("Tool: {}", tool.name);           // Case converter tool
    println!("Category: {}", tool.category);   // Text processing category

    // List supported text formats
    let formats = client.list_formats(&Default::default()).await?;
    for fmt in &formats.results {
        println!(".{} -- {} ({})", fmt.extension, fmt.name, fmt.mime_type);
    }

    // List available format conversions from plain text
    let conversions = client.list_conversions(&peasytext::ListConversionsOptions {
        source: Some("txt".into()),
        ..Default::default()
    }).await?;
    println!("Found {} conversion paths from text", conversions.results.len());

    Ok(())
}
```

Learn more: [Text Counter](https://peasytext.com/text/text-counter/) · [Case Converter](https://peasytext.com/text/text-case-converter/) · [Text Encoding Guide](https://peasytext.com/guides/text-encoding-utf8-ascii/)

### Browse Text Reference Content

The text processing glossary explains key concepts from basic character encodings to advanced Unicode normalization forms. Understanding the difference between ASCII and UTF-8, how byte order marks (BOM) affect file parsing, and when to use URL encoding versus Base64 helps developers handle text data correctly across platforms and programming languages.

| Glossary Term | Description |
|---------------|-------------|
| Slug | URL-safe string derived from a title or phrase, using hyphens and lowercase |
| Case Conversion | Transforming text between naming conventions like camelCase and snake_case |
| UTF-8 | Variable-width character encoding capable of representing all Unicode code points |
| Whitespace | Characters representing horizontal or vertical space in text rendering |

```rust
// Browse text processing glossary terms
let glossary = client.list_glossary(&peasytext::ListOptions {
    search: Some("slugify".into()),
    ..Default::default()
}).await?;
for term in &glossary.results {
    println!("{}: {}", term.term, term.definition); // Text processing concept definition
}

// Read in-depth guides on text encoding and conversion
let guides = client.list_guides(&peasytext::ListGuidesOptions {
    category: Some("text".into()),
    ..Default::default()
}).await?;
for guide in &guides.results {
    println!("{} ({})", guide.title, guide.audience_level); // Guide title and difficulty
}
```

Learn more: [Slug Glossary](https://peasytext.com/glossary/slug/) · [ASCII Glossary](https://peasytext.com/glossary/ascii/) · [Regex Cheat Sheet](https://peasytext.com/guides/regex-cheat-sheet-essential-patterns/)

### Search and Discovery

The unified search endpoint queries across all text tools, glossary terms, guides, and supported formats simultaneously. This is useful for building text editor plugins, documentation search, or CLI tools that need to discover text processing capabilities.

```rust
// Search across all text tools, glossary, and guides
let results = client.search("case", Some(20)).await?;
println!("Found {} tools, {} glossary terms",
    results.results.tools.len(),
    results.results.glossary.len()); // Cross-content text processing search results
```

Learn more: [BOM Glossary](https://peasytext.com/glossary/bom/) · [Slug Generator](https://peasytext.com/text/slug-generator/) · [All Text Guides](https://peasytext.com/guides/)

## API Client

The client wraps the [PeasyText REST API](https://peasytext.com/developers/) with strongly-typed Rust structs using serde deserialization.

```rust
use peasytext::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Or with a custom base URL:
    // let client = Client::with_base_url("https://custom.example.com");

    // List tools with filters
    let opts = peasytext::ListOptions {
        page: Some(1),
        limit: Some(10),
        search: Some("case".into()),
        ..Default::default()
    };
    let tools = client.list_tools(&opts).await?;

    // Get a specific tool
    let tool = client.get_tool("text-case").await?;
    println!("{}: {}", tool.name, tool.description);

    // Search across all content
    let results = client.search("case", Some(20)).await?;
    println!("Found {} tools", results.results.tools.len());

    // Browse the glossary
    let glossary = client.list_glossary(&peasytext::ListOptions {
        search: Some("slugify".into()),
        ..Default::default()
    }).await?;
    for term in &glossary.results {
        println!("{}: {}", term.term, term.definition);
    }

    // Discover guides
    let guides = client.list_guides(&peasytext::ListGuidesOptions {
        category: Some("text".into()),
        ..Default::default()
    }).await?;
    for guide in &guides.results {
        println!("{} ({})", guide.title, guide.audience_level);
    }

    // List format conversions
    let conversions = client.list_conversions(&peasytext::ListConversionsOptions {
        source: Some("txt".into()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `list_tools(&opts)` | List tools (paginated, filterable) |
| `get_tool(slug)` | Get tool by slug |
| `list_categories(&opts)` | List tool categories |
| `list_formats(&opts)` | List file formats |
| `get_format(slug)` | Get format by slug |
| `list_conversions(&opts)` | List format conversions |
| `list_glossary(&opts)` | List glossary terms |
| `get_glossary_term(slug)` | Get glossary term |
| `list_guides(&opts)` | List guides |
| `get_guide(slug)` | Get guide by slug |
| `list_use_cases(&opts)` | List use cases |
| `search(query, limit)` | Search across all content |
| `list_sites()` | List Peasy sites |
| `openapi_spec()` | Get OpenAPI specification |

Full API documentation at [peasytext.com/developers/](https://peasytext.com/developers/).
OpenAPI 3.1.0 spec: [peasytext.com/api/openapi.json](https://peasytext.com/api/openapi.json).

## Learn More About Text Processing

- **Tools**: [Text Counter](https://peasytext.com/text/text-counter/) · [Case Converter](https://peasytext.com/text/text-case-converter/) · [Slug Generator](https://peasytext.com/text/slug-generator/) · [All Tools](https://peasytext.com/)
- **Guides**: [Text Encoding Guide](https://peasytext.com/guides/text-encoding-utf8-ascii/) · [Regex Cheat Sheet](https://peasytext.com/guides/regex-cheat-sheet-essential-patterns/) · [All Guides](https://peasytext.com/guides/)
- **Glossary**: [Slug](https://peasytext.com/glossary/slug/) · [ASCII](https://peasytext.com/glossary/ascii/) · [BOM](https://peasytext.com/glossary/bom/) · [All Terms](https://peasytext.com/glossary/)
- **Formats**: [TXT](https://peasytext.com/formats/txt/) · [CSV](https://peasytext.com/formats/csv/) · [All Formats](https://peasytext.com/formats/)
- **API**: [REST API Docs](https://peasytext.com/developers/) · [OpenAPI Spec](https://peasytext.com/api/openapi.json)

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| **Python** | [peasytext](https://pypi.org/project/peasytext/) | `pip install "peasytext[all]"` |
| **TypeScript** | [peasytext](https://www.npmjs.com/package/peasytext) | `npm install peasytext` |
| **Go** | [peasytext-go](https://pkg.go.dev/github.com/peasytools/peasytext-go) | `go get github.com/peasytools/peasytext-go` |
| **Ruby** | [peasytext](https://rubygems.org/gems/peasytext) | `gem install peasytext` |

## Peasy Developer Tools

Part of the [Peasy Tools](https://peasytools.com) open-source developer ecosystem.

| Package | PyPI | npm | crates.io | Description |
|---------|------|-----|-----------|-------------|
| peasy-pdf | [PyPI](https://pypi.org/project/peasy-pdf/) | [npm](https://www.npmjs.com/package/peasy-pdf) | [crate](https://crates.io/crates/peasy-pdf) | PDF merge, split, rotate, compress -- [peasypdf.com](https://peasypdf.com) |
| peasy-image | [PyPI](https://pypi.org/project/peasy-image/) | [npm](https://www.npmjs.com/package/peasy-image) | [crate](https://crates.io/crates/peasy-image) | Image resize, crop, convert, compress -- [peasyimage.com](https://peasyimage.com) |
| peasy-audio | [PyPI](https://pypi.org/project/peasy-audio/) | [npm](https://www.npmjs.com/package/peasy-audio) | [crate](https://crates.io/crates/peasy-audio) | Audio trim, merge, convert, normalize -- [peasyaudio.com](https://peasyaudio.com) |
| peasy-video | [PyPI](https://pypi.org/project/peasy-video/) | [npm](https://www.npmjs.com/package/peasy-video) | [crate](https://crates.io/crates/peasy-video) | Video trim, resize, thumbnails, GIF -- [peasyvideo.com](https://peasyvideo.com) |
| peasy-css | [PyPI](https://pypi.org/project/peasy-css/) | [npm](https://www.npmjs.com/package/peasy-css) | [crate](https://crates.io/crates/peasy-css) | CSS minify, format, analyze -- [peasycss.com](https://peasycss.com) |
| peasy-compress | [PyPI](https://pypi.org/project/peasy-compress/) | [npm](https://www.npmjs.com/package/peasy-compress) | [crate](https://crates.io/crates/peasy-compress) | ZIP, TAR, gzip compression -- [peasytools.com](https://peasytools.com) |
| peasy-document | [PyPI](https://pypi.org/project/peasy-document/) | [npm](https://www.npmjs.com/package/peasy-document) | [crate](https://crates.io/crates/peasy-document) | Markdown, HTML, CSV, JSON conversion -- [peasyformats.com](https://peasyformats.com) |
| **peasytext** | [PyPI](https://pypi.org/project/peasytext/) | [npm](https://www.npmjs.com/package/peasytext) | [crate](https://crates.io/crates/peasytext) | **Text case conversion, slugify, word count -- [peasytext.com](https://peasytext.com)** |

## License

MIT
