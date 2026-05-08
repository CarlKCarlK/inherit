# Puzzle 7

We want to add a new method `is_odd()` to an existing concrete type `usize`, even though we do not own that type.

```mermaid
classDiagram
    direction TB

    class UsizeExtensions {
        <<superclass>>
        +is_odd() bool
    }

    class usize {
        <<concrete subclass>>
        +is_odd() bool
    }

    UsizeExtensions <|-- usize : is-a
```
