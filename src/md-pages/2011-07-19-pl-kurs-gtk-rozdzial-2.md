---
title: 'Kurs GTK+ – rozdział 2'
permalink: 'kurs-gtk-rozdzial-2'
category: 'wstęp'
tags: ['programowanie', 'GTK+', 'Kurs GTK+', 'kurs', 'GUI', 'okno']
language: pl
date: 2011-07-19 10:00:45+0000
---

# 2. Hello world w GTK+

## 2.1. Utworzenie pustego okna

No — więc jak już zainstalowałeś GTK+, czas przystąpić do kodzenia!

Uruchom więc swój ulubiony IDE i utwórz nowy, pusty projekt. Ja będę korzystał z Code::Blocks i kompilatora GCC.

Utwórz nowy plik źródłowy C++. Ja go nazwałem `main.cpp`. Wklej do niego następujący kod:

```cpp
#include <gtk/gtk.h>

int main (int argc, char *argv[]) {
    GtkWidget *okno;

    gtk_init (&argc, &argv);

    okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
    gtk_widget_show (okno);

    gtk_main ();

    return 0;
}
```

Następnie go skompiluj z następującą opcją kompilatora:

```
`pkg-config gtk+-2.0 --cflags`
```

Oraz linkera:

```
`pkg-config gtk+-2.0 --libs`
```

Jeżeli wszystko skonfigurowałeś poprawnie, program powinien się bezproblemowo skompilować. Po uruchomieniu powinieneś ujrzeć coś takiego:

[![Puste okno programu](/static/images/blog/2011-07-19-pl-kurs-gtk-rozdzial-2-kursgtk_02_scr01.png)](/static/images/blog/2011-07-19-pl-kurs-gtk-rozdzial-2-kursgtk_02_scr01.png)

Gratulacje! To Twój pierwszy program napisany z użyciem GTK+!

## 2.2. Interpretacja kodu

No ale fajnie by było wiedzieć, co powyższy kod właściwie robi. Już tłumaczę:

```cpp
#include <gtk/gtk.h>
```

Dołączamy odpowiedni plik nagłówkowy GTK+.

```cpp
GtkWidget *okno;
```

Tworzymy referencję do widżetu okna.

```cpp
gtk_init (&argc, &argv);
```

Inicjujemy GTK+ i przekazujemy parametry, z których został uruchomiony program.

```cpp
okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
```

Tworzymy okno programu. Argument `GTK_WINDOW_TOPLEVEL` określa, że okna ma zostać utworzone z dekoracjami oraz z domyślnym rozmiarem 200x200 px.

```cpp
gtk_widget_show (okno);
```

Wyświetlamy okno programu.

```cpp
gtk_main ();
```

Uruchamiamy główną pętlę GTK+.

Jak widać utworzenie pustego okna w GTK+ jest bardzo proste. W następnej części kursu opiszę dodawanie widżetów, takich jak przyciski, czy pola tekstowe do okna.
