// El objetivo del lexer es tomar el código fuente y partirlo en "tokens", que
// son unidades léxicas más pequeñas y fáciles de comprender.
// Este "tokenizado" es el primer paso que realiza el interprete.
package token

type TokenType string

type Token struct {
	Type TokenType
	// Los tokens pueden encapsular valores, por ej: INTEGER(1)
	// Un string no es para nada óptimo, pero si muy práctico
	Literal string
}

// TOKEN TYPES
const (
	ILLEGAL = "ILLEGAL" // For unrecognized tokens
	EOF     = "EOF"     // For the parser to know when to stop

	// Identifiers & Literals
	IDENT = "IDENT" // Identifier associated with the variable
	INT   = "INT"

	// Operators
	ASSIGN = "="
	PLUS   = "+"

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"

	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"

	// Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
)
