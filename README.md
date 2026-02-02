

> **Note**  
> This document provides a detailed overview of the unique syntax features of the Sylph programming language, focusing on significant differences from other languages.

## 1. Core Syntax Features

### 1.1 Output Statement

Sylph uses the `out` keyword for output, which is its unique syntax feature:

```sylph
// Output a variable
out x

// Output an expression
out 1 + 2 * 3
```

### 1.2 Function Definition

Sylph's function definition starts with the `def` keyword and ends with the `end` keyword, which is its unique syntax structure:

```sylph
def add(a, b)
    return a + b
end
```

### 1.3 Function Call

Sylph's function call directly uses the function name followed by arguments without parentheses, which is its unique syntax feature:

```sylph
// Call a function and pass arguments
result = add 5 3
out result
```

### 1.4 Multi-parameter Functions

Sylph's multi-parameter functions use commas to separate parameters in both definition and call:

```sylph
def multiply(a, b, c)
    return a * b * c
end

// Call a multi-parameter function
result = multiply 2, 3, 4
out result
```

### 1.5 Conditional Statement

Sylph's conditional statement starts with the `if` keyword and ends with the `end` keyword, without needing parentheses or braces:

```sylph
if x > 5
    out "x is greater than 5"
end
```

### 1.6 Loop Statement

Sylph's loop statement starts with the `while` keyword and ends with the `end` keyword, without needing parentheses or braces:

```sylph
while i < 10
    out i
    i += 1
end
```

### 1.7 Variable Declaration and Assignment

Sylph's variable declaration and assignment use the `=` operator without needing a keyword:

```sylph
// Declare and assign a variable
x = 10

// Declare and assign a variable with specified type
y = 20i32
```

### 1.8 Compound Assignment Operators

Sylph supports the following compound assignment operators:

```sylph
// Addition assignment
x += 5

// Subtraction assignment
y -= 3

// Multiplication assignment
z *= 2

// Modulo assignment
a %= 4
```

### 1.9 Integer Type System

Sylph supports multiple integer types specified by type suffixes, which is its unique syntax feature:

| Type Suffix | Example                                |
|-------------|----------------------------------------|
| `i8`        | `10i8`                                 |
| `i16`       | `1000i16`                              |
| `i32`       | `100000i32`                            |
| `i64`       | `1000000000i64`                        |
| `i128`      | `1000000000000000000i128`              |
| `bigint`    | `1000000000000000000000000000000bigint`|

> **Note**  
> Sylph automatically selects the appropriate type for integers without explicit type suffixes based on the value's size, which is its unique type inference feature.

### 1.10 Statement Block End

Sylph uses the `end` keyword as the end marker for statement blocks, which is its unique syntax feature:

```sylph
// End of function definition
def add(a, b)
    return a + b
end

// End of conditional statement
if x > 5
    out "x is greater than 5"
end

// End of loop statement
while i < 10
    out i
    i += 1
end
```

### 1.11 Comments

Sylph uses double slashes `//` for single-line comments:

```sylph
// This is a comment
x = 10 // Inline comment
```

## 2. Example Programs

### 2.1 Simple Calculation

```sylph
// Calculate the sum of two numbers
def add(a, b)
    return a + b
end

// Main program
x = 5
y = 3
result = add x y
out result
```

### 2.2 Loop Example

```sylph
// Calculate the sum from 1 to n
def sum_to_n(n)
    total = 0
    i = 1
    while i <= n
        total += i
        i += 1
    end
    return total
end

// Call the function and output the result
result = sum_to_n 10
out result
```

### 2.3 Conditional Statement Example

```sylph
// Check if a number is positive
def is_positive(n)
    if n > 0
        out "Positive"
    else
        out "Non-positive"
    end
end

// Test the function
is_positive 5
is_positive -3
```

## 3. Syntax Summary

> **Note**  
> Sylph language's core unique syntax features include:
>
> - Using `out` keyword for output  
> - Using `def` keyword to define functions, ending with `end`  
> - Function calls directly using function name followed by arguments without parentheses  
> - Using `if` keyword to start conditional statements, ending with `end`  
> - Using `while` keyword to start loop statements, ending with `end`  
> - Variable declaration and assignment using `=` operator without keywords  
> - Supporting multiple integer types specified by type suffixes  
> - Using `//` for single-line comments