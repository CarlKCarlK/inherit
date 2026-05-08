# Puzzle 1

In our library for sets of integers, we require that all integers have certain functions like `min_value`. We also want all integers to inherit certain other functions such as `exhausted_range`.

```mermaid
classDiagram
    direction TB

    class Integer {
        <<superclass>>
        +min_value() Self // required
        +max_value() Self // required
        +exhausted_range() RangeInclusive~Self~  // inherited
    }

    class u8 {
        <<subclass>>
        +min_value() Self
        +max_value() Self
    }

    class i16 {
        <<subclass>>
        +min_value() Self
        +max_value() Self
    }

    Integer <|-- u8 : is-a
    Integer <|-- i16 : is-a
```
