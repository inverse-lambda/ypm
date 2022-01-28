mycompoenent: drawing.something.mycomponent@'https://std@1.0.0/drawing'

// @ android: https://y.ide/compile/android@v1.0.0/
// android: "https://y.ide/generate/androidKotlin/" 
android: https://y.ide/gen/androidJava # mit oder ohne " davon abhängig wie auf Dateinamen zugegriffen + ob verschachtelte Komponenten (mehrere :) erlaubt sind

StartActivity:
  ## android.Activity
  
// in a 'Add.y' file, one might specify the "Add" function as Main
Main: Add

Add:
    > x
    > y
    < x + y

Add:
    // implicitly specifies >x and >y, if not known to scope
    x + y
    // implicitly specifies return of last statement

// If you need specific types being infered (e.g. for IDE auto complete), write tests.
Test:
    'Check two two digit numbers':      26 == Add 12 14
    'Check adding two zeros':           0  == Add  0  0
    'Check nonsense giving an error':   Err== Add "literal" 17.2f // imply "Add" can also give "Err"
    // Tests may infere specific types, which need to be handled!
    // Tests therefore also bear relevance towards compilability!

Main: 
    ::MultiType
    nums = 0.. .map{n: n*n}
        .takeWhile{squared: squared < 1000}
        .filter{squared: squared%2 == 1}
        .fold{0, (sum, squared: sum + squared) }
    < nums
    < 0...map (n: n*n)
        .takeWhile (squared: squared < 1000)
        .filter (squared: squared%2 == 1)
        .fold (0, (sum, squared: sum + squared))
    0...map | n: n*n
        takeWhile | x: x < 1000
        filter | squared: squared%2 == 1
        fold (0, |sum, squared|: sum + squared)
    0...map(n: n*n)
        .takeWhile(x: x < 1000)
        .filter(squared: squared%2 == 1)
        .foldInit(0)
        .fold(sum, squared: sum + squared)
    0..1000.map n:
        n * n
    .filter(...)

    (0..1000) n: 
        n%2==1 ? acc += n*n
        n >= 1000 ? < acc

    (0..1000) n: < 0xFF // ????

    (0..)
        .map (n*n)
        // wenn eindeutiger parameter geht auch direkt, zB nur ".map n" wäre ausreichend
        .takeWhile (squared < 1000)?
        .filter (squared%2 == 1)?
        .foldInit 0
        .fold (sum + squared, nochEinParamVonFold: 42) // implizite Closures? Je nach situation direkt berechnet oder anonyme Funktion?? (ohne param def??)
        .fold (0, sum,squared: sum + squared, nochEinParamVonFold: 42) // weglassen von Leerzeichen erlauben??? (statt zwingender Klammersetzung)
        .fold (0, (sum,squared): sum + squared, nochEinParamVonFold: 42) // weglassen von Leerzeichen erlauben??? (statt zwingender Klammersetzung)
        .fold (0, (sum,squared: sum + squared), nochEinParamVonFold: 42) // weglassen von Leerzeichen erlauben??? (statt zwingender Klammersetzung)
        
    (0..).map | n: n*n
        => map (n*n)
        - takeWhile (squared < 1000)
        - filter | 12, squared%2 == 1 && (otherFunction | withParams) && 'some else' // ???
        -> foldInit 0
        -> fold (sum + squared, nochEinParamVonFold: 42) // implizite Closures? Je nach situation direkt berechnet oder anonyme Funktion?? (ohne param def??)
        // Kann hald zu grauenvollen Mischungen führen (zB myMap.element->points), visuell dann noch dramatisch???
        // Oder Pfeil einfach (NUR) erforderlich wenn in nächster Zeile, auf vorherige Zeile bezogen!? (um nicht als eigene Instruktion zu gelten)
    'kompon 123 + 231 nentenname as is': 42

    // .. ggfs gut als Referenz auf oberste Ebene in einem File (ähnlich "this")

    filter otherfunc (x%2 == 1) -> fold // würde filter oder otherfunc gefolded?
    filter: x%2 == 1
    filter| x%2 == 1
    filter { x%2 == 1 }
    filter ( x%2 == 1 )

    sumOfSquaredOdds = // assign
    sumOfSquaredOdds : // set/compose type??

    someVals:
        green = "#00FF00"
        green: "#00FF00" // strike??? (nice to have just one assignment char? But ambiguity?)

    sumOfSquareOdds:
        x: (0..1000).where(n )

+ MemberVisibility:
    + Member1
    + Member2: Type2

< MemberVisibility:
    Member1
    Member2: Type2
    - HiddenMember1: Type1
    - HiddenMember2: Type2

// All members automatically visible, IF(!) static struct = no methods defined in it(??)
< MemberVisibility:
    Member1
    Member2: Type2

// No members (only return type equalities) visible otherwise; unknown members always count towards input params (???)
< MemberVisibility:
    Member1
    Member2: Type2
    < Member1 + Member2 + UnknownMember

/*
  MyAppNameLabel:
    android
    android.activities: ./activities

  # iwie nicht ganz so klar ("android" etwas versteckt ... syntax mehr highlighten?)
  MyAppNameLabel.android:
    activities: ./activities
*/