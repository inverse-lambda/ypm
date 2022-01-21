# Es gibt kein "vererben vs instanzieren"!

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
    @ 





















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