---
title: 'Windows 8 Consumer Preview — pierwsze wrażenia'
permalink: 'windows-8-consumer-preview-pierwsze-wrazenia'
category: 'zmiany'
tags: ['microsoft', 'windows', 'linux', 'firefox', 'pierwsze wrażenia', 'desktop', 'windows 8', 'metro', 'kafelki', 'aero', 'system', 'os', 'tablet', 'bsod', 'Pierwsze kroki']
language: pl
date: 2012-04-08 11:25:12+0000
---

**Microsoft udostępnił jakiś czas temu wersję Consumer Preview swojego najnowszego systemu operacyjnego znanego szerzej jako Windows 8. Postanowiłem więc go pobrać i przetestować.**

I tak przy okazji — jakby ktoś chciał również przetestować, to system znajdzie [tutaj](http://windows.microsoft.com/en-US/windows-8/iso).

Dodam od razu, że wspomniany system został testowany przeze mnie na wirtualnej maszynie, a konkretnie [VirtualBoksie](https://www.virtualbox.org/).

Zacznijmy więc od początku, czyli od instalatora. Ten bowiem nie zmienił się bardzo względem poprzedniej wersji. Wygląda niemalże tak samo, zmieniła się praktycznie tylko kolorystyka i logo, które szczerze mówiąc, bardzo mi się nie podoba. Swoją drogą, testując dalej system, odniosłem wrażenie, że firma MałyMiętki chce na siłę uczynić ten system brzydkim, ale o tym już za chwilę.

[![Windows 8 CP — instalator](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-instalator-1.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-instalator-1.jpeg)

Po przejściu do partycjonowania i tworzeniu nowej, pustej partycji, system wyświetlił komunikat, że może utworzyć dodatkową partycję na jakieś systemowe pliki... i rzeczywiście, utworzył:

[![Windows 8 CP — instalator — dodatkowa partycja](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-instalator-2.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-instalator-2.jpeg)

Szczerze mówiąc, nie mam pojęcia, co na tej partycji jest i będzie trzymane. I sam wątpię w użyteczność tej dodatkowej partycji. Dalsza część instalacji przebiegła podobnie jak w przypadku Windowsa 7. Jedyną istotną rzeczą, jaką zauważyłem, jest to, że instalator już nie kopiuje całej wieczności pierwszego procenta plików (jak to zauważyłem w przypadku Windowsa 7), a potem następne 99% bardzo szybko, a wyświetla ten postęp bardziej płynnie. Kilka restartów wirtualnej maszyny później zobaczyłem taki oto ekran:

[![Windows 8 CP — wybór koloru tła](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-personalizacja.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-personalizacja.jpeg)

System zapytał, jaki kolor bym chciał mieć jako swoje tło. Problem w tym, że wszystkie są bardzo brzydkie! Zostawiłem więc domyślny — wg. mnie najładniejszy, ale i tak strasznie brzydki... Następnie zalogowałem się do swojego konta Microsoft, które utworzyłem... nie pamiętam kiedy, i ujrzałem ekran nowego interfejsu Metro.

[![Windows 8 CP — interfejs Metro](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-metro.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-metro.jpeg)

Czyli — inaczej mówiąc — do interfejsu opartego na kafelkach ~~łazienkowych~~. Po kilku minutach użytkowania na typowym komputerze — desktopie, zamiast tabletu, pod który dedykowany jest ten interfejs odczułem, jakie Metro jest niewygodne. Z ciekawości wszedłem w panel zmiany avatara. Problem w tym, że nie potrafiłem z niego wyjść. Brak jest jakiegokolwiek przycisku do zamykania. Użyłem Alt+F4, potem odkryłem, że należy przytrzymać kursor przy górnej krawędzi ekranu, wcisnąć lewy przycisk myszy, a następnie przeciągnąć ją w dół. Na tabletach to pewnie będzie w miarę dobre rozwiązanie. Na desktopach? Absolutnie nie. Potem przetestowałem jeszcze Internet Explorera w interfejsie ~~Pociąg~~ ~~Tramwaj~~ ~~Lokomotywa~~ Metro, po czym przeszedłem już do interfejsu znanego z Visty i Windows 7, czyli Aero.

[![Windows 8 CP — Internet Explorer w interfejsie Metro](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-ie-metro.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-ie-metro.jpeg)

Uruchomiłem tam Internet Explorera, który ma już przy nazwie dopisaną liczbę 10 i przekonałem się, że ta "przeglądarka" wciąż nadaje się tylko do pobrania innej... Wciąż działa niemiłosiernie ociężale (zwłaszcza otwieranie nowych kart), a standardowy interfejs, czyli pasek adresu i kart połączone, to jakieś totalne dno (swoją drogą, zmieniłem to później). Pobrałem więc ~~Wilka otaczającego Ziemię~~ Firefoksa. Tutaj Windows chciał zabłysnąć inteligencją... Ale mu nie wyszło. Powiedział mi bowiem, że posiadam nowe aplikacje, które potrafią otwierać strony internetowe:

[![Windows 8 CP — instalacja Firefoksa](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-firefox.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-firefox.jpeg)

I tutaj pasuje tylko jeden obrazek...

[![No co ty nie powiesz?!](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-you-dont-say.png)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-you-dont-say.png)

No, ale mniejsza z tym. Chciałem potem zainstalować guest additions i... Windows się wykrzaczył.

[![Windows 8 CP — BSoD](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-bsod_win8.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-bsod_win8.jpeg)

Nie uważacie, że stare BSoD-y były lepsze? Zakorzeniły się w Windowsach od samego ich początku, przez wszystkie wydania wyglądały niemal tak samo... A tu nagle taka zmiana. A tak swoją drogą — system miał się zrestartować, jednak tego nie zrobił. Musiałem to zrobić własnoręcznie. Tymczasem chciałem zobaczyć, czy coś zostało zmienione w głównych aplikacjach. Oprócz wstążki w eksploratorze:

[![Windows 8 CP — odświeżony eksplorator](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-wstazka-eksplorator.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-wstazka-eksplorator.jpeg)

Zmian — i to dość dużych — doczekał się menedżer zadań. Po wciśnięciu Ctrl+Shift+Esc ukazuje się bowiem naszym oczom takie oto okno:

[![Menedżer zadań](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan.jpeg)

Wpierw, gdy to zobaczyłem, pomyślałem sobie "Eee, co to jest!...". Jednak potem wcisnąłem "More details" i oczom moim ukazał się (w miarę) zaawansowany menedżer zadań, z ładnym przedstawieniem dużych ilości informacji na temat działających procesów, usług i kilku innych rzeczy.

[![Windows 8 CP — menedżer zadań — procesy](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan-2.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan-2.jpeg)
[![Windows 8 CP — menedżer zadań — zasoby](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan-3.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-menedzer-zadan-3.jpeg)

Kolejną rzeczą, jaką przetestowałem (powierzchownie) jest Windows Store. Jest on — moim zdaniem — średnio wygodny i brakuje w nim chociażby wyszukiwarki.

[![Windows Store — lista aplikacji](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-1.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-1.jpeg)

Instalowanie programów jest natomiast bardzo wygodne: wystarczy wcisnąć "Install", odczekać chwilę i już można uruchomić aplikację. Dodatkowym atutem jest tu również możliwość zapauzowania instalowania aplikacji, czego brakuje chociażby w Centrum Oprogramowania Ubuntu (jest możliwość przerwania i późniejszego wznowienia zamiast prostego zapauzowania).

[![Windows Store — wybór aplikacji](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-2.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-2.jpeg)
[![Windows Store — instalacja aplikacji](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-3.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-windows-store-3.jpeg)

Powróciłem z powrotem do interfejsu Aero. Tutaj uruchomiłem Firefoksa. Chciałem zainstalować Chrome'a (w celu przetestowania) i... Oczom moim ukazał się po raz kolejny ukochany ekran znany szerzej jako Blue Screen of Death. Tym razem jednak udało mu się zrestartować wirtualną maszynę.

[![Windows 8 CP — kolejny BSoD](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-bsod-win8-2.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-bsod-win8-2.jpeg)

Tak swoją drogą — z ciekawości pobrałem najnowsze IDE od Microsoftu zwane Visual 11 Express Beta. Pozwala ono na tworzenie aplikacji w stylu Metro. Jednak... póki co, nie rozkminiłem jak to działa (i pewnie nie rozkminię 😜). Moim zdaniem nowy Visual jest brzydszy niż poprzednie, prawie cały jest wypełniony jednym kolorem... W dodatku chyba nie pozwala już na tworzenie w prosty sposób standardowych aplikacji (nieopartych na Metro), zresztą — tworzenie programów z nowym interfejsem również nie odbywa się w prosty sposób, tzn. przez przenoszenie kontrolek na okno. W każdym razie nie dopatrzyłem się czegoś takiego, aczkolwiek nie zagłębiałem się za bardzo w nowe IDE.

Powracając teraz do standardowo zainstalowanych aplikacji — nie ma żadnych nowości, wydaje mi się nawet, że aplikacji jest mniej, niż w Windows 7. Przetestowałem tylko kilka z nich, bo uruchamianie ich z poziomu interfejsu Metro to jakaś tragedia (przytrzymaj kursor myszy w lewym dolnym rogu ekranu, kliknij LPM, kliknij PPM, wciśnij "All apps", poszukaj lub wpisz kawałek nazwy programu programu i kliknij nań... Zamiast \[Win\] -\> kilka pierwszych liter -\> Enter, jak to w Win7 jest...) i powiem tak: notatnik tak samo beznadziejny jak w poprzednich wydaniach (on chyba pozostał niezmieniony od Windowsa 95...), Paint taki jak w Win7, w Windowsie Media Playerze też za wiele się nie zmieniło... 3 aplikacje. Każdą z nich zastępuje się lepszymi odpowiednikami zaraz po instalacji systemu. Wciąż są beznadziejne i nie nadają się prawie do niczego... Microsoft dołożył wszelkich starań, aby nic nie zrobić z tym. 😜

Zapragnąłem wreszcie zamknąć system. I teraz pojawia się problem: jak? W poprzednich wersjach systemu Windows wystarczyło wejść w menu Start i kliknąć Zamknij. A tutaj? No właśnie. Tutaj należy przytrzymać kursor myszy w prawym górnym rogu ekranu, wcisnąć "Settings" (tak! "Settings"! "Ustawienia"!), następnie "Power" i "Shut Down". Bardzo intuicyjnie.

Widzę, że wpis ma już prawie 1000 słów, tak więc będziemy zbliżać się do końca. Tak więc krótkie podsumowanie: Windows 8 ma szanse zdobyć popularność na tabletach dzięki interfejsowi Metro i ogromnej bazie oprogramowania. Na desktopach? Na pewno nie. Połączenie interfejsu Metro z Aero to jakiś kompletnie poroniony pomysł. Najnowszy system w ogóle nie nadaje się na dekstopy, jest niewygodny w obsłudze, dostęp do wszystkiego został skutecznie utrudniony. Mówi się, że co drugi system Microsoftu jest dobry. Windows XP był dobrym systemem, potem nadeszła Vista o wątpliwej sławie, potem — również całkiem dobry system — Windows 7, a teraz znów nadchodzi system, którego nie pokocha społeczność — Windows 8. Dobrze, że już przeszedłem na Linuksa, a na Windowsa nie zamierzam wracać 😄

A tak btw. — w startup screenie i na domyślnej tapecie jest taka fajna rybka. Zastanawia mnie, co może ona znaczyć. Może Microsoft uznał, że aktualne logo jest zbyt beznadziejne i zrobił takie alternatywne logo w postaci rybki?

[![Windows 8 CP — domyślna tapeta](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-rybka-1.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-rybka-1.jpeg)
[![Windows 8 CP — domyślna tapeta](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-rybka-2.jpeg)](/static/images/blog/2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia-rybka-2.jpeg)
