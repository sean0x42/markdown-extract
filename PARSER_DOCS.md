Say our current document state looks like this.

```yml
Document:
  BlockQuote:
    Paragraph: 'Hello world'
```

Upon receiving a line like so:

```
> This is the remainder of the line
```

We iterate through each open block, and determine whether the new line is
a match. e.g.

| Block      | Requirements | Is match? |
| ---------- | ------------ | --------- |
| Document   | Any          | Yes       |
| BlockQuote | Starts with 0-3 spaces, a '>', and 0-1 spaces. | Yes |
| Paragraph  | Is non blank string | Yes |

In this case, we consume the continuation markers (`> `) and add the remainder
of the line to the last open block. Seeing the following result:

```yml
Document:
  BlockQuote:
    Paragraph: 'Hello world\nThis is the remainder of the line'
```

>    >  Hello
> H
