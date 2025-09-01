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
just use cargo, there isn't anything special or interesting yet.

# how does it work?
for now it just runs a hardcoded string in the main function. It's still new

# how about the language itsef?
so it's a "concatenative / stack-based / forth-like" language, if you don't know much about those they look weird at first
I think these kinds of langs are cool and fun: [this will be a post or something explaining why at some point](https://en.wikipedia.org/wiki/Forth_(programming_language))
and I think they help teach a lot of concepts in a simple way.

The language is super simple right now. There are only a few immutable types, and very basic scoping and stuff like that.

you have 4 types, ints (i64) floats (f64) lists (heterogenous arrays) and functions (first class).

syntax wise, numbers are done with normal looking literals, i.e. 1,-4,1805 for ints. 3.2, 66.99, 0.0, -5.4 for floats.
lists are in square brackets like [1 3 [4 5] 6 6]

functions are a bit weird compared to classic forth, they are "first-class" meaning they can be put on the stack and
used as arguments or returned from other functions, things like that. they can also be anonymous, or named, either way
is fine. They are defined with parens like: (1 +) would make an anonymous function that adds one to whatever is below
it. To name them, put ::<name> after the definiton like so: (1 +)::inc. to call them, just type the word they define.
so 1 inc would give you 2 back, to refer to them without calling put a ` in front of the word, like `inc. functions
on the stack can be called with the built in function "call" so 1 (1 +) call, would be the same as 1 inc.

functions have scopes, meaning you can define stuff inside a function, and then that definition won't carry outside. This isn't
that useful right now, as there isn't a great way to refer to the args inside a function definition.

# what are the long term goals?
Mostly to have fun! The computer is for fun! Serious business? BOO!
But I think I'd like to make this into a sorta toolkit, for embedding into other projects, just to have a simple way of making
interactive stuff.

# what's there to work on in the short term?
oh boy lots of stuff:

right now the regular stack printing can't really handle function names, the BIFs don't store their names in an accessable way
and neither do defs, also it's probably good to come up with a better way to print anonymous functions.

it would also be good to be able to get help text and stuff like that for functions.

I'd like to come up with some good utilites for extracting args for BIFs, because BIFs just get the whole stack to mess with
however they like, they can do screwy things, like pop off a bunch of args and then fail without returning those args.
Since in the long term I want to make this embeddable users are gonna need to be able to add new BIFs and things like that.

Because of all the messing around with BIF stuff the error handling is kind of a mess, I need to clean it up
