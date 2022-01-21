/**
 * Data component which holds some color references
 * Color: will mostly be gray
 */
Some Component:
    Color: 'gray'
    Speak: "My color is {:Color}!"
    Helptext: |You can add dynamic variables to text by using curly braces \{ and \}, as well as a colon :|
    PI: 3.14
    Infotext: |A constant like PI={:PI} can similarly be added, but will be baked into the string instead!|

    Weiterer Text in unlabled Array: //?
        |Text 1|
        "Noch ein Text in dem auch geschwungene Klammern { und } beliebig vorkommen dürfen."
        |||Und ein dritter Text mit PI mit dem Wert {:PI}. Welcher auch
        |||mehrzeilig sein kann aber jedes mal mit "Dreifach-Pipe-Zeichen"
        |||beginnen muss, also der Signatur ||| und auch damit wieder endet.||| // eher nicht (u.a. Kollision mit boolean "oder")?
        =|Ich bestehe aus den drei Texten "{:0}", "{:1}" und "{:2}"|
        asdf|Ein weiterer könnte mit uniquer Bezeichnung wie "asdf" beginnen, die im string dann aber
        nicht gefolgt von dem | Zeichen vorkommen darf und mit dieser Kombination auch endet.asdf|
        asdf"Genau so auch mit "kostantem Text" in dem auch die geschwungenen Klammern { und } buchstäblich genommen werden.asdf"
        ""Ein weiterer gültiger Bezeichner könnte der doppelte Anführungsstrich sein, dieser darf dann aber nicht im Text doppelt vorkommen!""


// Kategorien von Methoden (implizit ermittelt, nicht ausdrücklich festgelegt und durch Refactor mit viel Sideeffects auch gewollt)
// - Compile-Time fixed (nur Konstanten, statischer Text, etc.)
// - Access-Time fitted (beinhalten Variablen die andere Werte annehmen können)
// - Externally dependent ...