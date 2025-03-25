---
title: 'Aktualizacje!'
permalink: 'aktualizacje'
category: 'Nowe aplikacje'
tags: ['Zmiany w aplikacjach', 'programowanie', 'gbx thumb extractor', 'c++', 'SmsPrice', 'Duplicate Lines Finder', 'program', 'aktualizacja']
language: pl
date: 2010-07-23 10:17:23+0000
---

## GBX Thumb Extractor

Wreszcie obiecywany GBX Thumb Extractor ujrzał światło dzienne. Prawie, bo niestety potrzebuje do działania kilku „składników”. Przede wszystkim wymagany jest program `jpegtran.exe`, który musi znajdować się w głównym katalogu programu, a oprócz tego ścieżka do pliku wyjściowego nie może zawierać spacji. W przeciwnym wypadku wynikowy obraz będzie odwrócony w pionie. No cóż, program jednak działa poprawnie.

Przede wszystkim zdecydowałem się na użycie interfejsu identycznego, jak w przypadku Duplicate Lines Findera, spora część kodu pochodzi też z tego programu. Jeszcze wspomnę o tym odwracaniu w pionie — jeżeli ktoś zna jakiś sposób na to odwracanie bez użycia zewnętrznych aplikacji, w C++ oraz współpracujący z kompilatorem MinGW32 — proszę o kontakt, nawet poprzez komentarz.

No, więc kończę tu opowiadanie o GBX Thumb Extractorze, żeby znowu nie wyszło jak z Duplicate Lines Finderem i zapodaję link do progsa:

**[Link do programu GBX Thumb Extractor](https://github.com/m4tx/gbx-thumb-extractor/releases/tag/v1.0)**

## SmsPrice 1.1

SmsPrice doczekał się aktualizacji. Zmiany są głównie kosmetyczne, można o nich poczytać w Changelogu, ale usunąłem też trochę denerwujący błąd, który zawiesza program. Nie będę się tu rozpisywał, bo w zasadzie nie ma o czym, zapraszam do ściągnięcia najnowszej wersji:

**[Link do programu SmsPrice](https://github.com/m4tx/smsprice/releases/tag/v1.1)**

## Duplicate Lines Finder 1.1

No tak, wydałem GBX Thumb Extractora, zaaktualizowałem SmsPrice, możnaby się domyślać, że aktualizacji doczeka także Duplicate Lines Finder. Zgadza się.

Tu zmian jest więcej: przede wszystkim dodałem możliwość wyboru separatora — dzięki temu, jeżeli ciągi znaków są oddzielone np. spacjami, można w takim pliku także wyeliminować duplikaty. Dodałem także statystyki, dzięki którym dokładnie widać, ile zyskano miejsca, ile w pliku było duplikatów, oraz jak długo program wykonywał swoją operację. Dodałem dymki z informacjami, co dany przycisk lub coś innego robi. Wprowadziłem ignorowanie pustych linii a także wsparcie dla Windows'owego systemu kodowania — program mógł czasami nie działać, gdyż Windows nieco inaczej koduje znak nowej linii. O reszcie zmian można przeczytać w Changelogu, a teraz po raz trzeci i ostatni w tym wpisie zarzucam link:

**[Link do programu Duplicate Lines Finder](https://github.com/m4tx/duplicate-lines-finder/releases/tag/v1.1)**

Miłego używania! 😉
