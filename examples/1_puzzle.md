# Puzzle 1

`RangeSetBlaze` stores sets of integers, `u8`, `i16`, etc.. Every integer type must have functions like `min_value`. We also want all integers to inherit certain other functions such as `exhausted_range`.

```mermaid
classDiagram
    direction TB

    class Integer {
        <<superclass>>
        +min_value() Self // required
        +max_value() Self // required
        +exhausted_range() RangeInclusive~Self~  // code
    }

    class u8 {
        <<subclass>>
        +min_value() Self
        +max_value() Self
        +exhausted_range() RangeInclusive~Self~  // inherited
    }

    class i16 {
        <<subclass>>
        +min_value() Self
        +max_value() Self
        +exhausted_range() RangeInclusive~Self~  // inherited
    }

    Integer <|-- u8 : is-a
    Integer <|-- i16 : is-a
```
