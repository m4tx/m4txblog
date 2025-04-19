---
title: 'Kurs GTK+ – rozdział 10'
permalink: 'kurs-gtk-rozdzial-10'
category: 'windows'
tags: ['GTK+', 'Kurs GTK+', 'GUI', 'okno', 'motyw', 'styl', 'Raleigh']
language: pl
date: 2011-08-18 18:54:58+0000
---

# 10. Motywy

## 10.1. Główne informacje

W tym rozdziale dowiesz się, w jaki sposób możesz ostylować swoje aplikacje oraz ustawić im standardowy wygląd Windows.

Zapewne zauważyłeś — zwłaszcza, jeżeli używasz Windowsa — że Twoje aplikacje napisane używając GTK+ wyglądają brzydko:

[![Przykładowa aplikacja z motywem Raleigh](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr01.png)

Kanciaste przyciski przypominające te z Windows 9x mogą niezbyt zachęcać do korzystania z aplikacji. Jest to wina domyślnego motywu wbudowanego w GTK+ — jest nim Raleigh. Warto więc wiedzieć, jak można ostylować swoją aplikację. Można w tym celu wykorzystać jeden z dostępnych w internecie motywów lub korzystać z aktualnie ustawionego w systemie motywu. Najpierw zostanie opisana ta druga opcja.

## 10.2. Motyw MS-Windows

Aby swoim aplikacjom wykorzystującym GTK+ nadać wygląd typowej aplikacji Windows, wykonaj kolejno poniższe kroki:

1.  W folderze z plikiem wykonywalnym swojej aplikacji utwórz katalog `share`, a w nim `themes`.

2.  Z folderu, gdzie masz bibliotekę GTK+ skopiuj katalog `bin\share\themes\MS-Windows` do folderu utworzonego w punkcie 1.

3.  W folderze z Twoją aplikacją utwórz katalog `etc`, a w nim `gtk-2.0`.

4.  W utworzonym w poprzednim punkcie folderze utwórz plik tekstowy o nazwie `gtkrc`. (bez rozszerzenia)

5.  Otwórz plik `gtkrc` i wklej do niego:

    ```plain
    gtk-theme-name = "MS-Windows"
    ```

6.  W katalogu z plikiem wykonywalnyn Twojego programu utwórz następującą strukturę katalogów: `lib\gtk-2.0\2.10.0\engines`. (zamiast 2.10.0 możesz mieć inny folder w zależności od używanej wersji GTK+)

7.  Przejdź do folderu, gdzie masz GTK+, a następnie do folderów `lib\gtk-2.0\2.10.0\engines`.

8.  Skopiuj plik `libwimp.dll` do folderu utworzone w punkcie 6.

Gotowe! Po uruchomieniu swojej aplikacji jej wygląd powinien się już stanowczo poprawić:

[![Przykładowa aplikacja z motywem Windows](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr02.png)

## 10.3. Inne motywy

Zawsze najlepszym wyborem będzie domyślny systemowy motyw, ponieważ aplikacja będzie wyglądać tak, jak użytkownik sobie tego zażyczy, jednak ustawienie aplikacji własnego stylu nie stanowi problemu. W tym celu wykonaj poniższe kroki.

Motyw można pobrać z internetu lub stworzyć własny. Ja opisuję Tę pierwszą opcję.

1.  Pobierz dowolny motyw GTK+, np ze strony [GNOME Art](http://art.gnome.org/themes/gtk2). Ja posłużę się motywem `Shiny — Black`.
2.  Wykonaj punkt 1 z poprzedniej porady.
3.  Wypakuj folder z motywem z pobranego archiwum do folderu `share\themes`. Ważne jest, aby w katalogu `share\themes\<nazwa motywu>` znajdował się plik `gtkrc`. Zapamiętaj również nazwę folderu z motywem.
4.  Wykonaj kroki 3-4 poprzedniej porady.
5.  Wykonaj krok 5 z tą różnicą, że `MS-Windows` zamień na nazwę folderu z Twoim motywem.
6.  Wykonaj kroki 6-7 poprzedniej porady.
7.  Wykonaj krok 8, ale zamiast `libwimp.dll` skopiuj plik `libpixmap.dll`.

Aplikacja prezentuje się teraz naprawdę ładnie:

[![Przykładowa aplikacja z motywem Shiny — Black](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-10-kursgtk_10_scr03.png)

To tyle w tym rozdziale. W następnym dowiesz się, do czego służy widżet `GtkTextView` oraz jak z niego korzystać.
