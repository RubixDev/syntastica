#include <wctype.h>

// https://en.cppreference.com/w/cpp/string/wide/iswdigit

int  iswalnum(wint_t ch) { return (ch >= 48 && ch <= 57) || (ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122); }
int  iswalpha(wint_t ch) { return (ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122); }
int  iswblank(wint_t ch) { return ch == 9 || ch == 32; }
int  iswcntrl(wint_t ch) { return (ch >= 0 && ch <= 31) || ch == 127; }
int  iswdigit(wint_t ch) { return ch >= 48 && ch <= 57; }
int  iswgraph(wint_t ch) { return ch >= 33 && ch <= 126; }
int  iswlower(wint_t ch) { return ch >= 97 && ch <= 122; }
int  iswprint(wint_t ch) { return ch >= 32 && ch <= 126; }
int  iswpunct(wint_t ch) { return (ch >= 33 && ch <= 47) || (ch >= 58 && ch <= 64) || (ch >= 91 && ch <= 96) || (ch >= 123 && ch <= 126); }
int  iswspace(wint_t ch) { return ch == 32 || (ch >= 9 && ch <= 13); }
int  iswupper(wint_t ch) { return ch >= 65 && ch <= 90; }
int iswxdigit(wint_t ch) { return (ch >= 48 && ch <= 57) || (ch >= 65 && ch <= 70) || (ch >= 97 && ch <= 102); }
