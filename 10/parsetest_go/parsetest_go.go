package main

import (
	"fmt"
	"io"
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
type IntConst struct{ i int16 }
type StringConst struct{ s string }

func (*Symbol) token()      {}
func (*Keyword) token()     {}
func (*Identifier) token()  {}
func (*IntConst) token()    {}
func (*StringConst) token() {}

func (s *Symbol) String() string      { return fmt.Sprintf("SYM(%c)", s.c) }
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

func getToken(rdr io.RuneScanner) Token {
	skipWhitespace(rdr)
	_, _, err := rdr.ReadRune()
	if err != nil {
		return nil
	} else {
		return &Keyword{"if"}
	}
}

func main() {
	st := "if (a < 3) { let b = 3; } "
	rdr := strings.NewReader(st)
	for {
		t := getToken(rdr)
		fmt.Println(t)
		if t == nil {
			break
		}
	}

	fmt.Println("vim-go")
}
