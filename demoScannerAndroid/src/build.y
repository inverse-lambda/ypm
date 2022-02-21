< Activity: https://inv-lam.com/android/activity
< android: https://inv-lam.com/android/
< Activity: android.Activity // möglich weil "alles" lazy initialisiert, somit hier nur referenzen festgehalten und erst aufgelöst wenn später gebraucht
< NavDrawer = android.NavDrawer // eher nicht mit =, aber klären was/wie mit veränderten Referenzen

= `https://y.dev/tools`

<= SomeVariable     // Variable wird bei Änderung gepublished
<= SomeOtherVar: 52 // Variable wird bei Änderung und initial mit 52 gepublished
<= YetAnother:  android.resource.id // YetAnother ist vom Typ resource.id --- nicht initial gepublished, nur compile-time verifiziert
<= YetAnother2: android.resource.id(52)
<= YetAnother2: android.resource.id = 52
<= YetAnother2: 52 : android.resource.id


source: |../src
target: |C:\Temp\bin

copy(from: source, to: target)
copy() // <- immediately execute, ohne @, direkter aufruf mit (), klar genug??
    from: source
    to:   target
    

? source == target:
    copy(some, else)
    