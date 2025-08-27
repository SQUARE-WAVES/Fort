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

# how does it work.
for now it just runs a hardcoded string in the main function. It's still new

# how about the language itsef.
so it's a "concatenative / stack-based / forth-like" language, if you don't know much about those they look weird at first
here is a little post about why these kinds of langs are cool and fun [insert real post here](https://en.wikipedia.org/wiki/Forth_(programming_language))

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

# what's there to work on?
oh boy lots of stuff, right now I need to flesh out the built-in-functions and maybe mess with the syntax a bit.
I want to add lazy lists and figure out how to handle closures and higher-order functions. 

In the long run I'm probably mostly gonna look at making this sorta like a "forth toolkit" where it's less concerned with being a heavily featured language
on its own, but rather a thing that's easy to grab and hack a little to embed in a project and add a repl. Kinda like lua. I also might
play with the data structures to make it more embedded friendly. I'm not really sure.

right now, for example the dictionary is just a vector of hashmaps of strings. Meaning every time you define a function there is a bunch of memory allocation,
as well, if I mess up how scopes get pushed and popped it's possible to just drop the root scope.

Also, when you make a list or function, the system just pushes values onto the regular stack until you get to the end, then it copies that stuff into a new vector
and saves a refcounted pointer to that data. It feels like a suboptimal way to go about it.
