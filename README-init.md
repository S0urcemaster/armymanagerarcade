# Army Manager Arcade

Mobile Web Game about building up an army, battle with other armies and advance in history

Keep the client English

## Anfang

Man startet als eine Person mit etwas Geld und Ausrüstung

## Erster Lauf: Gegner auswählen

Man sieht eine Liste passender Gegner, die man sich zutrauen könnte, zu schlagen

Die ersten 10 Gegner, die man bezwingt, kommen unversehrt zur eigenen Truppe hinzu

Dann, mit 2 Mann könnte man sich mehr als einen Gegner zutrauen. Das Angebot an Gegnern variiert entsprechend der eigenen Spielstufe. Verliert man auf diesem Niveau, muss man wieder bei 1 anfangen

Gewinnt man gegen einen stärkeren Gegner, wechseln mehr der gegenerischen Truppen auf die eigene Seite. Verliert man gegen einen schwächeren Gegner, erleidet man ebenso mehr Verluste

## Kampf

Wie bei einem Soldaten gegen einen 

Nachdem man das Schlachtfeld betreten hat, kann man einstellen, welcher Teil seiner Armee angreifen oder verteidigen soll

Der Kampf zwischen 2 Armeen unterteilt sich also in : Zentral, Linke Flanke, Rechte Flanke, dann Fernangriff, Infanterie, Kavallerie. Man kann zB sagen: Rechte Flanke defensiv, Kavallerie aggressiv - In den Flanken oder zentral sind alle Truppentypen enthalten

Bei der Gegnerauswahl sieht man nicht alle Details, nur genau die Belohnung, die man vom Auftraggeber bekommt im Falle des Sieges. Es gibt so ein gewisses Risiko und später ein Gefühl dafür, größere und stärkere Gegner besiegt zu haben

Eine Truppe (der Einzelspieler am Anfang hat dieselben Durchschnittswerte wie eine Truppe) kann in bestimmten Haltungen spezialisiert werden, zB Kavallerie, die dann als überlegener Vorteil in einer Schlacht ausgespielt werden kann. Für den Einzelspieler und eine Spielfigur gelten dieselben Stärken:

- Zentral
- Links
- Rechts
- Fernkampf
- Nahkampf
- Reiten

mit einer individuellen Gewichtung. Für den eigenen Charakter wird man am Anfang Punkte verteilen können, die sich mit einem Multiplikator auf den Durchschnitt der ganzen Truppe auswirken

Man greift mehrmals mit der eingestellten Haltung an und der Gegner wehr mit seiner Konterfähigkeit ab: Links/Rechts gespiegelt, Nahkampf/Speer schlägt Kavallerie, Kavallerie schlägt Fernkämpfer, Fernkämpfer hat Vorteil bei Infanterie

Jeder Spezialisierungsteil hat eine Zahl an Lebenspunkten. Geht die unter 50%, kann man selbst nicht mehr angreifen, bzw der Gegner flüchtet. Dann teilt sich der Teil, der gewonnen hat, auf die anderen Truppenteile auf und verstärkt sie

ZB (use case) kann man eine Stärke aufbauen und so dosieren, dass man rechtzeitig "durchbricht" bevor eine der schwächeren Teile nachgibt. Das könnte einen Teil der Spannung ausmachen auf einem Schlachtfeld, das erst mal nur mit Fortschrittsbalken animiert wird

## Rekrutierung

Hat man seine ersten Verbündeten besiegt und damit automatisch für sich gewonnen, geht es in Phase 2. Ab da bis zum Ende kann man Verluste an der eigenen Truppe erleiden

Es gibt eine Liste mit Gruppen von Einheiten, die sich bewerben. Einheiten von 1 oder mehr, entsprechend der Spielstufe

In welcher Spielstufe man ist, bestimmen Rufpunkte
Man erwirbt Rufpunkte durch Siege und verliert welche durch Niederlagen
Rufpunkte bestimmen die Qualität der Rekruten, die sich bewerben
Rufpunkte bestimmen die Größe der Armee, was die Gliederung der Liga bestimmt

Man wählt also unter den angebotenen Rekruten aus bis man voll ist oder kein Geld mehr hat

## Ausrüstung

Es gibt auch Ausrüstung. Manchmal sind Rekruten wegen ihrer Ausrüstung gut. In einem Screen nach der Rekrutierung rüstet man seine Armee aus

Infantrie
- Speer + Schild
Fernkämpfer
- Pfeil + Bogen
Kavallerie
- Pferd und Lanze

Jeder Soldat kann eine Rüstung bekommen

Zur Vereinfachung hat jeder Soldat mindestens eine "Schlechte Ausrüstung". Es gibt weitere Stufen/Qualitäten

- Schlechte Ausrüstung
- Gebrauchte Ausrüstung
- Akzeptable Ausrüstung
- Gute Ausrüstung
- Neue Ausrüstung
- Aufgewertete Ausrüstung
- Modernste Ausrüstung

Das Spiel geht über eine gewisse Kampagne, und Ausrüstung verliert pro Einsatz eine Stufe. Man soll also beste Ausrüstung für die letzte Spielphase vorhalten/ sparen wollen

## Kampf

Die 6 .. Zonen, sag ich jetzt mal, stehen sich gegenüber:

Zentral - Zentral
Rechts - Links
Links - Rechts
Kavallerie - Kavallerie
Fernkampf - Fernkampf
Infanterie - Infanterie

Bei Kampfbeginn stehen sie die Einheitstypen nicht gegensätzlich gegenüber. Erst muss man den gegnerischen,  gleichen Einheitentyp besiegen (in die Flucht schlagen mit mehr als 50% Verlust), dann stößt der Truppenteil, der gewonnen hat zu seinem bevorzugten Angriffsziel. ZB hat die Kavallerie gewonnen, greift sie danach ebenso die Fernkämpfer mit 50% Bonus an

Die Kämpfe zwischen den Zonen werden nach Breite der Angriffslinie berechnet, so dass sich immer gleich viel Soldaten auf ein mal gegenüberstehen. Ist eine Reihe ausgefochten, einer von beiden Soldaten, die sich gegenüber stehen, ist verwundet oder tot, treten die Gewinner ans Ende der Schlange. Verwundete werden bis zur nächsten Schlacht geheilt, treten aber nicht mehr in die Schlacht

Verwundete oder getötete Soldaten verlieren ihre Ausrüstung. Der Gewinner einer Schlacht bekommt am Ende alle verlorene Ausrüstung mit der üblichen Stufe Qualitätsverlust

Der Kampf zwischen 2 sich gegenüber stehenden Soldaten wird augsefochten, indem der Vorteil/ die stärkste Eigenschaft des Soldaten mit seiner Zuordnung zum Truppenteil mit demselben Vorzug verrechnet wird. Der stärkste Wert wird B Waffenqualität gegen den schwächsten Wert des Gegners verrechnet, abzüglich Rüstung. Der erste mit weniger als 50% Lebenspunkten fällt aus dem Kampf und der andere gewinnt. Ein Schlag kann tödlich sein, wenn er die Lebenspunkte eines Gegners mit einem Schlag auf 0 bringt

ZB hat ein Soldat, der Infantrie zugeordnet ist und zentral steht, bei Infantrie seinen höchsten Wert, steht somit gut, aber bei der Flanke wäre er lieber rechts, usw

Ein Soldat hat also 2 Spezialisierungen:

- Typ
- Flanke

und man muss die rekrutierten Soldaten möglichst optimal zuteilen

Ein Soldat hat einen weiteren Wert : Zugehörigkeit. Wird ein Soldat aus einer existierenden Gruppe herausgenommen oder wird eine Gruppe verteilt, verlieren die Mitglieder an Zugehörigkeitsgefühl. Soldaten mit hoher Zugehörigkeit bekommen einen Bonus beim Kampf

