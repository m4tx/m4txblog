---
title: 'Quake Live — pierwsze wrażenia'
permalink: 'quake-live-pierwsze-wrazenia'
category: 'internet explorer'
tags: ['firefox', 'Quake', 'Quake Live', 'FPS', 'Quake III Arena', 'Pierwsze kroki']
language: pl
date: 2011-06-08 15:00:35+0000
---

**O darmowym Quake III Arena w przeglądarce słyszałem dość dawno. Niestety, ze względu na słabe łącze internetowe, nie mogłem go wypróbować. Teraz mam nieco lepsze, a w tym wpisie postaram się nieco przybliżyć tę grę. Warto!**

I tak, tak — oczy Was nie mylą. Napisałem "darmowym Quake III Arena w przeglądarce". Quake Live bowiem działa na zasadzie pluginu do przeglądarki. Ale o tym zaraz.

Tak więc, aby zagrać w Quake Live, trzeba mieć konto. Wchodzimy więc na stronę [quakelive.com](http://quakelive.com/) i klikamy w duży, czerwony przycisk Register Now znajdujący się w górnej części strony. Po wpisaniu e-mailu, nicku, dwukrotnie hasła, podaniu swojej daty urodzenia, "zahaczeniu", że przeczytaliśmy i akceptujemy warunki licencyjne oraz warunki korzystania i ewentualnie "odhaczeniu" chęci skorzystania z newsletteru, można by wreszcie ściągnąć plugin do przeglądarki. I tu właśnie zaczynają się schody.
Quake Live obsługuje Internet Explorera, Firefoksa (w wersji 2.x albo 3.x) i Safari (ale tylko na MacOS). Tak więc jeżeli korzystasz z Internet Explorera — nie ma problemu. Jeżeli z Firefoksa — niekoniecznie. Czasami po prostu pomimo poprawnego zainstalowania pluginu, gra nie działa, a jeżeli masz Firefoksa 4.x, w ogóle pluginu nie da się zainstalować. Rozwiązanie jest jednak proste.
Należy zamiast instalowania pluginu, zapisać go na dysku, następnie otworzyć go w jakimś programie archiwizującym (7-zip, winRAR, File Roller, Ark... ew. można zmienić rozszerzenie na zip i otworzyć wbudowanym w Windowsa archiwizerem), a katalog plugins znajdujący się w środku skopiować do *~/.mozilla* (a na Windowsie... nie wiem, pewnie gdzieś w *%APPDATA%\mozilla*... albo coś takiego). Powinno działać.

Tak więc jak już mamy zainstalowany plugin, można przystąpić... Nie, nie do grania. Teraz musimy zassać jakieś 400MB. Są to takie rzeczy jak silnik, dźwięki, grafika, mapy, itd... Gdy już to zrobimy, **można wreszcie przystąpić do grania**. Naszym oczom ukazuje się taki oto widok:

[![Quake Live](/static/images/blog/2011-06-08-pl-quake-live-pierwsze-wrazenia-ql.jpeg)](/static/images/blog/2011-06-08-pl-quake-live-pierwsze-wrazenia-ql.jpeg)

Widać tutaj więc listę serwerów, która nas najbardziej interesuje, ponieważ jest to najłatwiejszy sposób na podłączenie się do serwera. Warto się również zainteresować opcją Customize, gdzie można określić, jakie serwery mają być wyświetlane (np. tylko Instagiby z graczami — ale przed rozpoczęciem gry — grającymi na podobnym poziomie, jak my). Jeżeli pierwszy raz grasz w Quake Live, lub po prostu chcesz sobie sam zagrać — zainteresuj się opcją Practice. Możesz tam rozegrać mecz z botami lub wykonać jakiś tutorial, albo po prostu oswoić się z grą. No więc jak już będziesz sobie chciał zagrać, warto wiedzieć, że domyślne ustawienia są beznadziejne. O ile na szczęście wszyscy wrogowie mają taki sam model, o tyle broń zajmująca prawie 1/4 ekranu dalej jest wyświetlana. Warto więc wprowadzić szereg udoskonaleń do ustawień gry.

Rozpocznij więc mecz w trybie Practice, a następnie otwórz więc konsolę za pomocą klawisza \` znajdującego się pod klawiszem Esc i wpisz:

- *r_picmip 30* — Rozmywa tekstury ścian i tym podobnych. Czemu? Ano otóż temu, że lepiej widać na takich ścianach wrogów, niż na jakichś ultrasuperpro wysokich detalach.
- *cg_drawGun 0* — dzięki temu broń nie jest już wyświetlana (uwierzcie, naprawdę nie musicie widzieć jaką macie broń... 😄 )
- *cg_forceEnemyModel tankjr/bright* — ustawia model przeciwnika na jaskrawy TankJR
- *cg_fov 110* — ustala kąt widzenia. Domyślne 90 jest beznadziejne (mało widać), najlepiej chyba ustawić 110.

I generalnie tyle wystarczy. Warto jeszcze wyłączyć wszelki dym, krew i takie tam, ale proponuję samemu pobawić się konsolą.

Możemy więc podłączyć się do jakiegoś serwera. W karcie Play Online wybieramy jakiś serwer. Należy odczekać 10 sekund na wyświetlenie reklamy, a następnie rozpoczyna się ładowanie modeli, tekstur, mapy i dźwięków. Gdy już jesteśmy na serwerze, można kliknąć... No co? Join Match!
I w tym momencie zaczyna się zabawa. 😄 10 osób nawalających się z 11 całkowicie odmiennych broni byleby kogoś zfragować, to nie jakieś tam mierzenie pół godziny w głowę... Tutaj czy ktoś trafi w palca czy w głowę, są identyczne obrażenia. Na mapach są porozstawiane różne power-upy, np. dodające 100hp, poczwórne obrażenia, czy niewidzialność, a ponadto zbroje, bronie i amunicja. To wszystko nadaje niesamowicie szybkiego tempa grze, a przyspieszają je jeszcze różne triki, takie jak circle i strafe jumping, grenade i rocket jumpy, plasma climbing i wszystko w odmiennych trybach jakimi są FFA, CTF, TDM, a ponadto wszelkie Insta\* (Instagiby, InstaCTFy itd. Kiedyś były jeszcze Rocket\* w których używało się tylko wyrzutni granatów i rakiet, ale już je usunięto. :( (są to tzw. DevPicki przygotowywane przez twórców, co tydzień są inne. Są dostępne przez weekend) Btw, widać je na screenie powyżej), a na rozrywkę we dwie osoby są również Duele. To wszystko nadaje tej niesamowitości tej grze i pomimo tego, że Quake III ma już 13 lat, to wciąż grywalność jest na bardzo wysokim poziomie.

Quake Live to pozycja obowiązkowa dla każdego wielkiego fana FPSów. Natomiast zarówno dla fana jak i nie-fana FPSów przyniesie ogromną ilość znakomitej rozrywki — a przy okazji warto wspomnieć — działającej na słabszym sprzęcie i na wszystkich popularnych systemach.
