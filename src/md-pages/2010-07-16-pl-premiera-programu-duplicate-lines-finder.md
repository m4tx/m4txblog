---
title: 'Premiera programu Duplicate Lines Finder'
permalink: 'premiera-programu-duplicate-lines-finder'
category: 'Nowe aplikacje'
tags: ['programowanie', 'blog', 'Duplicate Lines Finder', 'premiera', 'program', 'usuwanie zduplikowanych linii']
language: pl
date: 2010-07-16 18:31:21+0000
---

**Witam ponownie bardzo nieliczne osoby czytające tegoż bloga.**

Dziękuję, że czytając ten wpis, zdobyłeś się na trud odwiedzenia strony. No, ale przejdę do rzeczy. W dniu dzisiejszym ogłaszam premierę programu którego ukończyłem. Zwie się on Duplicate Lines Finder, no i służy — jak nazwa wskazuje — do wyszukiwania zduplikowanych linii w plikach tekstowych, ich usunięciu, a następnie zapisie do nowego pliku.

Przed napisaniem tego programu sporadycznie zdarzało mi się, że potrzebowałem właśnie usunąć duplikaty w plikach tekstowych. Było to zadanie żmudne i nudne, więc napisałem ten program. Nie jest on szczególnie skomplikowany, gdyż jego główne okno składa się jedynie z dwóch etykiet, dwóch pól edycyjnych i trzech przycisków. Ale najważniejsze, że robi to, co ma robić i robi to bezbłędnie. Chyba. Niestety, ale obsługa plików kosztowała mnie ponad 400KB, doliczając kod oraz ikonę wyszło mi 491.

W moim komputerze — podam tylko procesor, gdyż on jest odpowiedzialny za zrealizowanie głównego zadania — mam procesor dwurdzeniowy Intel Core 2 Duo E8400 ~3GHz. Przetworzenie pliku o rozmiarze 296KB zajmuje około 3 sekundy, a program korzysta niestety tylko z jednego rdzenia. Lecz wyobraź teraz sobie, ile czasu zajęło by Ci to samo, robiąc to ręcznie... Sprawdzić, które z ponad 26 000 linii się powtarzają, i w ogóle zdecydować się na coś takiego to nie lada sztuka. Jestem pewien, że żaden z ludzi nawet nie zdecydowałby się na coś takiego, a nawet jeśli to zrezygnowałby już przy pierwszej linii...

Obsługa wersji 1.0 wygląda tak: uruchom program, wybierz plik źródłowy, wybierz plik docelowy, wciśnij przycisk, poczekaj i zamknij program. Każdy sobie z nią poradzi. Nasuwa się natomiast pytanie: czemu nie zaimplementowałem w programie paska postępu?
No cóż, chciałem, ale jak go zrobiłem, to długość procesu wzrosła z 3 do około 30 sekund... Coś dziwnie prockożerny ten pasek jest!

O, no popatrz, dotrwałeś do końca... Ciężko było, żebyś odwiedził ten blog, ale to, że przeczytałeś cały ten wpis jest już prawdziwym cudem... Kurde, ale się rozpisałem! No, ale teraz już zapodaję link do programu:

[Link do programu Duplicate Lines Finder](https://github.com/m4tx/duplicate-lines-finder/releases/tag/v1.0)

Pozdro!
