# Puzzle 4

We want a small `LedLevel` enum type with two values (`On`, `Off`) that automatically participates in common behaviors (default value, debugging output, equality/order comparisons, hashing, copy/clone).


```mermaid
classDiagram
    direction TB

    class LedLevel {
        <<subclass>>
        On
        Off
    }

    class Defaultable {
        <<superclass>>
        +default()
    }

    class Debuggable {
        <<superclass>>
        +debug_string()
    }

    class EquatableOrdered {
        <<superclass>>
        +equals(other)
        +compare(other)
    }

    class Hashable {
        <<superclass>>
        +hash()
    }

    class CopyableCloneable {
        <<superclass>>
        +copy()
        +clone()
    }

    Defaultable <|-- LedLevel : is-a
    Debuggable <|-- LedLevel : is-a
    EquatableOrdered <|-- LedLevel : is-a
    Hashable <|-- LedLevel : is-a
    CopyableCloneable <|-- LedLevel : is-a
```
