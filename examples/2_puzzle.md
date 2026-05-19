# Puzzle 2

A servo is an electric motor that can move to a specified angle. We want a `ServoEsp` (our type that controls a servo on an ESP32 microcontroller) to work with any code that needs a servo. A `ServoPlayerEsp` is similar, but with animation ability. (Inspired by `device-envoy`.)


```mermaid
classDiagram
    direction TB

    class Servo {
        <<superclass>>
        +set_degrees(degrees)
    }

    class ServoPlayer {
        <<superclass>>
        +set_degrees(degrees) // from Servo
        +animate(steps)
    }

    class ServoEsp {
        <<subclass>>
        +set_degrees(degrees)
    }

    class ServoPlayerEsp {
        <<subclass>>
        +set_degrees(degrees)
        +animate(steps)
    }

    Servo <|-- ServoPlayer : is-a
    Servo <|-- ServoEsp : is-a
    ServoPlayer <|-- ServoPlayerEsp : is-a
```
