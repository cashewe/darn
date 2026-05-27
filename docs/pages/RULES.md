# Rules

`darn` uses "rules" to determine the cost to splitting text at each index. Rules are applied additively to text based on the markdown structures present. Note that `darn` works through use of **relative cost** - rules should be designed so less desirable behaviours are more costly, but absolute size of little consequence.

you can use `darn` in one of two ways:

1. with the pre-written rules
2. with custom rules

**NOTE** if you define any custom rules they will overwrite the pre-written rules entirely - you must choose between the two usage patterns.

## 1. pre-written rules

to use `darn` with pre-written rules, simply define the chunker without any arguments:

```
chunker = Chunker()
```

and the pre-written rules will be included as part of the initialisation of the object. This will get you up and running with some sensible defaults quickyl, but may not be suitable for your specific documents.

Since the pre-written rules are subject to change and I doubt I'll keep this document up to date, I will not write them out in full here, see [this file](https://github.com/cashewe/darn/blob/main/src/rule_manager/rules.rs) in the source code for the full list.

## 2. custom rules

As of [v1.2.0](https://pypi.org/project/darn-it/1.2.0/), `darn` supports use of custom defined rules. to use custom rules, pass a list of `Rule` objects into the chunker during initialisation:

```
chunker = Chunker(
    [
        Rule(
            on_punishment="const",
            on_scale=10,
            off_punishment="const",
            off_scale=0,
            nodetype=PyNodeType.Sentence
        ),
        Rule(
            on_punishment="const",
            on_scale=20,
            off_punishment="const",
            off_scale=0,
            nodetype=PyNodeType.Word
        ),
        ...
    ]
)
```
This list will replace the default rules entirely, so be sure its set to be exhaustive for your desires. Remember that rules stack, so your most expensive rules may need to be designed to account for low cost stacking etc...

as an explanation for the variables required for `Rule` objects, see the following 'schema':

| Variable | Type | Meaning | Example |
|----------|------|---------|---------|
| nodetype | darn.PyNodeType | which markdown [structure](https://docs.rs/markdown/latest/markdown/mdast/index.html) does the rule apply to? | PyNodeType.Paragraph |
| on_punishment | string | what punishment function should be applied to indexes within structures of type nodetype? | "const" |
| on_scale | int (positive) | how large should the punishment be to indexes within structures of type nodetype? | 20 |
| off_punishment | string | what punishment function should be applied to indexes with structures not of type nodetype? | "const" |
| off_scale | int (positive) | how large should the punishment be to indexes with structures not of type nodetype? | 20 |

### 'on' vs 'off' punishments explained

the distinction between 'on' and 'off' punishments may seem a little confusing at first - and to be fair it is. the logic here is that sometimes the presence of a structure may leave you wanting to punish 'nearby, but not on' that structure. As a concrete example, imagine you decide that whilst cutting a heading in two is absolutely unacceptable, youd also prefer not to cut immediately after the heading, as that would orphan the section heading from its contents. to get round this, you apply an `off_punishment` of "reverse_linear", causing high punishments in the text immediately after the heading, and lower punishments the further from the heading you move.

For the average usecase, it is worth blanket setting the off punishment scale to 0 and ignoring it - it is only useful in a select few circumstances.