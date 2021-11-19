Textbox:	@https://pyr.at/y/basic-web/Textbox.y
Web:		  ´https://pyr.at/basic-web@0.8.0/´
Webpage:	Web.Page
BasicWeb: pyr.at/basic-web@0.8.0

:irgendwas:
	[identity]
	# hier keine verschachteltene Comp-Referenzen mehr erlaubt (müssen zu einzelreferenzen vorher aufgelöst werden zB. Webpg: Web.Page)
	:Web/Page                    # new instance must be created (with parent component?)
	:HtmlDef.Page                      # ansprechbar via irgendwas.HtmlDef.Page? -- nicht optimal.
	:´https://pyr.at/basic-web@0.8.0/´ # nicht erlaubt
	Web.Textbox("Hello World!") # new instance is specified
	+ Web.OnClick                 # instance has be given

	
[Handscanner App]:
  pyr.at/basic-app 		 # allowed directly, or urls in some config file?
  some.other/basic-app # durch (punkt+)slash alleinstellungsmerkmal als url?
  Pages: pages/*
  Settings: [project settings path]
	
	@pageChange(evt, page): somethingHappens 
	@pageChange /, page {
		setNewPage(page)
	}

	@pageChange:
		setNewPage(.page)
		console.log(.evt)

	[Someelse in Handscanner]: indentationMattersFragezeichen
	# auto convert to spaces?? - new indentation = new context (no matter how far or less indented ... hm no, further right = deeper, amount of spaces does not matter)

 @someElse: # <- unclear indentation error
			somethingHere
			somethingThere
	nowBackAt [Handscanner App]?

	[notherThingHere] // square bracket overkill?
		blub

aktivieren:
	ByteArrayOutput(0x0F, aktivieren ? 0xF2 : 0xF0, 0x00)
	
	=> aktivieren
	=> out
	
	[on param change]
	LastMonitoredSteuerung = 0;
	out.write(ByteArrayOutput.toByteArray)
	
	[? aktivieren]

	-------------------------------------------------------------------------

	--- sldfdasf
	Some Here:
			some here

	
Meine Activity:
		'Andoird Base''s Activity'
		Android Base's Activity   # <- geht?? 
		'Android Base#s Activity' # <- geht!
		'Android Base:s Activity' # <- denke geht

		# FRAGE: Führt Labels mit 'quotes' deliminieren zu können dazu, dass alles deliminiert werden (siehe JSON)???
		#  WEIL: Wäre nicht hübsch/nett zu lesen find ich. Nicht immer zumindest, aber ja. Auch kay denk ich?

		# FRAGE: Sollte bei vorkommen von Leerzeichen schon 'quotes' erforderlich sein?
		#  WEIL: Mit Space getrennt könnte eine Funktion/Paramübergabe/etc. sein.

		# FRAGE: Sollen quotes nicht nur direkte Labels sein, sondern auch sonstiges evaluieren?
		#  WEIL: 
		#   BSP: "Wert = '3 + 12'" könnte zu "Wert = 15" übertragen werden?? (eher nicht mit selber Syntax!)
		#   BSP: "Wert = '3 + Component'" könnte aber ggfs nicht weiter aufgelöst werden (könnte so ein exaktes Label sein)

		# FRAGE: Typzuweisung mit ":", (Instanz-/)Wertzuweisung mit "="? (CSS Stil widerspricht?)
		# FRAGE: Unterschied zwischen "Typ" und "Wert" in y? Ist Wert/Instanz eine sehr primitive Version eines Typen?
		#  MISC: Ein bestimmter (primitiver) Wert ist immer konstant/finit (12 bleibt 12?)?
		#  MISC: Ein Wert (zB "beliebiger text") könnte auch Teil des wirklichen Typen (mit Methoden etc) sein? (daher ist wert sicherlich kein unbedingter Supertyp, eher einer (von ggfs mehreren) Parametern)?
		# FRAGE: Zuweisung dennoch auf gleichem/ähnlichem Weg möglich wie Typ????? (eher nicht oder? verwirrend?)

		#  MISC: Setting Values / Passing Values / Instantiation / Constants, share similarities!
		#  MISC: Type Definition (/ Structuring) is treated as a different concept!
