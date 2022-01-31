Activity: `https://inverse-lambda.com/android@1.0/activity` // das eigentlich lieber nicht hier => Linkage?

.
    = Activity
    = LayoutId: R.id.activity.main // LayoutId ist Label oder Typ? Oder Label ist immer ein inherenter Typ?

    @ OnCreate
        Activity.SetContentView(=) // SetContentView erwartet eine Art von LayoutId (oder dessen Supertyp)!
        Activity.SetContentView(.) // Punkt vs Strich hmmmm, = wäre etwas stärker visuell ersichtlich imho.
        ? User Logged In
            Get a Val: Do Something()
            Load User Info()
        ~ User Logged In         // Kein explizites ELSE vorhanden (Anti-Pattern-ish?)
            Show Alt Text For Not Logged In User ()

        // Alternatives?
        ¿ Question ?
            Do Some()
        ~ Negation ?
            Do Some Else()
        ? Another Boolean ?     // evtl automatisch via IDE zur spanischen Notation umwandeln?
            Also do Something()
        = Yet Another ?
            Also do Some Else()
        ? (App.UserInfo != ∅)   // auch direkt möglich mit Klammern die "Evaluierung" andeuten

        User Logged In
            = (App.UserInfo != ∅) // evtl als alternative zwei Nullen (00) um ohne IDE zu können bzw null überhaupt sinnvoll??
                                  // stattdessen könnte "boolean negativ" und "null" ident sein, somit simpler "nicht-check"??
            ~ App.UserInfo        // <- evtl zu stark vereinfacht? 
                                  // (hier nur als Demo, für diesen Wert "falsch" => da nicht doppelt verneint)

        ~ App.UserInfo
            Do some when UserInfo is not set()

    @ Load User Info
        ... // evtl Platzhalter funktional ähnlicher zu Python `pass`

-----------------------------------------------------------------------------------------------------------

InventurActivity
    = Activity
