#[cfg(feature = "parsers-all")]
use std::collections::BTreeMap;

#[cfg(feature = "parsers-all")]
use syntastica::{config::ThemeValue, renderer::TerminalRenderer};

#[cfg(not(feature = "parsers-all"))]
fn main() {
    compile_error!("this example requires the `parsers-all` feature to be enabled");
}

#[cfg(feature = "parsers-all")]
fn main() {
    example(
        r###"
fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    Regex::new(r"[a-fA-F0-9_]\s(.*)$");
}
"###,
        "rs",
    );

    example(
        r###"
import re

def fib(n: int) -> int:
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

def main():
    re.compile(r'[a-fA-F0-9_]\s(.*)$')
"###,
        "py",
    );

    example(
        r###"
int fib(int n) {
    if (n < 2) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
"###,
        "c",
    );

    example(
        r###"
#include <iostream>

int main() {
    unsigned int a = 1, b = 1;
    unsigned int target = 48;
    for (unsigned int n = 3; n <= target; ++n) {
        unsigned int fib = a + b;
        std::cout << "F("<< n << ") = " << fib << std::endl;
        a = b;
        b = fib;
    }

    return 0;
}
"###,
        "cpp",
    );

    example(
        r###"
:root {
    --bg-dark: #000;
}

#app.dark {
    background-color: var(--bg-dark);
}
"###,
        "css",
    );

    example(
        r###"
import (
    "math/big"
)

func fib(n uint64) *big.Int {
    if n < 2 {
        return big.NewInt(int64(n))
    }
    a, b := big.NewInt(0), big.NewInt(1)
    for n--; n > 0; n-- {
        a.Add(a, b)
        a, b = b, a
    }
    return b
}

func main() {
    regexp.Compile(`[a-fA-F0-9_]\s(.*)$`)
}
"###,
        "go",
    );

    example(
        r###"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <link rel="icon" type="image/svg+xml" href="/assets/logo.svg" />
        <link rel="stylesheet" href="/assets/theme.css" />
        <link rel="stylesheet" href="/src/global.scss" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="description" content="The playground for the rush programming language" />
        <title>rush Playground</title>
    </head>
    <body>
        <div id="app"></div>
        <script type="module" src="/src/main.ts"></script>
        <style>
            :root {
                --bg-dark: #000;
            }

            #app.dark {
                background-color: var(--bg-dark);
            }
        </style>
        <button style="background-color: red" onclick="alert(window.location.href)">Test</button>
    </body>
</html>
"###,
        "html",
    );

    example(
        r###"
class Fibonacci {
    /**
    * O(log(n))
    */
    public static long fib(long n) {
        if (n <= 0)
        return 0;

        long i = (int) (n - 1);
        long a = 1, b = 0, c = 0, d = 1, tmp1,tmp2;

        while (i > 0) {
            if (i % 2 != 0) {
                tmp1 = d * b + c * a;
                tmp2 = d * (b + a) + c * b;
                a = tmp1;
                b = tmp2;
            }

            tmp1 = (long) (Math.pow(c, 2) + Math.pow(d, 2));
            tmp2 = d * (2 * c + d);

            c = tmp1;
            d = tmp2;

            i = i / 2;
        }
        return a + b;
    }
}
    "###,
        "java",
    );

    example(
        r###"
var fib = (function(cache){
    return cache = cache || {}, function(n){
        if (cache[n]) return cache[n];
        else return cache[n] = n == 0 ? 0 : n < 0 ? -fib(-n)
            : n <= 2 ? 1 : fib(n-2) + fib(n-1);
    };
})();
            "###,
        "js",
    );

    example(
        r###"
{
    "key": "value",
    "good": false,
    "age": 42,
    "percentage": 0.3,
    "nothing": null,
    "list": [1, 2, 3],
    "object": {
        "key": "value"
    }
}
            "###,
        "json",
    );

    //     example(
    //         r###"
    // {
    //     "key": "value",
    //     "good": false,
    //     "age": 42,
    //     "percentage": 0.3,
    //     "nothing": null,
    //     "list": [1, 2, 3],
    //     // line comment
    //     "object": {
    //         "key": /* block comment */ "value"
    //     }
    // }
    //             "###,
    //         "jsonc",
    //     );
    //
    //     example(
    //         r###"
    // {
    //     key: "value",
    //     good: false,
    //     age: 42,
    //     percentage: 0.3,
    //     nothing: null,
    //     list: [1, 2, 3],
    //     // line comment
    //     object: {
    //         "key": /* block comment */ "value",
    //     },
    // }
    //             "###,
    //         "json5",
    //     );

    example(
        r###"
interface FooProp {
  name: string;
  X: number;
  Y: number;
}
declare function AnotherComponent(prop: { name: string });
function ComponentFoo(prop: FooProp) {
  return <AnotherComponent name={prop.name} />;
}
const Button = (prop: { value: string }, context: { color: string }) => (
  <button />
);
            "###,
        "tsx",
    );

    example(
        r###"
interface User {
  name: string;
  id: number;
}

class UserAccount {
  name: string;
  id: number;

  constructor(name: string, id: number) {
    this.name = name;
    this.id = id;
  }
}

const user: User = new UserAccount("Murphy", 1);
            "###,
        "tsx",
    );

    example(
        r###"
.intel_syntax
.global _start

.section .text

_start:
    call        main..main
    mov         %rdi, 0
    call        exit

main..main:
    push        %rbp
    mov         %rbp, %rsp
    sub         %rsp, 32
    mov         qword ptr [%rbp-8], 3
    lea         %rax, qword ptr [%rbp-8]
    mov         qword ptr [%rbp-16], %rax
    lea         %rax, qword ptr [%rbp-16]
    mov         qword ptr [%rbp-24], %rax
    mov         %rax, qword ptr [%rbp-24]
    mov         %rax, qword ptr [%rax]
    mov         qword ptr [%rbp-32], %rax
    mov         %rdi, qword ptr [%rbp-24]
    mov         %rdi, qword ptr [%rdi]
    mov         %rdi, qword ptr [%rdi]
    mov         %rsi, qword ptr [%rbp-24]
    mov         %rsi, qword ptr [%rsi]
    mov         %rsi, qword ptr [%rsi]
    call        __rush_internal_pow_int
    mov         %rdi, %rax
    mov         %rax, qword ptr [%rbp-32]
    mov         qword ptr [%rax], %rdi
    mov         %rdi, qword ptr [%rbp-24]
    mov         %rdi, qword ptr [%rdi]
    mov         %rdi, qword ptr [%rdi]
    call        exit
main..main.return:
    leave
    ret
            "###,
        "asm",
    );
}

#[cfg(feature = "parsers-all")]
fn example(code: &str, file_extension: &str) {
    println!(
        "{}",
        syntastica::highlight(
            code.trim(),
            file_extension,
            &mut TerminalRenderer,
            theme().into()
        )
        .unwrap()
    );
}

#[cfg(feature = "parsers-all")]
pub fn theme() -> BTreeMap<String, ThemeValue> {
    BTreeMap::from([
        ("black".to_owned(), ThemeValue::Simple("#181a1f".to_owned())),
        ("bg0".to_owned(), ThemeValue::Simple("#282c34".to_owned())),
        ("bg1".to_owned(), ThemeValue::Simple("#31353f".to_owned())),
        ("bg2".to_owned(), ThemeValue::Simple("#393f4a".to_owned())),
        ("bg3".to_owned(), ThemeValue::Simple("#3b3f4c".to_owned())),
        ("bg_d".to_owned(), ThemeValue::Simple("#21252b".to_owned())),
        (
            "bg_blue".to_owned(),
            ThemeValue::Simple("#73b8f1".to_owned()),
        ),
        (
            "bg_yellow".to_owned(),
            ThemeValue::Simple("#ebd09c".to_owned()),
        ),
        ("fg".to_owned(), ThemeValue::Simple("#abb2bf".to_owned())),
        (
            "purple".to_owned(),
            ThemeValue::Simple("#c678dd".to_owned()),
        ),
        ("green".to_owned(), ThemeValue::Simple("#98c379".to_owned())),
        (
            "orange".to_owned(),
            ThemeValue::Simple("#d19a66".to_owned()),
        ),
        ("blue".to_owned(), ThemeValue::Simple("#61afef".to_owned())),
        (
            "yellow".to_owned(),
            ThemeValue::Simple("#e5c07b".to_owned()),
        ),
        ("cyan".to_owned(), ThemeValue::Simple("#56b6c2".to_owned())),
        ("red".to_owned(), ThemeValue::Simple("#e86671".to_owned())),
        ("grey".to_owned(), ThemeValue::Simple("#5c6370".to_owned())),
        (
            "light_grey".to_owned(),
            ThemeValue::Simple("#848b98".to_owned()),
        ),
        (
            "dark_cyan".to_owned(),
            ThemeValue::Simple("#2b6f77".to_owned()),
        ),
        (
            "dark_red".to_owned(),
            ThemeValue::Simple("#993939".to_owned()),
        ),
        (
            "dark_yellow".to_owned(),
            ThemeValue::Simple("#93691d".to_owned()),
        ),
        (
            "dark_purple".to_owned(),
            ThemeValue::Simple("#8a3fa0".to_owned()),
        ),
        (
            "diff_add".to_owned(),
            ThemeValue::Simple("#31392b".to_owned()),
        ),
        (
            "diff_delete".to_owned(),
            ThemeValue::Simple("#382b2c".to_owned()),
        ),
        (
            "diff_change".to_owned(),
            ThemeValue::Simple("#1c3448".to_owned()),
        ),
        (
            "diff_text".to_owned(),
            ThemeValue::Simple("#2c5372".to_owned()),
        ),
        (
            "annotation".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "attribute".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "boolean".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "character".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "comment".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("grey".to_owned()),
            },
        ),
        (
            "conditional".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "constant".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constant.builtin".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constant.macro".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constructor".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("yellow".to_owned()),
            },
        ),
        ("error".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "exception".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("field".to_owned(), ThemeValue::Simple("$cyan".to_owned())),
        ("float".to_owned(), ThemeValue::Simple("$orange".to_owned())),
        (
            "function".to_owned(),
            ThemeValue::Simple("$blue".to_owned()),
        ),
        (
            "function.builtin".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "function.macro".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "include".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword.function".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword.operator".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("label".to_owned(), ThemeValue::Simple("$red".to_owned())),
        ("method".to_owned(), ThemeValue::Simple("$blue".to_owned())),
        (
            "namespace".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        ("none".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "number".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        ("operator".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "parameter".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "parameter.reference".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "property".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "punctuation.delimiter".to_owned(),
            ThemeValue::Simple("$light_grey".to_owned()),
        ),
        (
            "punctuation.bracket".to_owned(),
            ThemeValue::Simple("$light_grey".to_owned()),
        ),
        (
            "punctuation.special".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "repeat".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("string".to_owned(), ThemeValue::Simple("$green".to_owned())),
        (
            "string.regex".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "string.escape".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        ("symbol".to_owned(), ThemeValue::Simple("$cyan".to_owned())),
        ("tag".to_owned(), ThemeValue::Simple("$purple".to_owned())),
        (
            "tag.delimiter".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("text".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "text.strong".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.emphasis".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.underline".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: true,
                strikethrough: false,
                italic: false,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.strike".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: true,
                italic: false,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.title".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("orange".to_owned()),
            },
        ),
        (
            "text.literal".to_owned(),
            ThemeValue::Simple("$green".to_owned()),
        ),
        (
            "text.uri".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: true,
                strikethrough: false,
                italic: false,
                bold: false,
                link: Some("cyan".to_owned()),
            },
        ),
        (
            "text.todo".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("red".to_owned()),
            },
        ),
        ("text.math".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "text.reference".to_owned(),
            ThemeValue::Simple("$blue".to_owned()),
        ),
        (
            "text.environment".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.environment.name".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        ("note".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("warning".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("danger".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("type".to_owned(), ThemeValue::Simple("$yellow".to_owned())),
        (
            "type.builtin".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "type.qualifier".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("variable".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "variable.builtin".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "string.special.grammar".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "symbol.grammar.pascal".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        (
            "symbol.grammar.camel".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "symbol.grammar.upper".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "symbol.grammar.lower".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "storageclass.lifetime".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "tag.attribute".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        (
            "text.environment".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.environment.name".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.diff.add".to_owned(),
            ThemeValue::Simple("$green".to_owned()),
        ),
        (
            "text.diff.delete".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
    ])
}
