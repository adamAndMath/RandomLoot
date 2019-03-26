# RandomLoot
This is a program that extracts a rondom sampling of rows from a file (allowing repetitions).
It is designed to be used for generation of loot in pin and paper role play.

## File syntax
file: header \n elements\*

header = (name **:** type)**;**+

element = (val **;**)+ int

type = **str**
    | **bool** (**[** name **,** name **]**)?
    | **int** (**[** name | (name **=** int ) **]**)?
    | **float** (**[** name | (name **=** float ) **]**)?

# Supported Types
bool: Boolean value allowing (default: true, false).

string: Text sequences of any size.

float: Decimal number.

int: Whole number.
