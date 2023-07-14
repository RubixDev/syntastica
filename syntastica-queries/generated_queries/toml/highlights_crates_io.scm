(ERROR) @error

"}" @punctuation.bracket

"{" @punctuation.bracket

"]]" @punctuation.bracket

"[[" @punctuation.bracket

"]" @punctuation.bracket

"[" @punctuation.bracket

"=" @operator

"," @punctuation.delimiter

"." @punctuation.delimiter

(local_time) @string.special

(local_date) @string.special

(local_date_time) @string.special

(offset_date_time) @string.special

(float) @float

(integer) @number

(string) @string

(comment) @comment @spell

(boolean) @boolean

(pair
  (bare_key)
) @property

(quoted_key) @string

(bare_key) @type
