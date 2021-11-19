AndroidRenderer: 
  activities: renderer.ActivityRenderer
  resources:  renderer.ResourceRenderer

----------------------------------------

Bestandsdaten:
  // :init // implizit?
  some: ? init==success ? passt : nicht

Bestandsdaten.DatenLaden:
  lager: ui.comboBoxLager.SelectedValue 
  // !ui.gridArtikelliste.Columns.(ui.Lagername.Spaltenbezeichnung).Visible = (lager.lagerId == 0)
  // == wird in visueller Repräsenatation mal anders ausschauen
  !ui.gridArtikelliste.Columns.LagerNr.Visible = (lager.lagerId == 0) 

Bestandsdaten.DatenLaden:
- Ausgewähltes Lager abrufen
- Eingestellten Spezialfilter abrufen
- Lagerspalte ausblenden falls keines ausgewählt
- Lagerstände entsprechend abrufen
- Abgerufene Zeilenanzahl ausgeben
- Lagerstände ausgeben

Bestandsdaten.laden!
  - Ausgewähltes Lager abrufen
  - Eingestellten Spezialfilter abrufen
  - Lagerspalte ausblenden falls kein Lager ausgewählt
  - Lagerstände entsprechend abrufen
  - Abgerufene Zeilenanzahl ausgeben
  - Lagerstände ausgeben

  Ausgewähltes Lager abrufen = ui.comboBoxLager.SelectedValue
  Ausgewähltes Lager abrufen!
    = ui.comboBoxLager.SelectedValue

  Eingestellten Spezialfilter abrufen = ui.comboBoxBestandunterschreitungsfilter.SelectedValue

  Lagerspalte ausblenden falls kein Lager ausgewählt:
    ui.gridArtikelliste.Columns.LagerNr.Visible = (lager.lagerId == 0)

  Lagerspalte ausblenden falls kein Lager ausgewählt:
    - Lager ausgewählt = (lager.lagerId != 0)
    ? Lager ausgewählt : Lagerspalte ausblenden // andere Syntax für "nicht"!

  Lagerspalte ausblenden:
    !ui.gridArtikelliste.Columns.LagerNr.Visible = false // implizit auch einblenden falls doch, hier nicht berücksichtigt!!