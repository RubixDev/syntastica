#include <wctype.h>

int iswalpha(wint_t ch) {
    return (ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122);
}

int iswspace(wint_t ch) {
    return ch == 32 || (ch >= 9 && ch <= 13);
}

int iswalnum(wint_t ch) {
    return (ch >= 48 && ch <= 57) || (ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122);
}
