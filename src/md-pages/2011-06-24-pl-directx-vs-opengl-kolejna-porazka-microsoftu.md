---
title: 'DirectX vs. OpenGL — kolejna porażka Microsoftu'
permalink: 'directx-vs-opengl-kolejna-porazka-microsoftu'
category: 'windows'
tags: ['linux', 'DirectX', '3d', 'PlayStation 3', 'OpenGL', 'biblioteka', 'Przemyślenia']
language: pl
date: 2011-06-24 17:33:33+0000
---

**Tak, tak, kolejny wpis o Micro$ofcie — o tym, jak jego rozwiązania są denne i beznadziejne i dlaczego ludzie z nich korzystają.**

Czym są DirectX i OpenGL? Ano, są to — w skrócie — biblioteki głównie do renderowania 3D (i nie tylko, ale o tym za chwilę). DirectX jest zamknięty i od podstaw stworzony przez M$, natomiast OpenGL jest otwarty i stworzony przez Silicon Graphics, jednego z członków grupy ARB, czyli zrzeszenia kilku dużych firm komputerowych — m.in. Intel, NVIDIA, IBM, Apple i ATI. ARB jest aktualnie wcielone w grupę Khronos.

No, to jedziemy!

- DirectX jest dostępny tylko i wyłącznie na Windowsa (w tym również na jego Xboksowe i mobilne odmiany), natomiast OpenGL dostępny jest na Windowsa, Mac OS-a, Linuksa, PlayStation 3 i wszystkie inne popularne systemy (w tym systemy mobilne), ponadto posiada bindy na kilkadziesiąt języków programowania.
- Starsze Windowsy nie obsługują nowych DirectXów. Wszystkie obsługiwane przez OpenGLa systemy, nawet bardzo stare obsługują najnowszego OpenGL-a.
- OpenGL jest (często — zależnie od implementacji) szybszy.
- W OpenGL-u najnowsze technologie są implementowane wcześniej niż w DirectXie dzięki rozszerzeniom — np. technologia teselacji, która jest tak reklamowana przez Microsoft, producentów gier, czy czasopisma była dostępna w OpenGL-u 3 lata wcześniej.
- Microsoft straszy użytkowników przekonując ich do DirectXa — tzw. [FUD](http://pl.wikipedia.org/wiki/FUD).
- Mylące kampanie reklamowe — niektórzy użytkownicy przekonywali się do nowego DirectX-a, podczas gdy to samo można było uzyskać w starym edytując plik konfiguracyjny.
- OpenGL był początkowo lepiej napisany, potem Microsoft zaczął podkradać rozwiązania otwartej biblioteki.
- Łatwiej jest się nauczyć obsługi OpenGL-a niż DirectX-a. Do OpenGL-a jest ponadto więcej tutoriali i poradników w internecie.
- OpenGL jest biblioteką do grafiki, DirectX daje też rozwiązania do kontrolerów, internetu, dźwięku i tekstu — bezsens. (potem wydajność marna, a jak to mówią — "co jest do wszystkiego, to jest do niczego")
- DirectX-a trzeba zazwyczaj dodatkowo doinstalować. OpenGL jest dostarczany wraz ze sterownikami do karty graficznej.
- Żeby pisać aplikacje wykorzystujące OpenGL, nie trzeba nic doinstalowywać — odpowiednie nagłówki są instalowane zazwyczaj wraz z kompilatorem. Do DirectX-a trzeba instalować SDK.
- OpenGL działa na zasadzie klient-serwer, co oznacza, że można wyświetlić grafikę wyświetlaną przez OpenGL np. na zdalnym terminalu. DirectX nie daje takiej możliwości.

**Wniosek?**

DirectX to kolejny, beznadziejny badziew od Micro$oftu, który zdobył popularność tylko dzięki marketingowi i zastraszaniu klientów. OpenGL daje większe możliwości, jest prostszy w obsłudze, działa na wszystkich najpopularniejszych platformach i jednocześnie jest szybszy od DirectX-a. OpenGL daje lepszą grafikę nawet na starszym sprzęcie — głównie ze względu na kompatybilność DirectX-a nie tylko z samym Windowsem, ale również określonymi jego wersjami i kartami graficznymi. Producenci gier olewają takie platformy jak Linux pomimo tego, że skoro robią porty swoich gier na PlayStation 3 lub na Mac OS-a to oznacza to, że na Linuksa też się da.

Po raz n-ty z kolei można się przekonać, że ogromna korporacja, jeżeli tylko ma dużo kasy, może nic nie robiąc wywołać więcej szumu niż globalna społeczność, która odwala masę niesamowicie dobrej roboty w tempie dużo szybszym od Microsoftu. Daje masę własnościowego software'u zabijając to, co jest jednak lepsze. A ludzie i tak z tego korzystają. Idioci.
