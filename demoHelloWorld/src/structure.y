# 1) Es gibt kein "vererben vs instanzieren"!
# 2) Betonung auf "compile-time generics"!

Activity
    = Base: BaseActivityRoutine // interface bzw "semi-instanzierte Basisklasse"
    = Nav:  BaseNavigationRoute // weiterer verschachtelter Basis-Instanzverweis
    
    PublicAccessibleField = SomeType (??) // selbes wie in nächster Zeile "= SomeType" ???

    * OnCreate         // may be accessed/overridden
    * OnStart          // may be accessed/overridden
    + OnResume         // MUST be overridden/implemented
    + OnPause          // MUST be overridden/implemented
    * OnStop           // may be accessed/overridden
    * OnDestroy        // may be accessed/overridden
    - InnerNoOverwrite // can not be accessed from outside (private-ish)??

    Create  = Base.CreateMethod
    Destroy = Base.DestroyMethod

    SomeOtherMethod
        SOmeOtherSubThingie

    - InnerNoOverwrite
        HelperMethodCall
        + SomethingElse

    + SomeThirdMethod
        - YetAnotherSubThingie
            WithYetAnotherSubSub
                WithYetAnotherSubSubSub



MyActivity
    = Activity
    @ Start:
        
        Db.GetUsers(NewUserListReceived) # handler, implizit eindeutig genug ??? (TODO: Varianten prüfen!!)
        # auch möglich: "Db.GetUsers: @ Result:" aber wegen "=Result" Kurzform möglich
        
        # auch möglich lambda
        Db.GetUsers =>
            benutzerListe.Clear()
            benutzerListe.AddAll(.)
        # => Error // ??

    @ NewUserListReceived:
        benutzerListe.Clear()
        benutzerListe.AddAll(.: []Benutzer) # aktuellen (NewUserListReceived) Parameter übergeben, dieser muss Liste von Benutzern sein!
            
    Ein Text: lorem ipsum est ... # möglich irgendwie sinnvoll ohne Anführungsstriche? Befürchte eher nicht ...
    DURCH GROSSSCHREIBEN IST WERT KONSTANT: True


Db.GetUsers:
    = Result: []

# benutzerListe eigentlich ein Widget, aber veranschaulicht:
BenutzerListe
    @ Clear:  BenutzerListe = []
    @ AddAll: BenutzerListe += .




















Activity:
  - InnerNoOverwrite // can not be accessed from outside (private-ish)???????
  * OnCreate         // may be accessed/overridden
  * OnStart          // may be accessed/overridden
  + OnResume         // MUST be overridden/implemented
  + OnPause          // MUST be overridden/implemented
  * OnStop           // may be accessed/overridden
  * OnDestroy        // may be accessed/overridden

Activity:
  - InnerNoOverwrite:
        HelperMethodCall
        SomethingElse

    SomeOtherMethod:
        SOmeOtherSubThingie

  + SomeThirdMethod:
      - YetAnotherSubThingie