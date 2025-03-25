---
title: 'Swing, czyli poradnik "Jak zrobić beznadziejną bibliotekę do GUI"'
permalink: 'swing-czyli-poradnik-jak-zrobic-beznadziejna-biblioteke-do-gui'
category: 'GTK+'
tags: ['Qt', 'biblioteka', 'java', 'GUI', 'Swing', 'Oracle', 'Sun', 'krytyka', 'Przemyślenia']
language: pl
date: 2012-02-14 11:10:36+0000
---

**Na początku powiem może czym jest Swing. Otóż Swing jest to ulepszona wersja biblioteki AWT, służącej do tworzenia GUI w Javie. Obecnie Swing jest w zasadzie jedyną sensowną biblioteką do GUI dla Javy, niestety. Jest aktualnie rozwijany przez firmę Oracle, która rozwija również platformę, na której działa Swing — Javę. Swing jest więc rozwijany przez aktualnego właściciela Javy. Jest to bardzo dobrze z punktu widzenia programisty. Dlaczego więc Swing jest zły? Otóż...**

Swing na pewno nie jest najgorszą biblioteką do GUI. Oferuje ciekawe możliwości, jest wieloplatformowa, integruje się z systemem (mówię tutaj m.in. o wyglądzie aplikacji), ogólnie jest to bardzo fajna biblioteka. Ma jednak kilka bardzo, bardzo, ale to bardzo poważnych wad, których w na tyle profesjonalnej bibliotece nie powinno być.

Pierwszą wadą, jaką chciałbym napisać, są layout managery. Layout managery układają widżety według określonego schematu. Np. w GTK+ 3.0 mamy `GtkBox` (czyli odpowiednik `GtkHBox` i `GtkVBox` w GTK+ 2.x), `GtkGrid` (czyli `GtkTable`), `GtkAlignment` i... to wystarcza do stworzenia niemal każdego GUI. A jak nie, to zawsze jest jeszcze `GtkFixed`, czyli ustawianie widżetów jak się chce. Natomiast co mamy w Swingu? Otóż w Swingu sytuacja wygląda nieco inaczej. W Swingu bowiem mamy: `BorderLayout`, `BoxLayout`, `CardLayout`, `FlowLayout`, `GridBagLayout`, `GridLayout`, `GroupLayout` i `SpringLayout`. Czyli w GTK+ mamy 3 główne layout managery (są jeszcze inne, ale nie będziemy się nimi w tej chwili zajmować), natomiast w Swingu... 8. Nie mam nawet pojęcia, do czego służą 3 z nich. I kolejne porównanie do GTK+: `BoxLayout` to taki trochę odpowiednik `GtkBox`a, `GridLayout` to takie coś pomiędzy `GtkBox`em, a `GtkGrid`em, `FlowLayout` — sam nie wiem, `GridBagLayout` to jest taki czysty `GtkGrid`, tylko gorszy, a `BorderLayout`... `BorderLayout` to w żadnej bibliotece do GUI nie widziałem nawet. Ustawia on widżety na 4 brzegach rodzica + jeszcze jeden w środku. Na stronie [oficjalnego tutoriala Swinga znajdziecie screena](http://docs.oracle.com/javase/tutorial/uiswing/layout/border.html). To jest totalny bezsens. I zawsze, jak mam napisać jakiś program w Swingu, to się wkurzam, bo nie mogę dobrać odpowiedniego layout managera...

Następną rzeczą, która jest moim skromnym zdaniem źle zrobiona, to są zdarzenia. W innej, profesjonalnej bibliotece do GUI (żeby już nie faworyzować tak GTK+ 😄), konkretnie Qt, zdarzenia oparte są na sygnałach. Np. `QPushButton` — wysyła sygnał clicked, gdy użytkownik go wciśnie. I można potem taki sygnał bardzo łatwo przechwycić. Natomiast w Swingu mamy to rozwiązane w inny (czytaj: gorszy) sposób. Otóż podpinamy sobie `Listener`a (w tym przypadku `ActionListener`a) do takiego naszego `JButton`a. Kolejne, bezsensowne rozwiązanie, którego nie ma również w żadnej innej profesjonalnej bibliotece do GUI.

Kolejną poważną wadą Swinga jest brak możliwości wczytywania GUI z pliku. W GTK+ mamy pliki `GtkBuilder`a, w Qt mamy pliki `.ui`. Oba te formaty są oparte na XML-u. Natomiast co mamy w Swingu? W Swingu, niestety, nic nie mamy. Aczkolwiek wydaje mi się, że aby dodać taką możliwość, tj. wczytywanie GUI z pliku, i żeby to miało jakąś sensowną funkcjonalność, to najpierw trzeba by było napisać praktycznie całą tę bibliotekę od podstaw, tym razem używając m.in. opisanych już sygnałów.

Następną rzeczą, rzucającą się w oczy głównie na Linuksie, jest okropna szybkość działania Swinga przy ustawionej integracji wyglądu z systemem — `GtkLookAndFeel`. Nie dość, że ten wygląd GTK+ Swing udaje raczej beznadziejnie ("look" to jeszcze, chociaż i tak słabo, ale "feel" to już w ogóle...), to jeszcze ta szybkość... Plus za wspieranie Linuksa, ale dwa minusy za to, jak on jest wspierany, niestety.

To już koniec mojej krytyki biblioteki do GUI, jaką jest Swing. Mam nadzieję, że chociaż trochę przybliżyłem Wam to, jak **NIE** powinno się pisać tego typu bibliotek. Jak widać, nawet tak znana i ceniona firma Oracle (właściwie to Sun) potrafi napisać coś, z czego z trudem się korzysta. A jest się — można powiedzieć — skazanym na korzystanie z tego. Cóż, niestety.

Pozdrawiam 😄
