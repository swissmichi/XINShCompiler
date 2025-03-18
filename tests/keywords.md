# Datatypes

- T - Generic Datatype
- num - Integer
    - 3, 2
- float - Float
    - 2.21, 33.2
- text - Text
    - "Hello", "h"
- bool - Boolean
    - True, False, [ $x == 2 ]
- list:T - List with type T
    - (4, 3, 5), ("Hello", "Hi")
- param:V - Parameter with Variables V
    - [[x, y, z]], [length, height]
- range - Range
    - {1, 4}, {3, 7, 2}
- func - Function
    - function bar { echo "bar" }

# Variable syntax stuff

- Creating Variables of any datatype except func:
```
x=1
x:num=1
hello:text
hello="Hello"
```

- Declaring functions:
```
function bar [[a:num, b:num]] [[return:num]] {
    return $a+$b
}
```

- Calling Variables
```
$x
$y
$z
```

- Calling functions:
```
# No param
$foo

# no return 
$bar[[1]]

# Return

x=$foobar
x=$baz[[1]]
```

# I/O

- Output

```
echo "Hello"
echo $x
```

- Input 
```
read name
$name
```

# Converting datatypes

```
num2text $n
tex2bool $n
```
Cannot be done with funcs and params

# Control flow

```
if [$num = 2]; then
    echo "yup"
else if [$num = 3]; then
    echo "yep"
else
    echo "no"
fi
```

# While and For loops

```
while [$x < 5]; do
    echo $x
done

for i in {1,6,2}; do
    echo $i
done

for word in ("Hi", "Hi", "Hello"); do
    echo $word
done
```

# File handling

- checking for file and directory existance
```
dirExists = [dir "hello"]
fileExists = [file "hi.txt"]
```

- Reading and writing to files 
```
cat "hello.txt"
cat "i append text :3"  "hello.txt" False
cat "And i overwrite :3" "hello.txt" True
```

Copying and renaming files
```
cp "file.txt" "heyo.txt"
mv "heyo.txt" "nom.txt"
```

# Command calls

```
command "ls -lstra"
```

# Keywords

- num
- float
- text
- bool
- list
- range
- func
- param
- return
- echo
- read
- num2float
- num2text
- num2bool
- num2list
- num2range
- float2num
- float2text
- float2list
- text2num
- text2float
- text2bool
- text2list
- bool2num
- bool2text
- bool2list
- list2range
- range2list
- if
- then
- else
- for
- while
- do
- done
- in
- dir
- file
- cat
- cp
- mv
- False
- True
- command
