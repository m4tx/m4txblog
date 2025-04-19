---
title: 'Pierwsze kroki z Linuksem — instalacja'
permalink: 'pierwsze-kroki-z-linuksem-instalacja'
category: 'linux'
tags: ['debian', 'pierwsze kroki', 'linus torvalds', 'Pierwsze kroki']
language: pl
date: 2010-10-05 20:13:02+0000
---

## Pierwsze kroki z Linuksem — instalacja

Linux to otwarty system operacyjny stworzony pierwotnie przez Linusa Torvaldsa, dziś rozwijany przez wielu ludzi na całym świecie. Postanowiłem przejść całkowicie na Linuksa i relacje z tego opiszę na blogu. Miał to być cykl 7 wpisów prowadzony przez tydzień, aktualnie się trochę pozmieniało, jedno jest jednak pewne — będzie to cykl.

### Debian

Dystrybucją Linuksa, którą wybrałem jest — jak można się domyślić po obrazku u góry — [Debian](http://www.debian.org/). Jest to dystrybucja darmowa i bardzo popularna. Wybrałem ją ze względu na możliwości, a najbardziej jednak fakt, że jest to system przeznaczony dla doświadczonych Linuksowców. Co prawda Linuksa "liznąłem" do tej pory średnio, lecz nic nie stoi na przeszkodzie, aby takiej dystrybucji używać — w końcu jestem dobry w sprawach IT. Zapraszam wszystkich do czytania — może ktoś się nawet skusi używać Linuksa?

### Instalacja

Mogę powiedzieć od razu, że nie obyło się bez problemów. Instalacja jest co prawda, jak to wyczytałem w internecie bardzo prosta i rzeczywiście taka jest. Jednak bardziej chodzi o problemy z samym systemem...

Instalacja trwa około 15 minut. Rozpocząłem ją jeszcze w sobotę. Przebiegła ona z problemami, a pierwszym z nich było umieszczenie bootmanagera na dyskietce. Jednak po tym procesie system nie uruchomił się, wyświetlał się błąd. Ostatecznie zdecydowałem się na zainstalowanie GRUB-a (to jeden z bootloaderów używanych w Linuksie) na dysku twardym i powiem szczerze, że jak tak teraz piszę ten tekst spod Linuksa to myślę, że to nie był błąd.

Uruchomiłem system. Szybko okazało się, że nie ma on zainstalowanego środowiska graficznego. Włączyłem więc Windowsa i dowiedziałem się tego i owego o podstawowej konfiguracji Debiana. Przełączając się ponownie na Linuksa zauważyłem także, że system posiada środowisko graficzne GNOME, a nie tak jak chciałem — KDE! Wersję z KDE zdobyłem dopiero w poniedziałek, więc i w tym dniu rozpocząłem ponowną instalację. Tym razem instalator automatycznie zainstalował środowisko graficzne, jednak pytając się wcześniej o to. W wersji z GNOME nie miałem takiego wyboru.

### Po instalacji

Po instalacji już właściwego systemu, różnica w wydajności była ogromna. System uruchamiał się oraz zamykał w przeciągu 15 sekund. Nie ma nawet co porównywać tego wyniku z Windowsem — około dwie minuty na uruchomienie i minuta na zamknięcie. Tak, to prawda, że to wyniki z "zawalonego" systemu, ale z tego co przynajmniej słyszałem, w Linuksie nie powinno być dużej różnicy również przy dużej ilości zainstalowanych programów.

Niestety, KDE jest po angielsku. Co prawda język angielski nie jest dla mnie dużym problemem, jednak łatwiej by było porozumiewać się z systemem w języku ojczystym.

Po instalacji nie miałem połączenia z internetem. Zanim jednak skonfigurowałem je, zaznajomiłem się nieco z Linuksem, a dokładniej z konsolą.

Kolejne przełączenie na Windowsa. Zassałem odpowiedni program, dociągnąłem zależności i wreszcie miałem działający internet w Debianie. I — co też ważne — system łączył się w czasie około jednej sekundy — w Windowsie było to około 10 sekund oraz około minuty na uruchomienie programu do tego...

W systemie znalazłem przeglądarkę internetową Konqueror, która jest domyślnie wbudowana w KDE, konsolową w3m oraz zmodyfikowaną wersję Firefoksa pod nazwą Iceweasel. Wybrałem tą ostatnią i to właśnie z niej piszę ten wpis. Chciałem pobrać Firefoksa 4 beta 6, jednak był problem z zależnościami — po prostu brakowało mu jakiegoś pliku. Zrezygnowałem i odłożyłem Firefoksa na inny dzień.

Komunikator Tlen, którego używałem w Windowsie, ma swoją wersję na Linuksa, nie było więc żadnego problemu. Problem był natomiast z Xfire, który to już swojej Linuksowej wersji nie ma. Chciałem zainstalować Pidgina z wtyczką gfire, ale... Ech, te zależności!

## Zakończenie

No tak, jak zwykle chciałem napisać więcej, a wyszło jak wyszło... I jeszcze to zakończenie w tym miejscu... No nic, może innym razem będzie lepiej! Prawdopodobnie jutro albo pojutrze napiszę kolejny wpis o Linuksie. Pozdro!
