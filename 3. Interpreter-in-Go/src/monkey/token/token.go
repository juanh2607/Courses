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
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG 	 = "!"
	ASTERISK = "*"
	SLASH    = "/"
	LT       = "<"
	GT       = ">"
	EQ       = "=="
	NOT_EQ   = "!="

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
	TRUE 	 = "TRUE"
	FALSE 	 = "FALSE"
	IF       = "IF"
	ELSE     = "ELSE"
	RETURN   = "RETURN"
)

var keywords = map[string]TokenType {
	"fn": FUNCTION,
	"let": LET,
	"true": TRUE,
	"false": FALSE,
	"if": IF,
	"else": ELSE,
	"return": RETURN,
}

// Defines if the identifier is a keyword or a regular identifier
func LookupIdentifier(identifier string) TokenType {
	// "Asignación con Comprobación de Existencia". Si ok es true =>
	// ya te queda asignado el valor. 2 pájaros de un tiro
	if tok, ok := keywords[identifier]; ok {
		return tok
	}

	return IDENT
}