# βοΈ upl

The intuitive command-line file uploader.

## πΊοΈ Features (roadmap)

- [x] Upload files via the command-line
- [x] Sane built-in upload destinations
- [ ] Configuration files via TOML
- [ ] ShareX custom uploader (.sxcu) support

## ποΈ Installation

```bash
$ cargo install upl
```

## βοΈ Usage

View usage with the `upl --help` command.

### π€ Examples

- `upl --file foo.zip`
- `upl -f bar.png`

### π οΈ Augment

upl is a tool meant to *only* be used for uploading files, I recommend augmenting upl with a tool like [maim](https://github.com/naelstrof/maim) and [xclip](https://github.com/astrand/xclip).
