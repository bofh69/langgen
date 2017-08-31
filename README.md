# langgen
A crate to generate natural language from templates.

## Intro
Who really likes reading "The goblins runs north"?
Writing code to see if the goblins are in plural or not is not that hard,
but as you add visibility, hearing, etc the many cases tend to explode.
This crate is supposed to help with that.

This crate was inspired by DUMII, a MUD originally written by Christer Holgersson and Mikael Jakobsson. A lot of the examples are taken from DUMII's documentation.


## Example
Strings like this:
```
\The(me) give\s(me) \a(env) to \the(obj).
```
is used to generate messages like:
The old postman gives a stamp to Gandalf the gray.
You give a stamp to Gandalf the gray.
The small goblin gives a stamp to someone.
Someone gives a stamp to Gandalf the gray.

## Concepts
There are a few concepts involved.

### Named Objects
A named object is an object with four names and some flags;
* A singular short name (ie postman)
* A singular long name (ie old postman)
* A plural short name  (ie postmen)
* A plural long name (ie old postmen)
The names can be nouns and proper names.

### Viewers
A viewer can answer questions such as if it can see another viewer.

### Actors
An Actor is a Named Object and also a Viewer.

### Template Texts
A template text is a string like this:
```
\The(me) give\s(me) \a(env) to \the(obj).
```

"me", "env" and "obj" are refering to Actors.
"\The", "\s" and "\a" (among others) are directives

As can be infered, the text can contain directives that references
named objects.

### Output Handlers
An output handler is a Viewer and it also has methods to send
the generated texts to the users.

### Templates
A template is a collection of template texts and some description about
who should see each template text:
```
# Comments can be written here.
can_see_curses(me)
\The(me) shiver\s(me) as \he(me) see\s(me) \the__(obj) cursed \word(obj).
*
all
\The(me) look\s(me) at \the(obj).
```
The template object can take an output handler and a context refering to
the actors and render the context

An output handler, a context that points out the actors and a template
is used to generate the texts to the user.

## Features
* A named object can be created from a string like this:
  * "!Gandalf, !Gandalf the gray"
  * "orc, old orc"  
  * "louce, blue louce, lice, blue lice"
* A configurable system can create irregular plural names from singular names:
  * "\*fe" -> "\*ves" (ie knife becomes knives).
  * "\*man" -> "\*men" (ie woman, women).
* There is a macro system to make it easy to add styling:
  * "\The(me) say\s(me) ''\text{Hello}\``" can first be transformed into:
    "\The(me) say\s(me) ''\style(bold)Hello\style()\``" before it is used.

## Details

```
CODE     DESCRIPTION         RESULT                                      NO_THE
====     ===========         ======                                      ======
\the()   the-long            the beautiful wand(s)                       Hansoh the Dwarf/you
\the_()  the-short           the wand(s)                                 Hansoh/you
\The()   The-long            The beautiful wand(s)                       Hansoh the Dwarf/You
\The_()  The-short           The wand(s)                                 Hansoh/You
\thes()  the-Long            the beautiful wand's (wands')               Hansoh the Dwarf's/your
\the_s() the-Short           the wand's (wands')                         Hansoh's/your
\the_ss() the-Short          the wand's (wands')                         Hansoh's/yours
\Thes()  The-Long            The beautiful wand's (wands')               Hansoh the Dwarf's/Your
\The_s() The-Short           The wand's (wands')                         Hansoh's/Your
\a()     a-long              a/an/some beautiful wand(s)                 Hansoh the Dwarf/you
\a_()    a-short             a/an/some wand(s)                           Hansoh the Dwarf/you
\A()     A-long              A/An/Some beautiful wand(s)                 Hansoh the Dwarf/You
\A_(     A-short             A/An/Some wand(s)                           Hansoh the Dwarf/You
\my()    possessive-long     your/his/her/its/their beautiful wand(s)    Hansoh the Dwarf/you
\my_()   possessive-short    your/his/her/its/their wand(s)              Hansoh/you
\word()  word-long           beautiful wand(s)                           Hansoh the Dwarf/you
\word_() word-short          wand(s)                                     Hansoh/you
\Word_() Word-short          Wand(s)                                     Hansoh/You
\STR()   string              <the argument to STR()>
\s()     verb-end            <nothing>/s/es
\is()    is/are              " is "/" are "
\style(style) Adds style. It is up to the output system to make sense of it.


\num(num) number from num    42
\snum(num) string from num   nine
\str(text) string from text  prints text as a string


CODE             MALE    FEMALE  NEUTER  PLURAL      YOU         SOMETHING   SOMEONE
====             ====    ======  ======  ======      ===         =========   =======
\he()            he      she     it      they        you         it          he
\He()            He      She     It      They        You         It          He
\he_s()          he's.   she's.  it's.   they're.    you're.     it's.       he's
\He_s()          He's.   She's.  It's.   They're.    You're.     It's.       He's
\his()           his.    her.    its.    their.      your.       its.        his
\hiss()          his     hers    its     theirs      yours       its         his
\him()           him     her     it      them        you         it          him
\himself()       himself herself itself  themselves  yourself    itself      himself


```
