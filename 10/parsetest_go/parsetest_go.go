package main

import (
	"fmt"
	"io"
	"strconv"
	"strings"
	"unicode"
)

type Token interface {
	token()
	String() string
}

type Symbol struct{ c rune }
type Keyword struct{ s string }
type Identifier struct{ s string }
type IntConst struct{ i uint16 }
type StringConst struct{ s string }

func (*Symbol) token()      {}
func (*Keyword) token()     {}
func (*Identifier) token()  {}
func (*IntConst) token()    {}
func (*StringConst) token() {}

func (s *Symbol) String() string      { return fmt.Sprintf("SYM('%c')", s.c) }
func (s *Keyword) String() string     { return fmt.Sprintf("Keyword(%s)", s.s) }
func (s *Identifier) String() string  { return fmt.Sprintf("ID(%s)", s.s) }
func (s *IntConst) String() string    { return fmt.Sprintf("INT(%d)", s.i) }
func (s *StringConst) String() string { return fmt.Sprintf("STRING(%s)", s.s) }

func isKeyword(w string) bool {
	switch w {
	case "if":
		return true
	case "else":
		return true
	case "do":
		return true
	case "let":
		return true
	default:
		return false
	}
}

func skipWhitespace(rdr io.RuneScanner) {
	for {
		r, _, err := rdr.ReadRune()
		if err != nil {
			break
		}
		if !unicode.IsSpace(r) {
			rdr.UnreadRune()
			break
		}
	}
}

func isIdent(r rune) bool {
	return unicode.IsLetter(r) || unicode.IsDigit(r)
}

func isSymbol(r rune) bool {
	return !isIdent(r)
}

func getWord(rdr io.RuneScanner) string {
	var w strings.Builder
	for {
		r, _, err := rdr.ReadRune()
		if err != nil {
			break
		}
		if isSymbol(r) || unicode.IsSpace(r) {
			rdr.UnreadRune()
			break
		}
		w.WriteRune(r)
	}
	return w.String()
}

func getInt(rdr io.RuneScanner) uint16 {
	w := getWord(rdr)
	i, err := strconv.Atoi(w)
	if err != nil {
		panic("ATOI " + w)
	}
	return uint16(i)
}

func getStringConst(rdr io.RuneScanner) string {
	var w strings.Builder
	for {
		r, _, err := rdr.ReadRune()
		if err != nil {
			break
		}
		if r == '"' {
			break
		}
		w.WriteRune(r)
	}
	return w.String()
}

func getToken(rdr io.RuneScanner) Token {
	skipWhitespace(rdr)
	r, _, err := rdr.ReadRune()
	if err != nil {
		return nil
	}
	if r == '"' {
		return &StringConst{getStringConst(rdr)}
	} else if isSymbol(r) {
		return &Symbol{r}
	} else {
		rdr.UnreadRune()
		if unicode.IsDigit(r) {
			return &IntConst{getInt(rdr)}
		} else {
			w := getWord(rdr)
			if isKeyword(w) {
				return &Keyword{w}
			} else {
				return &Identifier{w}
			}
		}
	}
}

func main() {
	st := "if (a < 3) { let b = 3; } "
	rdr := strings.NewReader(st)
	for {
		t := getToken(rdr)
		if t == nil {
			break
		}
		fmt.Println(t)
	}
}
