# Mdtools

Mdtools is a tool used for editing markdown file. It is able to:

- Add tag into frontmatter.
- Make footlinks.

## Add tags

Update tags of frontmatter of files.

Before run

```rust
// C:\mdfiles\Readings.md
---
tags: [readings]
---
...

```

Run command

```
$ hugox add-tag "C:\mdfiles\" news books
```

After run

```rust
// C:\mdfiles\Readings.md
---
tags: [books, news, readings]
---
...

```
