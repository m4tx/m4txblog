---
title: 'Kurs GTK+ — rozdział 1'
permalink: 'kurs-gtk-rozdzial-1'
category: 'wstęp'
tags: ['programowanie', 'windows', 'linux', 'GTK+', 'Kurs GTK+', 'kurs', 'instalacja', 'GUI']
language: pl
date: 2011-07-18 09:20:47+0000
---

[![Logo GTK+](/static/images/blog/2011-07-18-pl-kurs-gtk-rozdzial-1-gtkplus_logo.png)](/static/images/blog/2011-07-18-pl-kurs-gtk-rozdzial-1-gtkplus_logo.png)

Z racji, że piszę aktualnie kurs biblioteki GTK+ dla pewnej strony WWW (nie, nie będę robił reklamy), postanowiłem również opublikować go również na moim blogu. Zapraszam do czytania. 😄

# 1. Wstęp

## 1.1. Czym jest GTK+?

GTK+ to biblioteka służąca do budowania GUI – graficznego interfejsu użytkownika. Pierwotnie została stworzona na potrzeby programu GIMP, stąd też nazwa — _The **G**IMP **T**oolkit_. Znak **+** pojawił się w nazwie, gdy autorzy dodali do GTK możliwość programowania obiektowego.

Biblioteka GTK+ została napisana w C, ale w oparciu o implementację obiektowości dla C – GObject. Dla C++ istnieje biblioteka gtkmm — nakładka na GTK+ wprowadzającą wygodniejszą składnię w stylu C++. Ten kurs będzie się skupiał na GTK+. Gtkmm zostanie omówiony w następnych rozdziałach.

Do GTK+ zostało stworzonych wiele bindów na różne języki programowania, m.in. dla C# — Gtk#, dla Pythona – PyGTK, czy nawet do PHP – PHP-GTK. GTK+ można używać na wszystkich popularnych systemach operacyjnych – Windows, Linux, Mac OS, Solaris, BSD i wielu innych. To wszystko czyni bibliotekę GTK+ bardzo uniwersalną.

## 1.2. Instalacja

GTK+ można pobrać z oficjalnej strony projektu. W przypadku Linuksa sprawa jest zazwyczaj dużo prostsza dzięki wykorzystaniu repozytoriów. W dystrybucjach opartych na Debianie (m.in. Ubuntu) wystarczy bowiem w terminalu – jako root – wydać polecenie: <span style="font-family: Courier New,monospace;">apt-get install libgtk2.0-dev</span> lub zainstalować pakiet <span style="font-family: Courier New,monospace;">libgtk2.0-dev</span> dowolnym menedżerem pakietów. W innych dystrybucjach będzie zapewne podobnie. W przypadku Windowsa natomiast sprawa wygląda nieco trudniej, ponieważ GTK+ trzeba ręcznie pobrać i zainstalować. Najprościej skorzystać z tzw. All-in-one-bundle, który – dla systemów 32-bitowych – można pobrać [stąd](http://ftp.gnome.org/pub/gnome/binaries/win32/gtk+/2.22/gtk+-bundle_2.22.1-20101227_win32.zip), oraz – dla systemów 64-bitowych – [stąd](http://ftp.gnome.org/pub/gnome/binaries/win64/gtk+/2.22/gtk+-bundle_2.22.1-20101229_win64.zip). All-in-one-bundle ma tylko jedną wadę – nie jest w najnowszej wersji.

Drugim sposobem wejście na [stronę downloadu GTK+](https://www.gtk.org/docs/installations/), wybór systemu, a następnie pobranie biblioteki i wszystkich zależności do niej.

Wersja GTK+ użyta w tym kursie to 2.24.4. Możesz używać starszej wersji – 2.22.1. Warto wspomnieć o GTK+ 3.0 – nie opisałem jej tylko ze względu na to, że nie jest tak rozpowszechniona i łatwo dostępna na wszystkie systemy operacyjne, jak GTK+ 2.x.
