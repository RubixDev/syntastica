;; Forked from https://raw.githubusercontent.com/charmbracelet/tree-sitter-vhs/0c6fae9d2cfc5b217bfd1fe84a7678f5917116db/queries/highlights.scm
[
  "Output"
  "Backspace"
  "Down"
  "Enter"
  "Escape"
  "Left"
  "Right"
  "Space"
  "Tab"
  "Up"
  "Set"
  "Type"
  "Sleep"
  "Hide"
  "Show"
] @keyword

[
  "Shell"
  "FontFamily"
  "FontSize"
  "Framerate"
  "PlaybackSpeed"
  "Height"
  "LetterSpacing"
  "TypingSpeed"
  "LineHeight"
  "Padding"
  "Theme"
  "LoopOffset"
  "Width"
  "BorderRadius"
  "Margin"
  "MarginFill"
  "WindowBar"
  "WindowBarSize"
  "CursorBlink"
] @type

["@"] @operator

(control) @function.macro

(float) @float

(integer) @number

(comment) @comment @spell

[
  (path)
  (string)
  (json)
] @string

(time) @symbol

(boolean) @boolean
