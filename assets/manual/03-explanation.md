# how the cypher works

this is the part where i explain how this sausage gets made. why? sometimes it's fun to learn about these kinds of things :]

## what is this "bean"
essentially, it's a fancy way to display a number. more specifically, any value between 0 and 215.

effectively, each version of the letter has a "value":
- lowercase = 0
- uppercase = 1
- 1337 5p33k = 2
- absent = 3

then you use this as an elaborate counting system. since the last letter can be any value from 0 to 3, it brings 4 unique values. therefore the 2nd to last letter will get multiplied by 4. hope that made sense?

to not make you do maths for no reason, here are the (correct) multiplication values:
- b * 72
- e * 24
- a * 8
- n * 4
- s * 1

sum all those together and you get the integer value of the "bean"!

note: this cypher will become greater the moment someone invents a new number that looks like N.

## values the cypher accepts
now, again, you can define the characters behind the IDs yourself! but by default, the cypher accepts ASCII (for the most part), with some additional latvian (🇱🇻) letters. no bias here at all, no sir. the full list you can check out [in the source code,](https://github.com/Kortimu/bean-cypher/blob/main/src/hash_convert.rs) if you care

and yes, this does mean you can freely interpret the cyphered characters however you wish. characters, phrases, sentences, heck - even entire bee movie scripts *should* be supported! (i think?)
