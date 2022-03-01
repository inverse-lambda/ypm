
Class:
    ǁ ⟪Package⟫
    ǁ ⟪Imports⟫
    ǁ ⟪Visibility⟫⟪abstract option⟫class ⟪Class Name⟫⟪Extends⟫⟪Implements⟫{
    ǁ ⟪Class Body⟫
    ǁ }

Class: 
       |{Package}
       |{Imports}
       |{Visibility}{Abstract}class {Name}{Extends}{Implements}{{
       |    {Body}\n
       |}}
------------------------------------------------

《class_visibility》《abstract》class《ActivityName》extends AppCompatActivity《implements Something》{

    《@Override protected void onResume() { super.onResume();《additional instructions》}》

}

class_visibility class_modifier《class》activity_name《{》 // nicht wirklich so

Class: // raw, only params and param-escapes: {{, }} - which allows almost 1:1 representation of what to print!
    |{Package}
    |{Imports}
    |{Visibility}{Abstract}class {Name}{Extends}{Implements}{{
    |    {Body}
    |}}

Something without params, but cant start with space: """{}\
    asdf } these brackets mean nothing
    really noooo thing {{ { }}}}}} ...


Continue with another parameter: """{}
      {Package}
    {Imports}
    {Visibility}{Abstract}class {Name}{Extends}{Implements}{{
        {Body} \n
    }}

// CONSIDER: What to do with newlines at the end(/beginning) of the string literal? => add one \n, if more needed add \x20 or other whitespace on separate lines, if less??
Continue with another parameter: """{optional param encoder component}\
    \x20 {Package}
    {Imports:encode info list} //??
    {Visibility}{Abstract}class {Name}{Extends}{Implements}{{
        {Body} \\n
    }}

Class: 
    |《Package》
    |《Imports》
    |《Visibility》{abstract option:}class{class name:}{extends:}{implements:} \{
    |
        
Visibility
    ⊃ |        |package private
    ⊃ |public  |visible to all packages and levels
    ⊃ |private |non visible

Visibility
    = String
    ⊃ 
    ⊃ public 
    ⊃ private

//Visibility ⊃ public, private
//Visibility: {"public", "private", ""}

Class: ǁ《Package》
       ǁ《Imports》
       ǁ《Visibility》《abstract option》class 《class name》《extends》《implements》{
       ǁ《Class Body》
       ǁ}

Class:
    """{}
    {Package}
    {Imports}
    {Visibility}{Abstract}class {Name}{Extends}{Implements}{{
        {Body}
    }}

Class: """｟｠
    ｟Package｠
    ｟Imports｠
    ｟Visibility｠｟abstract option｠class ｟class name｠｟extends｠｟implements｠{
    ｟Class Body｠
    }

Class: """⟪⟫\
    ⟪Package⟫
    ⟪Imports⟫
    ⟪Visibility⟫⟪abstract option⟫class ⟪class name⟫⟪extends⟫⟪implements⟫{
    ⟪Class Body⟫
    }

Class: """❰❱ // starting with empty space; comment here allowed?
    |❰❱    ❰Package❱
    |❰Imports❱
    |❰Visibility❱❰abstract option❱class ❰class name❱❰extends❱❰implements❱{
    |❰Class Body❱
    |}

Class: """<>\
    <Package>
    <Imports> mit bisschen extra sichtbaren Spaces          \
    <Visibility><abstract option>class <class name><extends><implements>{
    <Class Body> \< echte escapete eckige \> Klammern
    }

--------------------------------------------------------------------------------
Considerations:
    - Raw vs Backspace Escapes (by default/switchable?)
    - Starting with spaces, when tab encoded
    - Intro-sequence (""") vs individual line prefix (|) !!!!!
    - Format-values / variables (by default/switchable?)

It should be possible:
    - Single line strings
    - Multiline raw strings (indented and non-indented)
    - Multiline parameterized strings (mainly indented)

Proposal:
    - One startsequence for raw block, with optional format delimiters, and optional escaping (""", """<>, """\, """<>\)
      (starting with spaces/tabs not possible for """ and """\ ?????)

--------------------------------------------------------------------------------

Activity:
    = java.Class
    java.Class.Imports.Add(|androidx.appcompat.app.AppCompatActivity
    java.Class.Imports: Required Imports
    .Imports: Required Imports

Required Imports:
    - android.content.Context
    - androidx.appcompat.app.AppCompatActivity
    