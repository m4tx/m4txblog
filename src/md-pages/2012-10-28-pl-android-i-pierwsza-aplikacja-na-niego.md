---
title: 'Android i pierwsza aplikacja na niego'
permalink: 'android-i-pierwsza-aplikacja-na-niego'
category: 'Nowe aplikacje'
tags: ['programowanie', 'SmsPrice', 'Sms Premium', 'windows', 'linux', 'google', 'j2me', 'Android', 'SMS zwrotny', 'Java Platform Micro Edition', 'netto', 'brutto', 'Eclipse']
language: pl
date: 2012-10-28 16:34:54+0000
---

**Około 2 tygodnie temu stałem się posiadaczem wynalazku zwanego smartfonem z Androidem, a dokładniej Samsunga Galaxy Ace 2. Już po tych dwóch tygodniach (wczoraj konkretnie) zainteresowałem się pisaniem aplikacji na system operacyjny z zielonym robocikiem w logo...**

O samym Androidzie nie będę się zbytnio rozpisywał, bo to nie miejsce na to, jednak powiem co nieco na temat pisania na niego aplikacji. Programowanie bowiem na Androida jest wbrew pozorom bardzo wygodne. O ile na J2ME trzeba było się zazwyczaj męczyć z emulatorami, a potem ręczną instalacją aplikacji na realnym sprzęcie, o tyle w przypadku Androida życie mamy znacznie ułatwione. Wystarczy podpiąć smartfona do komputera, włączyć debugowanie USB (na Windowsie również zainstalować sterowniki...), aby móc wcisnąć "Run" w Eclipsie, a po sekundzie czy dwóch — oglądać wyniki na ekranie urządzenia. Otwieramy sobie dodatkowo okno LogCata w Eclipsie, i...? I mamy podgląd standardowego wyjścia (które jest, jak wiadomo, niezastąpione przy debugowaniu), wyrzucone, niezłapane wyjątki, oraz inne przydatne rzeczy. Ogromny plus jeśli chodzi o tworzenie aplikacji.

Pokrótce mogę również pochwalić Eclipse'a (czy raczej plugin Android Development Tools) za znakomity edytor RAD, pozwalający w prosty sposób zaprojektować GUI nawet złożonych aplikacji, oraz samego Androida za system zarządzania zasobami aplikacji. Zarządza się nimi bowiem za pomocą plików XML — to właśnie w XML-u składowane są aktywności (odpowiedniki okien), ciągi znaków, ścieżki do obrazków i inne.

Jeśli chodzi o spostrzeżenia związane z programowaniem programów i gier na platformę Android, to póki co — tyle, albowiem w tej kwestii nie zagłębiałem się jeszcze zbytnio. Przejdźmy teraz natomiast do właściwej części wpisu, czyli do samej aplikacji...

"Cóż to za aplikacja" — zapewne zastanawiacie się. Patrząc na jej nazwę dochodzę do wniosku, że to już jest chyba tradycja, że portuję ją niemal na każdą nową platformę, na którą uczę się pisać aplikacje. Pewnie już wiecie o czym mówię. Tak, chodzi o aplikację...

**SmsPrice**

której pierwsza wersja została opublikowana na m4txblogu 7 lipca 2010 roku w wersji na Windowsa. 10 lipca 2011 roku został opublikowany jej port na platformę Java Platform, Micro Edition. Teraz powstał natomiast kolejny port, dodający po raz kolejny nowe "ficzery", ponownie na platformę mobilną, tym razem na Androida.

Co zostało ulepszone? Przede wszystkim, dodałem numery 8XXX, 5XXX, oraz 6XXX. Z racji, iż numery 5XXX oraz 6XXX dotyczą SMS-ów przychodzących, dodałem wyświetlanie informacji na temat czy płaci się za SMS-y wysyłane, czy zwrotne. Zamiast starego "Za tego SMS-a zapłacisz: X.XX zł z VAT (X.XX zł bez VAT)", jest "Cena netto: X,XX zł" i "Cena brutto: X,XX zł", w osobnych linijkach. Nie jest już wyświetlana cena SMS-a 0,00 zł; teraz po prostu jest napisane "SMS darmowy". Dodałem przycisk usuwający wpisany numer telefonu oraz okno "O programie" (co prawda było ono w wersji Windowsowej, jednak pozbyłem się go w wersji J2ME; teraz ponownie wraca). Jeśli chodzi o porównanie nowej wersji SmsPrice'a ze starymi, to to by było na tyle. Zapraszam więc tradycyjnie do [downloadu](https://github.com/m4tx/smsprice-android/releases/tag/v1.0) i pozdrawiam 😄
