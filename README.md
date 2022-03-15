# hugox
A companion to help edit markdown files.

# Subcommands

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
