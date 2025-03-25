---
title: 'Search and Replace — nowa, prosta aplikacja'
permalink: 'search-and-replace-nowa-prosta-aplikacja'
category: 'Nowe aplikacje'
tags: ['windows', 'linux', 'gnu gpl', 'Search and Replace', 'plik tekstowy', 'GNU GPL v3', 'txt', 'aplikacja', 'nowa', 'kod źródłowy']
language: pl
date: 2011-08-17 14:25:11+0000
---

**W dniu dzisiejszym — tj. 18 sierpnia 2011 roku — światło dzienne ujrzała nowa aplikacja mojego autorstwa. Jest nią Search and Replace.**

Search and Replace to prosta aplikacja, która zamienia jeden ciąg znaków w pliku tekstowym na inny. Ciągi te definiowane są w pliku *config.txt*. Format tego pliku wyjaśniam poniżej:

```plain
Tekst zamieniany
Tekst docelowy
Tekst zamieniany
Tekst docelowy
Tekst zamieniany
Tekst docelowy
itd...
itd...
```

Jak widać konfiguracja programu jest idiotycznie prosta. Obsługa również jest łatwa (pomimo tego, że program działa w konsoli). Można ją przedstawić jako `"Search and Replace" plik-do-konwersji`.

Jednym z celów projektu było zmieszczenie kodu źródłowego w stu linijkach. Udało się — Search and Replace to jest bowiem 99 linijek kodu. Razem z komentarzami — bez nich jest jeszcze mniej.

A, tak tak — kod źródłowy. Kod źródłowy aplikacji jest bowiem otwarty, ponieważ program jest udostępniany na licencji GNU GPL v3. Każdy może więc go sobie podejrzeć — choć tak naprawdę nie ma czego 😜

No, więc to by było na tyle zbędnego gadania. Zapraszam do [pobieralni](https://github.com/m4tx/search-and-replace/releases/tag/v1.0)!

Enjoy!

PS. Tak btw. — program dostępny jest zarówno dla Windowsa, jak i Linuksa 😄
