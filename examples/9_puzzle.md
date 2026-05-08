# Puzzle 9

We want a `FlashBlockable` abstraction where any concrete `T` can be saved and loaded by key, but only when that `T` also satisfies serialization and deserialization capabilities.

```mermaid
classDiagram
    direction TB

    class FlashBlockable {
        <<superclass>>
        +save(flash_block, key)
        +load(flash_block, key)
    }

    class Serialize {
        <<superclass>>
        +serialize()
    }

    class Deserialize {
        <<superclass>>
        +deserialize()
    }

    class T {
        <<concrete subclass>>
        +serialize() // inherited
        +deserialize() // inherited
        +save(flash_block, key) // inherited
        +load(flash_block, key) // inherited
    }

    Serialize <|-- FlashBlockable : is-a
    Deserialize <|-- FlashBlockable : is-a
    FlashBlockable <|-- T : is-a
```
