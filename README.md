# langgen [![Build Status](https://travis-ci.org/bofh69/langgen.svg?branch=master)](https://travis-ci.org/bofh69/langgen) [![Build status](https://ci.appveyor.com/api/projects/status/6llxcykk8gqj1eig?svg=true)](https://ci.appveyor.com/project/bofh69/langgen) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
A crate to generate natural language from templates.

## Intro
Who really likes reading "The goblins runs north"?
Writing code to see if the goblins are in plural or not is not that hard,
but as you add visibility, hearing, etc the many cases tend to explode.
This crate is supposed to help with that.

This crate was inspired by DUMII, a MUD originally written by Christer Holgersson and Mikael Jakobsson. A lot of the examples are taken from DUMII's documentation.


## Example

This crate has both an API and a template system for generating strings.
The API can be used like this:
```
output.out().The(me).v_e("give").\a_(obj).s("to").the_(env);
```
Traits are used to make this easy to integrate with existing code.


Strings like this:
```
\The(me) give\s(me) \a(obj) to \the_(env).
```
is used to generate messages like:
* The elf gives a stamp to Gandalf the gray.
* You give an apple to the old elf.
* The small goblin gives a stamp to someone.
* Someone gives something to the green rabbits

## Concepts
There are a few concepts involved.

### Named Objects
A named object is an object with four names and some flags;
* A singular short name (postman)
* A singular long name (old postman)
* A plural short name  (postmen)
* A plural long name (old postmen)
The names can be nouns and proper names.

### Viewers
A viewer is something that can observe and interact with objects.

### Objects
An Object is a Named Object and also a Viewer.

### Template Texts
A template text is a string like this:
```
\The(me) give\s(me) \a(env) to \the(obj).
```
"me", "env" and "obj" are refering to Objects.
"\The", "\s" and "\a" (among others) are codes that are transformed into
proper English for the refered Objects.

### Output Handlers
An output handler is a Viewer and it has methods to send
the generated texts to the user/users.

### Templates
A template is a collection of template texts and some description about
who should see each template text:
```
# Comments can be written here.
can_see_curses(viewer), can_see_curses(me)
\The(me) shiver\s(me) as \he(me) see\s(me) \the__(obj) cursed \word(obj).
*
can_see_curses(me)
\The(me) shiver\s(me) as \he(me) see\s(me) \the(obj).
*
all
\The(me) look\s(me) at \the(obj).
```
The template object can take an output handler and a context refering to
the objects and render the context with the template.

## Features
* A NamedFactory can create Named objects from a string like this:
  * "!Gandalf, !Gandalf the gray"
  * "orc, old orc"  
  * "louce, blue louce, lice, blue lice"
* Configurable rules to create irregular plural names from singular names:
  * "\*f" -> "\*ves" (for making elf become elves).
  * "\*fe" -> "\*ves" (for making knife become knives).
  * "\*man" -> "\*men" (for making woman become women).
* There is a macro system to make it easy to add styling:
  * "\The(me) say\s(me) \quot{Hello}" will first be transformed into:
    "\The(me) say\s(me) ''\style(bold)Hello\style()\``" before it is used.

## Details

```
CODE     DESCRIPTION         RESULT                                      NO_THE
====     ===========         ======                                      ======
\the_()  the-long            the beautiful wand(s)                       Hansoh the Dwarf/you
\the()   the-short           the wand(s)                                 Hansoh/you
\the__() the                 the/<nothing>
\thes_() the-Long            the beautiful wand's (wands')               Hansoh the Dwarf's/your
\thes()  the-Short           the wand's (wands')                         Hansoh's/your
\thess_() the-Long           the beautiful wand's (wands')               Hansoh the Dwarf's/yours
\thess() the-Short           the wand's (wands')                         Hansoh's/yours
\a_()    a-long              a/an/some beautiful wand(s)                 Hansoh the Dwarf/you
\a()     a-short             a/an/some wand(s)                           Hansoh the Dwarf/you
\a__()   a                   a/an/<nothing>
\my_()   possessive-long     your/his/her/its/their beautiful wand(s)    Hansoh the Dwarf/you
\my()    possessive-short    your/his/her/its/their wand(s)              Hansoh/you
\word_() word-long           beautiful wand(s)                           Hansoh the Dwarf/you
\word()  word-short          wand(s)                                     Hansoh/you
\plural_() pl.word-long      beautiful wands                             Hell's Dwarves/you
\plural()  pl.word-short     wands                                       Borg/you
\s()     string              <the argument to s()>
\v_e()   verb-ending         <nothing>/s/es
\is()    is/are              " is "/" are "
\style(style) Adds style. It is up to the output system to make sense of it.


\num(num)  number from num   42
\snum(num) string from num   nine
\str(text) string from text  A string

The leading case of the code determines the case of the word.


CODE             MALE    FEMALE  NEUTER  PLURAL      YOU         SOMETHING   SOMEONE
====             ====    ======  ======  ======      ===         =========   =======
\he()            he      she     it      they        you         it          he
\he_s()          he's    she's   it's    they're     you're      it's        he's
\his()           his     her     its     their       your        its         his
\hiss()          his     hers    its     theirs      yours       its         his
\him()           him     her     it      them        you         it          him
\himself()       himself herself itself  themselves  yourself    itself      himself

```
