---
title: 'Bezpieczeństwo popularnych protokołów komunikacji'
permalink: 'bezpieczenstwo-popularnych-protokolow-komunikacji'
category: 'Porady'
tags: ['Gadu-Gadu', 'Xfire', 'Wireshark', 'Sniffer', 'Skype', 'XMPP', 'Jabber', 'bezpieczeństwo', 'szyfrowanie', 'SSL']
language: pl
date: 2012-01-14 19:32:17+0000
---

**Bezpieczeństwo to aktualnie temat bardzo popularny. Wszyscy zabezpieczają swoje komputery jak mogą, wszelkimi antywirusami, firewallami i wszelkimi programami z "anty" w nazwie. Okazuje się jednak, że niebezpieczeństwo czyha bliżej, niż mogłoby się wydawać, a konkretnie w komunikatorach internetowych.**

Okazuje się bowiem, że podejrzeć rozmowy prowadzone z innymi osobami jest wręcz idiotycznie prosto. Wystarczy program zwany snifferem. Ten wpis pokaże, jak posługiwać się znanym narzędziem tego typu — Wiresharkiem, jak podejrzeć rozmowę, oraz wskaże, które sieci są bezpieczne, a które nie.

## Pierwsze kroki z Wiresharkiem

Po instalacji wspomnianego programu (nie będę tego opisywał, każdy sobie z tym poradzi...), uruchamiamy go. Warto wspomnieć, że w systemach Linuksowych może być konieczne uruchomienie go jako użytkownik z najwyższymi uprawnieniami — jako root. Warto również pozamykać wszelkie inne aplikacje, które korzystają z internetu — dzięki temu łatwiej będzie odszukać cechy charakterystyczne dla danego protokołu, czy aplikacji — np. port komunikacji, czy docelowy adres IP. Tak więc po uruchomieniu Wiresharka ujrzymy takie oto okno:

[![Wireshark — interfejs](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_1.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_1.jpeg)

## Podsłuchiwanie rozmowy z Gadu-Gadu

Po zaznajomieniu się z interfejsem, czas przejść do śledzenia rozmowy. Klikamy więc przycisk *Start a new live capture* (w wersji 1.6.2 jest to 3 ikonka od lewej na toolbarze). Wysyłamy teraz jakiś spam — np. do Infobota lub do siebie:

[![Wireshark — podsłuchiwanie rozmowy](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg.jpeg)

Jak widać, powtarza nam się dosyć często "gadugadu" (nawet logiczne, ale później pokażę, że nie zawsze tak jest). Zatrzymujemy więc przechwytywanie, klikając przycisk *Stop the running live capture*. Klikamy na jakiś wpis zawierający frazę "gadugadu". Okazuje się, że jest to port, konkretnie 8074:

[![Wireshark — podsłuchiwanie rozmowy z Gadu-Gadu](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg_2.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg_2.jpeg)

W widoczny na górze pasek "Filter" wpisujemy więc *tcp.port == 8074* i ponownie uruchamiamy przechwytywanie. Znowu piszemy sobie do Infobota jakiś spam i obserwujemy okno Wiresharka. Po wysłaniu kilku wiadomości, odnajdujemy je w widocznych pozycjach na liście w Wiresharku. Można je bowiem bez problemu odczytać:

[![Wireshark — podsłuchiwanie rozmowy GG](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg_3.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_gg_3.jpeg)

Był to jeden z banalnych przykładów, bowiem od razu widoczna była fraza "gadugadu" jako port. Teraz pokażę, jak sobie radzić w trudniejszych warunkach, kiedy nie widać takiego "gadugadu" — czyli będziemy podsłuchiwać rozmowę z Xfire.

## Podsłuchiwanie rozmowy z Xfire

Na tym kroku jeszcze bardziej zachęcam do wyłączenia absolutnie wszystkich zbędnych aplikacji, powyłączania zbędnych protokołów w komunikatorze itp. Dzięki temu łatwiej będzie wykonać poniższe kroki.

Uruchamiamy więc nowe przechwytywanie. (Pamiętaj o wyczyszczeniu pola "Filter"!)

[![Wireshark — przechwytywanie rozmowy z Xfire](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_xfire_1.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_xfire_1.jpeg)

Jak widać, dużo pakietów jest przesyłanych na porcie 25999. I tutaj widać, jak ważne jest odizolowanie środowiska działania Wiresharka — dzięki zamknięciu zbędnych aplikacji, mamy niewiele pakietów, które mogą być "podejrzane". Robimy więc analogicznie do podsłuchiwania rozmowy z GG — filtrujemy pakiety: *tcp.port == 25999*. Uruchamiamy przechwytywanie. Okazuje się, że w tym przypadku również można bezproblemowo wszystko podsłuchać:

[![Wireshark — podsłuchiwanie rozmowy z Xfire](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_xfire_2.jpeg)](/static/images/blog/2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji-wireshark_xfire_2.jpeg)

## Jakie protokoły są więc bezpieczne?

Nie przetestowałem ich zbyt wiele, a wręcz bardzo mało. Udało mi się jednak ustalić, że następujące protokoły:

- Skype
- XMPP (Jabber)

korzystają z szyfrowania. Natomiast opisywane we wpisie Gadu-Gadu oraz Xfire z szyfrowania nie korzystają, co pozwala na łatwe podsłuchiwanie konwersacji.

## Zakończenie

Mam nadzieję, że ten wpis Ci się przyda — dzięki niemu może zaczniesz używać bezpiecznych, bardzo dobrych protokołów — np. polecany przeze mnie XMPP. Dowiesz się samodzielnie, które protokoły bezpieczne nie są, a dzięki znajomości obsługi Wiresharka możesz się też dobrze bawić. 😄
