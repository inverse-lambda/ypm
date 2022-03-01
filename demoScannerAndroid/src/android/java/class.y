--@|: """{}\ // not recommended/allowed??!
--@>: """{}+2

Class@'''
    {Package}
    {Imports}
    {Visibility}{Abstract}class {Name}{Extends}{Implements}{{
        {Body}
    }}'''


---------------------------------------------------------------

Class.Package
    => Package Name
    <= "package {Package Name};\n\n"
Class.Package: <= "package {=};\n\n" // :+1:
Class.Package: <= "package {=>};\n\n" // ???



Class.Package@""": "package {=};\n\n"
Class.Package@str: "package {=};\n\n"
Class.Package@"""
    package {=};\n\n

Class.Package@"""[]\
    package [=];\n\n

Class.Imports 
    => Imports: []
    line: "import {single import:};\n"
    // <= "{*Imports{import {*}; }*}" 
    // <= |{import {=.each()};\n} //?
    // ??

---------------------------------------------------------------
# Basic Usage Example
---------------------------------------------------------------
Activity:
    = Class: java.Class
    Class.Imports: |androidx.appcompat.app.AppCompatActivity;
    Class.Package: 'androidx.appcompat.app;' // not directly this id!

Activity:
    = Class: java.Class
        Imports: |androidx.appcompat.app.AppCompatActivity;
        Package: 'androidx.appcompat.app;' // not directly this id!

Activity:
    = Class: java.Class
Activity.Class:
    Imports: |androidx.appcompat.app.AppCompatActivity;
    Package: 'androidx.appcompat.app;' // not directly this id!

Activity:
    = Class: java.Class
    Class:
        Imports: |androidx.appcompat.app.AppCompatActivity;
        Package: 'androidx.appcompat.app;' // not directly this id!

= Class: |{Package}
         |{Imports}
         |{Visibility}{Abstract}class {Name}{Extends}{Implements}{{
         |    {Body}
         |}}
    @str: "{=}\n";