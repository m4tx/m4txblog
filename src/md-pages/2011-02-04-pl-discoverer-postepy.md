---
title: 'Discoverer — postępy'
permalink: 'discoverer-postepy'
category: 'Zmiany na blogu'
tags: ['Zmiany w aplikacjach', 'spam', 'reCAPTCHA', 'discoverer', 'Akismet']
language: pl
date: 2011-02-04 13:03:29+0000
---

2 tygodnie temu pisałem o rozpoczęciu prac nad projektem trójwymiarowej gry sandboksowej. Wpis ten dotyczył będzie tego, co do tej pory udało mi się stworzyć.

...Zaczynając od silnika. W poprzednim wpisie napisałem, iż silnikiem gry będzie Irrlicht. Po przeczytaniu różnych opinii na forach, po własnych doświadczeniach i spostrzeżeniach zmieniłem Irrlichta, póki jeszcze zbyt dużo nie zrobiłem na Ogre. Otrzymałem sporo problemów z konfiguracją bibliotek, sporo dłuższy, jednak bardziej "normalny" i dający więcej możliwości kod, lepszą grafikę i szybszy (aczkolwiek mam wątpliwości) renderer. Ponadto jeszcze obsługę DirectX 11 (nie, nie wiem po co będzie ona w Discovererze), możliwość kompilacji gry bez zmian w kodzie na platformę iPhone (z tym że takowej wersji i tak nie będzie, ponieważ nie posiadam aktualnie ani iPoda Touch, ani iPhone'a, ani iPada). Pomimo wymienionych wcześniej argumentów, zmianę silnika uważam za plus.

Aktualnie niestety męczę się nad optymalizacją silnika. Nie jest to wcale takie proste, a konieczne, gdyż próba wyświetlenia 10 000 bloków zakończyła się tym, że na mojej wprawdzie nie najnowszej generacji i nie najwydajniejszej, aczkolwiek w zupełności mi wystarczającej karcie graficznej (a konkretnie GeForce 9600GT) zakończyła się zwyczajnym wyświetleniem tych bloków, liczba klatek na sekundę osiągała maksymalnie 2... No cóż, grę trzeba zoptymalizować, aktualnie zastanawiam się nad odpowiednim algorytmem do tego. Możliwe, że sama optymalizacja zajmie lwią część czasu spędzonego przy tworzeniu gry.

Hmmm, co by tu jeszcze... No cóż, screenów na obecną chwilę nie pokażę, bo nawet nie ma czego pokazywać. Jeżeli uda mi się zoptymalizować grę, wyświetlić jakieś bloki i nałożyć na nie jakieś tekstury, to wyślę zrzuty ekranowe na bloga i oczywiście je pokażę, aby nie było, że nic nie robię. 😄

Aha, jeszcze jedno — zamieszczony w poprzednim termin publikacji gry jest... no cóż... zaniżony. 😜 Nie jest wykluczone, a nawet bardzo możliwe, iż gra ukaże się nawet w wakacje, dlatego proszę nie traktować tego terminu jako absolutną regułę, i że jeżeli gra się nie pojawi w tym terminie, to w ogóle jej nie będzie. Postaram się napisać Discoverera tak szybko, jak to będzie możliwe, jednak nie mogę też zapominać o optymalizacji, błędach, no i grę muszę oczywiście dopracować z dbałością o każdy szczegół. O wszelkich pracach będę informował na blogu, a ostateczny termin będzie znany pod koniec prac.

No, to tyle o mojej grze, powiem jeszcze kilka informacji na temat bloga...

Otóż, zamieniłem wtyczkę do obsługi reCaptchy na Akismeta... Hm, tylko czym jest Akismet? Więc, Akismet jest usługą filtrowania spamu. Polega to na tym, że jeżeli zostanie dodany na mojego bloga jakiś komentarz lub trackback, zostanie on najpierw przesłany do serwerów Akismeta. Jeżeli wcześniej jakiś użytkownik otrzymał na swoim blogu identyczny komentarz albo trackback i zostanie oznaczony jako spam, serwery Akismeta dodają go do czarnej listy. Wtedy serwery Akismeta przysyłają do serwerów mojego bloga wiadomość, iż przesłany komentarz to spam, i skrypt mojego bloga zwyczajnie go usuwa. Proste i skuteczniejsze nawet od reCaptchy.

I jeszcze jedna informacja. Planuję w najbliższym czasie (najpóźniej w dniu publikacji Discoverera) otworzyć wersję mojego bloga w języku angielskim. Prawdopodobnie wpisy publikowane w wersji polskiej nie miałyby odpowiedników w wersji angielskiej. Wersja angielska więc będzie stworzona jedynie do publikacji moich programów oraz gier, i... tak, zostanie stworzona tylko ze względu na Discoverera... A właśnie, zapomniałem. Zapomniałem w poprzednim wpisie jeszcze dodać, iż pierwsza wersja Discoverera będzie dostępna tylko w języku angielskim. 😄 Postanowiłem tak ze względu na to, iż sama społeczność polska jest niewielka w porównaniu ze społecznością całego świata — język angielski jest najpopularniejszym językiem. W następnych wersjach prawdopodobnie zrobię jednak system zmiany języków, co oznacza, iż są szansę na Discoverera po polsku.

No, to by chyba było na tyle. Trzymajcie się! 😜
