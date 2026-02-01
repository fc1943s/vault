
```custom-frames
urlSuffix: github.com/mrakgr/The-Spiral-Language
frame: https://
style: height:50vh
```

```


'\n \na\n\n' becomes '\n \na\n' instead of '\n \na\n\n', \n\na\n\n becomes \na\n instead of \na\n\n, a\n\n becomes a\n instead of a\n\n







inl main () =
    inl x = (* test *) 1i32
    inl x = """a
b
c"""


inl main () =
    inl x = (* test *) 1i32
    inl x = """a
b
c"""


inl main () =
    inl x = (* test *) 1i32
    inl x = """a
b
c"""


```

```
there's something in the parse pipeline hurting p99 making the algorithm take minutes for large files, it should be instantaneous like fparsec. rework what's needed outputting the changes in md diff form, no comments or explanations. prioritize changing the parsing lib than the dib_export functions (since they're already instant using fparsec). don't suggest removing the thunk from result though (from '() -> t' to t), that was added as an improvement attempt (didn't help, but didn't make it slower, it will be removed later, but for now let's make it instant without changing this part)

---

## dib_export

---

# parsing.dib

---
```

```








```