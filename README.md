# Unreal Doc
Tool for generating documentation from Unreal C++ sources.

## Table of contents
1. [About](#about)
1. [Installation](#installation)
1. [Config file](#config-file)
    1. [Simple config setup for baking into JSON portable format](#simple-config-setup-for-baking-into-json-portable-format)
    1. [Simple config setup for baking into MD Book](#simple-config-setup-for-baking-into-md-book)
    1. [Advanced config setup for baking into MD Book](#advanced-config-setup-for-baking-into-md-book)
1. [Markdown doc comments](#markdown-doc-comments)
1. [Markdown book pages](#markdown-book-pages)
1. [Run documentation baking command](#run-documentation-baking-command)
1. [Examples](#examples)

## About

Do you create an Unreal Engine plugin and need to generate a documentation combined with book
for it, but Doxygen seems limited or sometimes not working at all?

Fear not - this tool understands Unreal C++ header files syntax and Markdown doc comments, it
can also bake a book pages out of Markdown files, all of that being able to cross-reference
each other!

## Installation

- Installation using `cargo-install` from [Rust toolset](https://rustup.rs/):
    ```bash
    cargo install unreal-doc
    ```
- [Pre-built binaries](https://github.com/PsichiX/unreal-doc/releases)

## Config file

Config TOML file tells this tool evenrythig about how to build documentation for your project.
At this moment there are two baking backends available:
- **`Json`**
    
    Portable representation of documentation and book that can be used in third party
    applications with custom way of baking documentation.

- **`MdBook`**

    Uses [MD Book](https://github.com/rust-lang/mdBook) for baking HTML5 bundle for online or
    offline web books.

> Although config file can be named whatever you want, it's a good rule to give config file
`UnrealDoc.toml` name.

### Simple config setup for baking into JSON portable format

```toml
input_dirs = ["./source"]
output_dir = "./docs"
```

- `input_dirs`
    
    List of directories and Unreal C++ header or Markdown files this tool should read.

- `output_dir`

    Path to directory where generated documentation should be put.

### Simple config setup for baking into MD Book

```toml
input_dirs = ["./source"]
output_dir = "./docs"
backend = "MdBook"

[backend_mdbook]
title = "Documentation"
build = true
cleanup = true
```

- `backend`
    
    Specifies MD Book baking backend.

- `backend_mdbook.title`

    Title of the generated documentation and book bundle.

- `backend_mdbook.build`

    Set to true if this tool should also run `mdbook build` command on generated files to
    build HTML5 version of the bundle, ready to put online.

- `backend_mdbook.cleanup`

    Set to true if this tool should cleanup `output_dir` directory before baking new files.
    Useful for ensuring no old/unwanted files will exist between iterations of documentation
    baking.

### Advanced config setup for baking into MD Book

```toml
input_dirs = ["./source"]
output_dir = "./docs"
backend = "MdBook"

[settings]
document_private = true
document_protected = true
show_all = true

[backend_mdbook]
title = "Documentation"
build = true
cleanup = true
header = "header.md"
footer = "footer.md"
assets = "assets/"
```

- `backend_mdbook.header`

    Path to file that contains Markdown content that will be put on every documentation and
    book page header section.

- `backend_mdbook.footer`

    Path to file that contains Markdown content that will be put on every documentation and
    book page footer section.

- `backend_mdbook.assets`

    Path to directory that contains assets (usually images/animations/videos) referenced in
    documentation and book pages.

## Markdown doc comments

Overview of all possible things you can do with Markdown doc comments.
```c++
#pragma once

using Foo = std::vector<int>;

enum class Something : uint8;

template <typename T>
struct BAR Foo : public Bar;

class FOO Bar;

template <typename T>
void* Main(const Foo& Arg);

/// Description of enum
///
/// More information and examples.
UENUM(BlueprintType, Meta = (Foo = Bar))
enum class Something : uint8
{
	A,
	B
};

/// Description of struct
///
/// More information and examples.
///
/// [`struct: Self::Foo`]()
/// [`struct: Self::A`]()
USTRUCT(BlueprintType, Meta = (Foo = Bar))
template <typename T>
struct BAR Foo : public Bar
{
protected:
	/// What is this method
	///
	/// What it does
	UFUNCTION()
	virtual void Foo(
		/// Argument
		int A,
		/// Argument with default value
		AActor* B = nullptr) const override;

private:
	/// What is this property
	///
	/// What impact does it have
	UPROPERTY()
	int A[] = {0};
};

/// Description of class
///
/// More information and examples.
///
/// [`class: Self::Bar`]()
UCLASS()
class FOO Bar
{
public:
	/// What is this method
	///
	/// What it does
	Bar();
};

/// What is this function
///
/// What does it do
///
/// [`function: Self`]()
///
/// See:
/// - [`enum: Something`]()
/// - [`struct: Foo`]()
/// - [`struct: Foo::Foo`]()
/// - [`struct: Foo::A`]()
/// - [`class: Bar`]()
/// - [`function: Main`]()
///
/// # Examples
/// ```snippet
/// hello_world
/// ```
/// ```snippet
/// hello_world2
/// ```
/// ```snippet
/// wait_what
/// ```
template <typename T>
void* Main(
	/// Some referenced data
	const Foo& Arg)
{
	//// [snippet: hello_world]
	if (true)
	{
		printf("Hello");
	}
	//// [/snippet]

	//// [snippet: hello_world2]
	printf("World");
	//// [/snippet]
}

//// [snippet: wait_what]
struct Wait
{
	int What = 0;
};
//// [/snippet]

/// Proxy documentation for injecting code with macros.
///
//// [proxy: injectable]
//// void Injected() const;
//// [/proxy]
#define INJECT            \
	void Injected() const \
	{                     \
	}

struct Who
{
	int What = 0;

	void SetWhat(int InWhat)
	{
		//// [ignore]
		this->What = InWhat;
		//// [/ignore]
	}

    /// Operator overload.
	friend bool operator==(const Who& Lhs, const Who& Rhs)
	{
		//// [ignore]
		return Lhs.What == Rhs.What;
		//// [/ignore]
	}

	//// [inject: injectable]
	INJECT
};
```

## Markdown book pages

Standard expected structure of the book Markdown files:

- `documentation.md` (optional)

    This is essentially the content of main page of your documentation + book,
    a hello page you can call it. it has to be placed in root of book source
    Markdown files directory.

- `index.txt` (required)

    This file contains a list of files or directories with optional names (useful
    mostly for directories). The order specified in this file will match order on
    side pages index.

    ```
    some_book_page.md
    directory_with_subpages: Title on side pages index
    ```

- `index.md` (optional)

    Markdown file content used in to describe content of given directory content.
    First line of the content will be used as title of the directory on side pages
    index.

    ```md
    # Book

    Optional directory content description
    ```

- `hello.md`

    Content for given book page. First line will be used as page title on side
    pages index.

        # Hello World!

        Lorem ipsum dolor amet

        ```cpp
        void main() {
            printf("Hello World!");
        }
        ```

## Run documentation baking command

Once you have config file and documentation itself all in place, it's time to
actually bake documentation and book bundle:

```bash
unreal-doc -i path/to/UnrealDoc.toml
```

## Example

If you want to see an example of decoumentation and book source files structure,
take a look at [/resources](/resources) directory in this repository or more real
life example that can be found here:
https://github.com/PsichiX/Unreal-Systems-Architecture/tree/master/Plugins/Systems/Documentation

Real life example of baked plugin documentation:
https://psichix.github.io/Unreal-Systems-Architecture/systems