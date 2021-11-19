###################################################################################

generator: https://ilambda.cc/rust/androidGenerator.y

MyApp:
  generator(CURRENT_PATH)
  generator.Path: "./intermediate"

###################################################################################

# renderer: "https://yyy.cc/std@1.0.0/renderer.y"
renderer: -

activityRenderer: 
  => Activity 
  renderer
  #Activity: activity # unbekannter referenzierbarer Typ (activity wird explizit als Platzhaltertyp herangezogen)
  #Activity: act

  !renderActivity

  !setPackage: renderer.AppendBlock('package ' + Activity.Package + ';') // adds 2 Linebreaks at the end
  !setImports: [renderer.AppendLine('import ' + import + ';' <= import: Activity.Imports)]
  !setImports: [renderImport(.) <= Activity.Imports]
  !setImports: renderImports(Activity.Imports)
  !renderImports(Activity.Imports)
  !setClass:

  renderImport:
    >Import
    !renderer.AppendLine('import ', Import, ';')

  renderImports:
    >Imports
    [renderer.AppendLine('import ' + .NameOfSingleImport + ';' <= Imports)]

  Activity # noch unbekannter Typ
  renderer # oben importierter Typ


activityRenderer.setClass:


:: Dieses ...
// oder so iwie ...
:ActivityRenderer:
  Activities: []
  Activities: [activity]
  
  # renderers: render(Activities)      // 1 render component mit allen Activities als Parameter ...
  renderers: [activityRenderer(a) <= a: Activities] // Das Kleiner-Gleich-Zeichen bezieht für alle Activities je einen renderer (ähnlich "[activityRenderer() for a in Activities]" in Python) ((das label "a:" könnte in diesem Fall auch weggelassen werden, veranschaulicht aber die grundsätzliche Syntax))

  # render.Content: Activities


# Restriktionen(?)
act:
  = Value
  >Value<: <= 10
  >Value<:  >  0
