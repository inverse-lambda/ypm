# Development Plan

- [ ] Implement Lexer 
  - [ ] Sprache grob spezifizieren
    - [x] Eintrittspunkt in Datei festlegen -> "main" Komponente
    - [x] Tabs und Spaces und erlaubte Kombinationen festlegen
      - [x] Gemischt erlaubt, wobei 1 Tab = 4 Spaces entspricht?
      - Nur Spaces erlaubt?
      - Mischung ähnlich Python erlaubt (spaces nach tabs etc.)
    - [ ] Parallel vs Synchron Syntax bestimmen
      - Mit vorangestelltem Dash (-) für parallele/allgemeine Operationen
      - Mit vorangestelltem Dash (-) für explizit synchrone Operationen
      - Mit Rufzeichen (!) für auszuführende statt deklarative Operationen??
      - ..

    - [ ] Gute Codesnippets für das Schreiben in y finden

  - [ ] Lexer Grundfunktion
    - [ ] Einfache Wörter finden
    - [ ] Doppelpunkt finden
    - [ ] Einrückungen finden
- [ ] Implement Parser
