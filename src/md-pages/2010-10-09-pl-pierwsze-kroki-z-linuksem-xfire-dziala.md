---
title: 'Pierwsze kroki z Linuksem — Xfire działa!'
permalink: 'pierwsze-kroki-z-linuksem-xfire-dziala'
category: 'windows'
tags: ['linux', 'Pidgin', 'pierwsze kroki', 'zależności', 'Gfire', 'źródła', 'GLib', 'GTK+', 'konsola', 'Pierwsze kroki']
language: pl
date: 2010-10-09 19:00:58+0000
---

**Dzisiaj działo się znacznie więcej niż od dnia instalacji. Mimo, iż odwiedziłem dzisiaj kolegę miałem czas, aby się tym wszystkim zająć. Ale po kolei.**

Pobrałem przede wszystkim program, dzięki któremu nie muszę już się przełączać na system Windows, aby uruchomić połączenie internetowe. Jest to już jeden z kroków do uniezależnienia się od systemu Microsoftu.

Próbowałem ponownie zainstalować Pidgina z wtyczką Gfire. Generalnie się nawet udało, lecz... Łatwo nie było.

Chciałem zainstalować komunikator Pidgin ze źródeł. I tu właśnie zaczęły się kłopoty. Najpierw skrypt configure zażądał biblioteki GLib. OK, pobieram z internetu wtyczkę GLib, próbuję zainstalować. Lipa. GLib potrzebuje następnych zależności. Następnie, gdy już udało mi się zainstalować GLiba, okazało się, że Pidgin wymaga także GTK+. Instaluję. No i tu też są problemy. GTK+ również wymaga sporej liczby zależności. Pobierałem je z początku co prawda z internetu w postaci źródeł, jednak potem zrezygnowałem. Pojechałem do wspomnianego wcześniej znajomego. Po powrocie nie męczyłem się już dłużej. Wpisałem aptitude install pidgin. Wyświetliło się, że pobrać trzeba ponad 70MB danych. Generalnie nie byłoby problemu, ale na moim łączu... "No trudno, jakoś przeżyję" — pomyślałem. I faktycznie przeżyłem.

Zabierałem się za instalację Gfire. Wymagał najpierw pakietu purple, który zainstalowałem poprzez aptitude, a następnie bibliotek programistycznych GTK+. To także zainstalowałem przez aptitude. Wreszcie poczułem ulgę. Po wpisaniu ./configure i odczekaniu kilku sekund wyświetlony został jednoznaczny komunikat "Type make to compile.". Make. Make install. Uruchamiam Pidgina i...

Działa. Po sporych mękach udało mi się porozmawiać z użytkownikiem Xfire w systemie Linux.

Ja myślę, że to nieużywanie Linuksa przez większość społeczeństwa bierze się właśnie z konsoli. Ludzie po prostu boją się wpisywania kolejnych komend w terminalu, lecz tak naprawdę nie jet to niczym trudnym. Wystarczy kilka dni, aby nauczyć się obsługi systemu i przyzwyczaić się do niego. Do Linuksa istnieje wiele ułatwień takich jak na przykład aptitude, dzięki którym praca z tym systemem staje się prostsza i przyjemniejsza. Coraz bardziej odczuwam potęgę Linuksa oraz nie mam żadnych planów, takich jak powrót do Windowsa, bądź przejście na zupełnie inny system. Każdemu polecam ten system, a wszystkich czytelników bloga zapraszam na kolejne części serii "Pierwsze kroki z Linuksem"!
