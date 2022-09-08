# Pincer

Generate permutations and combinations of sets of elements.

Inspired by [crunch](https://sourceforge.net/projects/crunch-wordlist/).

## Usage:

Multiple functions are provided with this Rust library.

Each function has built-in documentation when hovered.

The `from_pattern()` function is the most complicated, using a set of symbols to iterate letters, numbers and symbols.
```
, for all uppercase letters
@ for all lowercase letters
% for all numeric characters
^ for all special characters
```
Here are the characters represented, respectively:
```
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
a b c d e f g h i j k l m n o p q r s t u v w x y z
0 1 2 3 4 5 6 7 8 9
  ! " # $ % ( ) * + , ' - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~  
```
An example function input and output:
```
for s in from_pattern(vec!['a', '%', '@'], None) {
    println!("{}", s);
}
// Output: a0a, a0b, a0c ... a1a, a1b, a1c ... a9x, a9y, a9z
```

### Notes

For planned features, to suggest a feature or to report a bug, please see the [Github issues tab](https://github.com/Sheumais/pincer/issues).
Pull requests with new features are also welcome!

Optimisations to the functions will come in future. For now, they get the job done.