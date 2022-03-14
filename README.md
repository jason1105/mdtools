# hugox
A companion to help edit markdown files.

# Add tags

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
$ hugox add-tag C:\mdfiles news books
```

After run

```rust
// C:\mdfiles\Readings.md
---
tags: [books, news, readings]
---
...

```
