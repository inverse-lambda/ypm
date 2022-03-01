// in class.test.y

@ Empty package name should not work:
    class: ..
        Package:
        Imports: [some, utils]
        Visibility: public
        Abstract:
        Name: ExampleClass
        Extends:
        Implements:

some, utils, public, ExampleClass: str.itself