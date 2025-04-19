---
title: 'Duplicate Lines Finder 2.0'
permalink: 'new-application-duplicate-lines-finder-2-0'
category: 'Zmiany w aplikacjach'
tags: ['c++', 'Duplicate Lines Finder', 'gnu gpl', 'java', 'separator', 'wielojęzyczność', 'licencja', 'Freeware', 'minimalistyczność']
language: pl
date: 2012-02-12 17:47:57+0000
---

**Niektórzy czytelnicy mojego bloga zapewne pamiętają moje początki z językiem C++, kiedy to napisałem Duplicate Lines Findera 1.0, a krótko potem — 1.1. W dniu dzisiejszym — 12 lutego 2012 roku — ponad półtora roku po wydaniu ostatniej wersji, tj. 1.1 program powraca — tym razem nowy, odświeżony, jako już Duplicate Lines Finder 2.0.**

Zmian jest dosyć sporo. Przede wszystkim nastąpiła zmiana języka programowania, w którym wspomniany program jest napisany — na język Java, oraz zmiana licencji — z Freeware na GNU GPL. Oznacza to jednocześnie, że program jest teraz Open Source!

Kolejną ważną zmianą jest wielojęzyczność. Java umożliwiła mi bardzo łatwą zmianę języka w zależności od tego, jaki język w systemie ma aktualnie ustawiony użytkownik programu. Aktualnie jedynymi obsługiwanymi językami są polski oraz angielski, ale... Może kiedyś się to zmieni? 😄

Następną zmianą jest nowy interfejs. Wybór plików — docelowego oraz źródłowego — odbywa się obecnie poprzez 2 przyciski. Zrezygnowałem z etykiety, pola tekstowego i przycisku tak, jak to wyglądało w poprzednich wersjach — teraz funkcje tych wszystkich widżetów przejął jeden przycisk. Oprócz tego dodałem pasek postępu pokazujący postęp wykonywania operacji. Jest on połączony z przyciskami Start i Anuluj w następujący sposób: po wciśnięciu przycisku Start, zamienia się on w przycisk Anuluj. Po opuszczeniu kursorem obszaru tegoż przycisku, zamienia się on w progressbar. Po najechaniu myszką na pasek, zamienia się on z powrotem w przycisk Anuluj. Prosto i minimalistycznie 😄 Oprócz tego rozbudowałem również statystyki.

Z pozostałych zmian warto zaznaczyć, że teraz separator nie musi być już pojedynczym znakiem — może być dowolnej długości. Program teraz działa — chyba, nie robiłem dokładnych testów — szybciej.

To tyle o nowym Duplicate Lines Finderze. Mam nadzieję, że Wam się spodoba 😄

Ah — warto również wspomnieć, że opublikowałem aplikację również na [angielskiej wersji m4txbloga](/blog/new-application-duplicate-lines-finder-2-0/).

[Zapraszam więc do pobieralni.](https://github.com/m4tx/duplicate-lines-finder/releases/tag/v2.0)

Pozdrawiam 😄
