# MdParser

This module will convert some presumably markdown formatted string into a format usable for the rest of the process. To do this we rely on the following external assumptions:

- the `markdown` crate defines an exhaustive mdast family
- all text is technically valid markdown (I *think* this holds but im not certain)
- something else will create the input strings (i.e. read the input text etc...), this assumption is made to allow users to perform any cleaning etc... they want first.

ultimately, the output of the public `parse` method will be an enumdict, which maps the start and end of each instance of a given type onto the type label, for instance:

```
# Hi *Earth*!
```
producing the following MDAST (accoding to the [mardown docs](https://docs.rs/markdown/latest/markdown/fn.to_mdast.html))

```
Root { children: [Heading { children: [Text { value: "Hi ", position: Some(1:3-1:6 (2-5)) }, Emphasis { children: [Text { value: "Earth", position: Some(1:7-1:12 (6-11)) }], position: Some(1:6-1:13 (5-12)) }, Text { value: "!", position: Some(1:13-1:14 (12-13)) }], position: Some(1:1-1:14 (0-13)), depth: 1 }], position: Some(1:1-1:14 (0-13)) }
```

would parse to

```
Heading [0, 13]
Text [0, 13]
Emphasis [5, 12]
```

Importantly, notice that text covers only a single range, despite the fact that there are multiple 'text' nodes in the mdast. this is because they overlap directly, so are trimmed out to avoid unneccessary computation in future steps