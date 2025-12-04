# Rail Grammar

## Expressions

```ebnf
<Expression>
  ::= <LiteralExpression>
    | <OperatorExpression>
    | <GroupedExpression>

<NegationExpression>
  ::= "!" <Expression>
    | "-" <Expression>

<OperatorExpression>
  ::= <ArithmeticExpression>
    | <ComparisonExpression>
    | <LazyBooleanExpression>

<ArithmeticExpression>
  ::= <Expression> "+" <Expression>
    | <Expression> "-" <Expression>
    | <Expression> "*" <Expression>
    | <Expression> "/" <Expression>

<ComparisonExpression>
  ::= <Expression> "==" <Expression>
    | <Expression> "!=" <Expression>
    | <Expression> "<"  <Expression>
    | <Expression> "<=" <Expression>
    | <Expression> ">"  <Expression>
    | <Expression> ">=" <Expression>

<LazyBooleanExpression>
  ::= <Expression> "||" <Expression>
    | <Expression> "&&" <Expression>

<GroupedExpression> ::= "(" <Expression> ")"

<LiteralExpression>
  ::= <IntegerLiteral>
    | <FloatLiteral>
    | "true"
    | "false"
```

## Literals

```ebnf
<Letter>
  ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
    | "H" | "I" | "J" | "K" | "L" | "M" | "N"
    | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
    | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
    | "c" | "d" | "e" | "f" | "g" | "h" | "i"
    | "j" | "k" | "l" | "m" | "n" | "o" | "p"
    | "q" | "r" | "s" | "t" | "u" | "v" | "w"
    | "x" | "y" | "z"

<BinDigit> ::= "0" | "1"

<DecDigit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

<BinLiteral> ::= <BinDigit>+

<DecLiteral> ::= <DecDigit>+

<IntegerLiteral>
  ::= <BinLiteral>
    | <DecLiteral>

<FloatLiteral> ::= <DecLiteral> "." <DecLiteral>
```

## Statements

```ebnf
<Statement>
  ::= ";"
    | <ExpressionStatement>

<ExpressionStatement>
  ::= <Expression> ";"
```

## Types

```ebnf
<Type>
  ::= <ParethesizedType>
    | <NeverType>
    | <TupleType>

<ParethesizedType> ::= "(" <Type> ")"

<NeverType> ::= "!"

<TupleType>
  ::= "(" ")"
    | "(" (<Type> ",")+ <Type>? ")"
```
