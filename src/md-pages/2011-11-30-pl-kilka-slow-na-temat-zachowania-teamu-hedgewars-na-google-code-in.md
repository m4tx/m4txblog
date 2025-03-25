---
title: 'Kilka słów na temat zachowania teamu Hedgewars na Google Code-in'
permalink: 'kilka-slow-na-temat-zachowania-teamu-hedgewars-na-google-code-in'
category: 'gnome'
tags: ['Google Code-in', 'Google Code-in 2011', 'mentor', 'hedgewars', 'backz', 'Przemyślenia']
language: pl
date: 2011-11-30 18:09:24+0000
---

**Całkiem niedawno pisałem o tym, że udało wykonać mi się pierwsze zadanie na Google Code-in 2011. Od razu zechciałem zabrać się za następne, jednak team Hedgewars skutecznie mi to utrudnił...**

Zacznijmy od tego, że najpierw chciałem wykonać 2 inne zadania (1 od GNOME, drugie od Hedgewars). Mniejsza z tym, w tym momencie interesuje mnie zadanie [backz](http://www.google-melange.com/gci/task/view/google/gci2011/7140256). Zacytuję teraz opis tego zadania:

> Unify the back button functionality, avoid using the save button in the settings page.

Co oznacza mniej więcej:

> Zunifikuj funkcjonalność przycisku powrotu — usuń przycisk zapisywania w oknie ustawień.

I tutaj zaczyna się moja historia:

GNOME przygotowało swoje zadania: wypisało wymagania, opisało dokładnie zadanie, zamiast — jak tutaj — wstawić lakoniczny prawie nic nie wyjaśniający opis — i to mi się podoba, i podejrzewam, że wielu innym członkom konkursu również. A tutaj nawet nie wiadomo o co chodzi: czy chodzi o projektowanie — można to wywnioskować po tym, że brak jest tagów w opisie tego zadania na oficjalnej stronie Hedgewars — czy może o pisanie kodu — tego można się domyślić po... tagach na stronie Google Code-in. Eh...

OK, przejdźmy więc może dalej: udało mi się wykonać to zadanie w kilka godzin. Wysłałem plik `.patch`. Potem po kolejnych kilku godzinach łaskawie PMKNMZ (Pewien Mentor Którego Nie Musicie Znać) (po prostu nie chcę podawać nicku...) odpisał mi na IRC-u, że "aplikowanie tego patcha to męka" i "ustawienia uzbrojenia nie zapisują się" (to również zostało przetłumaczone)... A u mnie się zapisywały. Argh.

Mało tego! Kolejne próby skontaktowania się z wyżej wspomnianym PMKNMZ zakończyły się niczym, podobnie jak kilka poprzednich. Z jego rozmowy z innym użytkownikiem IRC-a wywnioskowałem, że...

Poszedł grać w Minecrafta.

Następnego dnia dowiedziałem się o pewnej opcji diffa, która generuje patch w zunifikowanym standardzie. Oczywiście PMKNMZ mi o nim nie powiedział, bo po co. Lepiej powiedzieć, że aplikowanie patcha to męka i pójść grać.

Po wysłaniu nowego patcha i napisaniu szczegółowej instrukcji dla kompletnych idiotów jak go zaaplikować, a następnie odczekaniu kolejnych kilku godzin ten sam mentor powiedział, że... źle wykonałem zadanie, ponieważ zamieniłem jedną zmienną w klasie z prywatnej (private) na publiczną (public). PMKNMZ powiedział, żebym dodał odpowiednie instrukcje do metody `GoBack()`.

I tutaj kończy właśnie się historia z zadaniem backz. Dałem sobie spokój. Praca z kodem pisanym przez kilka osób, nieposiadający żadnych wytycznych formatu (każdy plik pisany innym stylem), mentorami, którzy wolą grać w Minecrafta zamiast pomóc uczestnikom konkursu nie jest zbyt przyjemna. Tym samym kończę ten wpis i radzę Wam, drodzy czytelnicy:

Nie wykonujcie zadań Hedgewars. Czeka Was napisany wieloma stylami kod, nieprzyjaźni mentorzy oraz męka przez długi czas, przez który zdążylibyście wykonać kilka innych zadań.

Pozdrawiam...
