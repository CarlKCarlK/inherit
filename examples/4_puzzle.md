# Puzzle 4

We want a small `LedLevel` enum type with two values (`On`, `Off`) that automatically participates in common behaviors (default value, debugging output, equality/order comparisons, hashing, copy/clone).

```mermaid
classDiagram
    direction TB

    class LedLevel {
        <<concrete class>>
        On
        Off
    }

    class Defaultable {
        <<abstract class>>
        +default()
    }

    class Debuggable {
        <<abstract class>>
        +debug_string()
    }

    class EquatableOrdered {
        <<abstract class>>
        +equals(other)
        +compare(other)
    }

    class Hashable {
        <<abstract class>>
        +hash()
    }

    class CopyableCloneable {
        <<abstract class>>
        +copy()
        +clone()
    }

    Defaultable <|-- LedLevel : is-a
    Debuggable <|-- LedLevel : is-a
    EquatableOrdered <|-- LedLevel : is-a
    Hashable <|-- LedLevel : is-a
    CopyableCloneable <|-- LedLevel : is-a
```
