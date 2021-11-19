# y?
Inverse lambda's build system, transpiler and package manager

## Reserved Symbols

Although there are no reserved words, several characters are not permitted (e.g. as part of a component name or label).

- ASCII control characters, 0x00 to 0x1F (0 to 31) are invalid,
- as well as the following nine printable characters: / \ : * ? " < > |

Furthermore, even though not explicitly forbidden, it is highly recommended to not end component names in a space or dot, nor use the following as names for components or labels (as they could lead to problems under Windows file systems):
- CON, PRN, AUX, NUL,
- COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9,
- LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, LPT9

## Primitive Constructs

Primitive constructs like quoted strings or plain numbers don't necessarily correlate to a specific primitive type. The type must still be inferred, derived or indicated.

### String Literals
Dual quoting is not supported in a common form. String literals can be specified 
- between double quotes (e.g. `"my string"`), <!--- -formatting is baked into the language (e.g. `"my {}" "string"`)) -->
- between three double quotes (and an optional single-line delimiter)
#### Examples
    "string literal"

    """any delimiter
    String with double quotes (") and tripple double quotes (""") via any delimiter.
    """any delimiter

    """
    However, without a specific delimiter "tripple double quotes" are not allowed.
    """

### Numbers
- 42 (decimal)
- 0xFF (hexadecimal)
- etc.

## Package/Component/Parameter Visibility
### The Backstory
A general consideration was that components and labels starting with a small letter are hidden, while those starting with a capital letter are publically (e.g. at least package wide) accessible (similar to Go). Range of visibility for hidden files and labels could extend top down, therefore, a lowercase subfolder (and its files/componets) would be visible from any parent folder.

However, while this offers a clear binary system for visibility, it is not the most intuitive, nor is it the most obvious (esp. to untrained programmers). Therefore, the best way might be to abstract visibility in an intuitive way, which neither needs explaining nor hinders the "development flow".
### The Task
The concept of visibility offers several properties (vastly simplified):
- Separation of concerns
- Safety against improper reads/modification (esp. on multi-threading)
- Clarity, by only showing relevant fields for the context
- Forcing developers to deal with context appropriate data

Can these also be accomplished, without directly specifying visibility modifiers?
### The Idea
One basic concept to replace (explicit) visibility is not new at all, namely, exporting. However, when used exactly as e.g. in JavaScript, the problem is obviously that this concept typically only deals with package level visibility. On the other hand, making everything visible would open a Pandora's box of catastrophies, or would it?
- Strong composition (few elements per component, but many layers) could also lead to clarity, separation of concerns and forcing the developer to deal with context appropriate data (in a "soft" way)
- Safety should be dealt with either way, even at a compile-time, and is generically targeted later on
- (Default) immutability over visibility/hiding, similar to Rust (requiring ``mut'' for mutability)

A main point being, that the compiler can analyse parameter use and generate e.g. getters/setters as needed. Letting the nearness of parameters (and the otherwise required nesting complexity) serve as replacement for manually specifying visibility.