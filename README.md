# Fort
its like forth or something

# Why did you make this?
I'm messing with programming languages and a forth type thing is fun and easy to make.
Also I'm working on something kinda like [sapf](https://notes.billmill.org/link_blog/2025/01/sapf_-_sound_as_pure_form.html)
so having a forth-ish language would be good for that.

Right now this is just in the "for fun" stage, it's not particularly useful, it's just me trying to figure out the best
ways to arrange the kinda stuff you need to make a system like this. It will probably evolved to do different things in different
ways as I work with it a bit.

# how do I build it?
Use cargo to build things. Right now this builds as a library, so to run stuff you will need to run the example
```
cargo run --example repl
```
this will drop you into a no frills repl to play around (though there isn't much to play with yet)

# how about the language itsef?
*WARNING* this stuff changes quickly so this likely won't be up to date but I'll try to make sure it's generally right

so it's a "concatenative / stack-based / forth-like" language, if you don't know much about those they look weird at first
I think these kinds of langs are cool and fun: [this will be a post or something explaining why at some point](https://en.wikipedia.org/wiki/Forth_(programming_language))
and I think they help teach a lot of concepts in a simple way.

The language is super simple right now. There are only a few immutable types, and very basic scoping and stuff like that.

you have 4 types, ints (i64) floats (f64) lists (heterogenous arrays) and functions (first class).

syntax wise, numbers are done with normal looking literals, i.e. 1,-4,1805 for ints. 3.2, 66.99, 0.0, -5.4 for floats.
lists are in square brackets like [1 3 [4 5] 6 6]

functions are a bit weird compared to classic forth, they are "first-class" meaning they can be put on the stack and
used as arguments or returned from other functions, things like that. they can also be anonymous, or named, either way
is fine. 

They are defined with parens like: (1 +) would make an anonymous function that adds one to whatever is below
it. To name them, put ::<name> after the definiton like so: (1 +)::inc. to call them, just type the word they define.
so 1 inc would give you 2 back, to refer to them without calling put a ` in front of the word, like `inc this will put the
function value on the stack.

function values on the stack can be called with the built in function "!" so 1 (1 +) !, would be the same as 1 inc.

```
( (1 +)::inc inc map)::bump_list
```
would define a function called "bump_list" in the global scope, but not a function called inc.
but you can mess around with that using symbols. Symbols are made by putting ' in front of a word. Like 'inc.
when you use the ! function with a symbol it looks up that word in the current scope and calls that. This means you can
alter the behavior of a function after the fact and do weird meta stuff. For example:
```
(1 +)::inc
(('inc !) map)::bump_list

[1 2 3 4 5] bump_list
[1 2 3 4 5] ( (4 +)::inc bump_list) ) !
```

will end you up with [2 3 4 5 6] and [5 6 7 8 9] on the stack

# what are the long term goals?
Mostly to have fun! The computer is for fun! Serious business? BOO!
The goal right now is to make this like a toolkit for embedding into other programs to give them an interactive component
like lua but it's a weird home rolled language.

# what's there to work on in the short term?
oh boy lots of stuff:

presently I'm working on simplifying the core stack machine down into parts that can be selectively added on and changed
so that it's easier to make and try out new language features
