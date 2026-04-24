;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-arduino/53eb391da4c6c5857f8defa2c583c46c2594f565/queries/highlights.scm
(
  (identifier) @function.builtin
  (#any-of?
    @function.builtin
    ; Digital I/O
    "digitalRead"
    "digitalWrite"
    "pinMode"
    ; Analog I/O
    "analogRead"
    "analogReference"
    "analogWrite"
    ; Zero, Due & MKR Family
    "analogReadResolution"
    "analogWriteResolution"
    ; Advanced I/O
    "noTone"
    "pulseIn"
    "pulseInLong"
    "shiftIn"
    "shiftOut"
    "tone"
    ; Time
    "delay"
    "delayMicroseconds"
    "micros"
    "millis"
    ; Math
    "abs"
    "constrain"
    "map"
    "max"
    "min"
    "pow"
    "sq"
    "sqrt"
    ; Trigonometry
    "cos"
    "sin"
    "tan"
    ; Characters
    "isAlpha"
    "isAlphaNumeric"
    "isAscii"
    "isControl"
    "isDigit"
    "isGraph"
    "isHexadecimalDigit"
    "isLowerCase"
    "isPrintable"
    "isPunct"
    "isSpace"
    "isUpperCase"
    "isWhitespace"
    ; Random Numbers
    "random"
    "randomSeed"
    ; Bits and Bytes
    "bit"
    "bitClear"
    "bitRead"
    "bitSet"
    "bitWrite"
    "highByte"
    "lowByte"
    ; External Interrupts
    "attachInterrupt"
    "detachInterrupt"
    ; Interrupts
    "interrupts"
    "noInterrupts"
  )
)

(
  (identifier) @type.builtin
  (#any-of?
    @type.builtin
    "Serial"
    "SPI"
    "Stream"
    "Wire"
    "Keyboard"
    "Mouse"
    "String"
  )
)

(
  (identifier) @constant.builtin
  (#any-of?
    @constant.builtin
    "HIGH"
    "LOW"
    "INPUT"
    "OUTPUT"
    "INPUT_PULLUP"
    "LED_BUILTIN"
  )
)

(function_definition
  (function_declarator
    declarator: (identifier) @function.builtin
  )
  (#any-of? @function.builtin "loop" "setup")
)

(call_expression
  function: (identifier) @constructor.builtin
  (#any-of? @constructor.builtin "SPISettings" "String")
)

(declaration
  (type_identifier) @type.builtin
  (function_declarator
    declarator: (identifier) @constructor.builtin
  )
  (#eq? @type.builtin "SPISettings")
)
