<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sylph Language Syntax Documentation</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            line-height: 1.6;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        h1, h2, h3, h4, h5, h6 {
            color: #333;
            font-weight: 600;
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 1em;
            border-bottom: 3px solid #333;
            padding-bottom: 0.5em;
        }
        h2 {
            font-size: 2em;
            margin-top: 1.5em;
            margin-bottom: 1em;
            border-bottom: 2px solid #ccc;
            padding-bottom: 0.3em;
        }
        h3 {
            font-size: 1.5em;
            margin-top: 1.2em;
            margin-bottom: 0.8em;
        }
        code {
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            background-color: #f5f5f5;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-size: 0.9em;
        }
        pre {
            background-color: #f5f5f5;
            padding: 1em;
            border-radius: 5px;
            overflow-x: auto;
            margin: 1em 0;
        }
        pre code {
            background-color: transparent;
            padding: 0;
        }
        table {
            border-collapse: collapse;
            width: 100%;
            margin: 1em 0;
        }
        th, td {
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }
        th {
            background-color: #f2f2f2;
            font-weight: 600;
        }
        .highlight {
            background-color: #fff3cd;
            padding: 0.2em 0.4em;
            border-radius: 3px;
        }
        .two-column {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            margin: 1em 0;
        }
        .note {
            background-color: #e7f3ff;
            border-left: 4px solid #2196F3;
            padding: 1em;
            margin: 1em 0;
            border-radius: 0 5px 5px 0;
        }
        .warning {
            background-color: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 1em;
            margin: 1em 0;
            border-radius: 0 5px 5px 0;
        }
    </style>
</head>
<body>
    <h1>Sylph Language Syntax Documentation</h1>
    
    <div class="note">
        <p>This document provides a detailed overview of the unique syntax features of the Sylph programming language, focusing on significant differences from other languages.</p>
    </div>

    <h2>1. Core Syntax Features</h2>

    <h3>1.1 Output Statement</h3>
    
    <p>Sylph uses the <code>out</code> keyword for output, which is its unique syntax feature:</p>
    
    <pre><code>// Output a variable
out x

// Output an expression
out 1 + 2 * 3</code></pre>

    <h3>1.2 Function Definition</h3>
    
    <p>Sylph's function definition starts with the <code>def</code> keyword and ends with the <code>end</code> keyword, which is its unique syntax structure:</p>
    
    <pre><code>def add(a, b)
    return a + b
end</code></pre>

    <h3>1.3 Function Call</h3>
    
    <p>Sylph's function call directly uses the function name followed by arguments without parentheses, which is its unique syntax feature:</p>
    
    <pre><code>// Call a function and pass arguments
result = add 5 3
out result</code></pre>

    <h3>1.4 Multi-parameter Functions</h3>
    
    <p>Sylph's multi-parameter functions use commas to separate parameters in both definition and call:</p>
    
    <pre><code>def multiply(a, b, c)
    return a * b * c
end

// Call a multi-parameter function
result = multiply 2, 3, 4
out result</code></pre>

    <h3>1.5 Conditional Statement</h3>
    
    <p>Sylph's conditional statement starts with the <code>if</code> keyword and ends with the <code>end</code> keyword, without needing parentheses or braces:</p>
    
    <pre><code>if x &gt; 5
    out "x is greater than 5"
end</code></pre>

    <h3>1.6 Loop Statement</h3>
    
    <p>Sylph's loop statement starts with the <code>while</code> keyword and ends with the <code>end</code> keyword, without needing parentheses or braces:</p>
    
    <pre><code>while i &lt; 10
    out i
    i += 1
end</code></pre>

    <h3>1.7 Variable Declaration and Assignment</h3>
    
    <p>Sylph's variable declaration and assignment use the <code>=</code> operator without needing a keyword:</p>
    
    <pre><code>// Declare and assign a variable
x = 10

// Declare and assign a variable with specified type
y = 20i32</code></pre>

    <h3>1.8 Compound Assignment Operators</h3>
    
    <p>Sylph supports the following compound assignment operators:</p>
    
    <pre><code>// Addition assignment
x += 5

// Subtraction assignment
y -= 3

// Multiplication assignment
z *= 2

// Modulo assignment
a %= 4</code></pre>

    <h3>1.9 Integer Type System</h3>
    
    <p>Sylph supports multiple integer types specified by type suffixes, which is its unique syntax feature:</p>
    
    <table>
        <tr>
            <th>Type Suffix</th>
            <th>Example</th>
        </tr>
        <tr>
            <td><code>i8</code></td>
            <td><code>10i8</code></td>
        </tr>
        <tr>
            <td><code>i16</code></td>
            <td><code>1000i16</code></td>
        </tr>
        <tr>
            <td><code>i32</code></td>
            <td><code>100000i32</code></td>
        </tr>
        <tr>
            <td><code>i64</code></td>
            <td><code>1000000000i64</code></td>
        </tr>
        <tr>
            <td><code>i128</code></td>
            <td><code>1000000000000000000i128</code></td>
        </tr>
        <tr>
            <td><code>bigint</code></td>
            <td><code>1000000000000000000000000000000bigint</code></td>
        </tr>
    </table>

    <div class="note">
        <p>Sylph automatically selects the appropriate type for integers without explicit type suffixes based on the value's size, which is its unique type inference feature.</p>
    </div>

    <h3>1.10 Statement Block End</h3>
    
    <p>Sylph uses the <code>end</code> keyword as the end marker for statement blocks, which is its unique syntax feature:</p>
    
    <pre><code>// End of function definition
def add(a, b)
    return a + b
end

// End of conditional statement
if x &gt; 5
    out "x is greater than 5"
end

// End of loop statement
while i &lt; 10
    out i
    i += 1
end</code></pre>

    <h3>1.11 Comments</h3>
    
    <p>Sylph uses double slashes <code>//</code> for single-line comments:</p>
    
    <pre><code>// This is a comment
x = 10 // Inline comment</code></pre>

    <h2>2. Example Programs</h2>

    <h3>2.1 Simple Calculation</h3>
    
    <pre><code>// Calculate the sum of two numbers
def add(a, b)
    return a + b
end

// Main program
x = 5
y = 3
result = add x y
out result</code></pre>

    <h3>2.2 Loop Example</h3>
    
    <pre><code>// Calculate the sum from 1 to n
def sum_to_n(n)
    total = 0
    i = 1
    while i &lt;= n
        total += i
        i += 1
    end
    return total
end

// Call the function and output the result
result = sum_to_n 10
out result</code></pre>

    <h3>2.3 Conditional Statement Example</h3>
    
    <pre><code>// Check if a number is positive
def is_positive(n)
    if n &gt; 0
        out "Positive"
    else
        out "Non-positive"
    end
end

// Test the function
is_positive 5
is_positive -3</code></pre>

    <h2>3. Syntax Summary</h2>

    <div class="note">
        <p>Sylph language's core unique syntax features include:</p>
        <ul>
            <li>Using <code>out</code> keyword for output</li>
            <li>Using <code>def</code> keyword to define functions, ending with <code>end</code></li>
            <li>Function calls directly using function name followed by arguments without parentheses</li>
            <li>Using <code>if</code> keyword to start conditional statements, ending with <code>end</code></li>
            <li>Using <code>while</code> keyword to start loop statements, ending with <code>end</code></li>
            <li>Variable declaration and assignment using <code>=</code> operator without keywords</li>
            <li>Supporting multiple integer types specified by type suffixes</li>
            <li>Using <code>//</code> for single-line comments</li>
        </ul>
    </div>

</body>
</html>