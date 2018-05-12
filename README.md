# RandomLoot
This is a program that extracts an arbitrary amount of turples randomly from a file(allowing repetitions).
It is designed to be used for generation of loot in pin and paper role play.

## File syntax
file: header \n elements\*

header: prop[ **:** header]

prop: **[** type **|** name **]**

element: val[ **:** element]

# Supported Types
bool: Boolean value allowing (y,n), (t,f), (yes,no), (true,false) and (1,0).

string: Text sequences of any size.

float: Decimal number.

int: Whole number.

coin: Formated whole number. Representing the value as the apropiate coins.

ufloat, uint, ucoin: Unsigned versions of the numeric types.

file|path: Boolean value representing whether the touple contains a member of the file.
