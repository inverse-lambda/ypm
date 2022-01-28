Activity: `https://inverse-lambda.com/android@1.0/activity` // das eigentlich lieber nicht hier => Linkage?

.
    = Activity
    = LayoutId: R.id.activity.main // LayoutId ist Label oder Typ? Oder Label ist immer ein inherenter Typ?

    @ OnCreate
        Activity.SetContentView(=) // SetContentView erwartet eine Art von LayoutId (oder dessen Supertyp)!
        Activity.SetContentView(.) // Punkt vs Strich hmmmm, = wäre etwas stärker visuell ersichtlich imho.

        Load User Info()

    @ Load User Info
        ... // evtl Platzhalter funktional ähnlicher zu Python `pass`

-----------------------------------------------------------------------------------------------------------

InventurActivity
    = Activity
