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
        ? Another Boolean ?
            Also do Something()
  
        User Logged In
            = (App.UserInfo != ∅)

    @ Load User Info
        ... // evtl Platzhalter funktional ähnlicher zu Python `pass`

-----------------------------------------------------------------------------------------------------------

InventurActivity
    = Activity
