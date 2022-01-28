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

        Wert: |Weitere Überlegung für mehrzeiligen Text:
              |
              |   Dieser könnte zB dadurch dass er nicht in selbiger Zeile geendet hat,
              |   implizit erwarten dass er mehrzeilig ist und nicht direkt determiniert wird,
              |   sondern bis zum letzten Vorkommen der Zeile mit Pipe-Symbol (\|) weitergehen.
              |   Dies wäre möglich, weil Daten nicht in sich ohne Labels geführt werden, bzw.
              |   bei Aufzählung in einer Liste müsste ein Sonderzeichen (-) verwendet werden!

        Mehrere Werte
            - |Text der einfach weiter geht, muss dieser eigentlich terminiert werden?
            - |Ich finde nicht, eigentlich ist auch so klar, dass ein neuer Punkt hier
              |begonnen wurde, der sich auch über mehrere Zeilen ziehen kann ... hmmmm
            - |Wäre eine terminierung in diesem Fall ggfs optional dennoch möglich?|
            - |Oder wäre eine solche eher verwirrend? Weiterarbeit in der Zeile wäre möglich!
            - |Somit könnten auch Parameter wie {:0} als suffixe übergeben werden|{"Dieser"} // <- nicht so klar imho

        Anderes Zeichen für mehrere Werte evtl
            [] 12.3
            [] |Text|
            [] |Mehrzeiliger Text
               |evtl etwas unklarer als der Bindestrich auch wenn zugehörig bzw assoziierbar mit []
            [] "Unklar ob gut so" // Mischformen evtl auch schlecht (also | und " erlaubt), vielleicht " ohne Parameter
            [] "Oder {:0} in Anführungsstrichtexten Parameter erlauben"{:"nur"} // <- milder Brechreiz / eher nicht
            [] |Vielleicht besser einfach keine unmittelbar nachgestellten Parameter? Wo notwendig? Nur {:Platzhalter}!|
            [] Mehrdimensional auch möglich
                [] |Eins
                [] |Zwei
                [] Variable
                [] |Vier
                   |also innherhalb der Unterkategorie zumindest, somit quasi [6][3] oder [+6][+3] oder [*7][*4]




// Kategorien von Methoden (implizit ermittelt, nicht ausdrücklich festgelegt und durch Refactor mit viel Sideeffects auch gewollt)
// - Compile-Time fixed (nur Konstanten, statischer Text, etc.)
// - Access-Time fitted (beinhalten Variablen die andere Werte annehmen können)
// - Externally dependent ...