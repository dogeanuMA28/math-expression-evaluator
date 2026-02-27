# Expression Evaluator (Rust)

A command-line application written in **Rust** that parses, validates, and evaluates mathematical expressions containing real numbers, arithmetic operators, and parentheses.

## About it

This project implements a mathematical expression evaluator without using external parsing 
libraries.  
The program reads an expression from standard input, tokenizes it, checks its validity and 
evaluates it while respecting operator precedence and nested parentheses.

Supported operators:
- addition (`+`)
- subtraction (`-`)
- multiplication (`*`)
- division (`/`)
- parentheses (`()`)

---

## How It Works

The evaluation process is divided into three stages:

1. **Tokenization**
   The input string is converted into a sequence of tokens (numbers, operators, parentheses).

2. **Validation**  
   The token sequence is checked to ensure the expression is syntactically correct (valid operator order and balanced parentheses).

3. **Evaluation**  
   The expression is computed using operator precedence rules and contextual stacks to handle nested parentheses.

---
