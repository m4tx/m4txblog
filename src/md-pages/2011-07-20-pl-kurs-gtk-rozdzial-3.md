---
title: 'Kurs GTK+ – rozdział 3'
permalink: 'kurs-gtk-rozdzial-3'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'GUI', 'okno', 'przycisk', 'tytuł', 'rozmiar', 'kontener', 'położenie']
language: pl
date: 2011-07-20 13:30:30+0000
---

# 3. Widżety

## 3.1. Wstawienie przycisku

W tym rozdziale dowiesz się, jak wstawić przycisk do naszego pustego okna. Aby jednak to zrobić, potrzebujemy kontenera, w którym będziemy mogli go umieścić. Użyjemy kontenera, który się nazywa fixed. Jego główną cechą jest to, że widżety umieszcza się w nim, podając położenie oraz rozmiar w pikselach. Poniżej znajduje się kod źródłowy z drugiego rozdziału:

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

Po:

```cpp
GtkWidget *okno;
```

umieścimy definicje zmiennych, które przechowają nam kontener oraz przycisk. Tak więc dopisujemy pod spodem:

```cpp
GtkWidget *kontener;
GtkWidget *przycisk;
```

No — ale jak widać, są to tylko wskaźniki, więc nasz kontener i przycisk przydało by się jakoś utworzyć. W tym celu pod linijką

```cpp
okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
```

dopisz:

```cpp
    kontener = gtk_fixed_new();
    gtk_container_add(GTK_CONTAINER(okno), kontener);

    przycisk = gtk_button_new_with_label("Twój pierwszy przycisk");
    gtk_widget_set_size_request(przycisk, 180, 35);
    gtk_fixed_put(GTK_FIXED(kontener), przycisk, 10, 10);
```

Już wszystko tłumaczę:\
Pierwsza linijka odpowiada za utworzenie kontenera fixed. Następna umieszcza go w oknie programu. Trzecia tworzy nowy przycisk z etykietą "Twój pierwszy przycisk". Następna ustala rozmiar przycisku (80x35 pikseli) i ostatnia umieszcza go w kontenerze 50 pikseli od lewej krawędzi kontenera i 20 od górnej.

Żeby ujrzeć nasz przycisk, należy jeszcze zamienić linijkę

```cpp
gtk_widget_show (okno);
```

na:

```cpp
gtk_widget_show_all(okno);
```

Aby wyświetlić wszystkie widżety, a nie tylko okno programu.

Całość wygląda następująco:

```cpp
#include <gtk/gtk.h>

int main (int argc, char *argv[]) {
    GtkWidget *okno;
    GtkWidget *kontener;
    GtkWidget *przycisk;

    gtk_init (&argc, &argv);

    okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);

    kontener = gtk_fixed_new();
    gtk_container_add(GTK_CONTAINER(okno), kontener);

    przycisk = gtk_button_new_with_label("Twój pierwszy przycisk");
    gtk_widget_set_size_request(przycisk, 180, 35);
    gtk_fixed_put(GTK_FIXED(kontener), przycisk, 10, 10);

    gtk_widget_show_all(okno);

    gtk_main ();

    return 0;
}
```

Po skompilowaniu programu powinieneś ujrzeć coś takiego:

[![Okno z przyciskiem "Twój pierwzy przycisk"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-3-kursgtk_03_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-3-kursgtk_03_scr01.png)

## 3.2. Właściwości okna programu

Uruchamiając swój program, z pewnością zauważyłeś, że rozmiar okna się zmienił. Z 400x400 px zmniejszył się do rozmiarów kontenera. Na szczęście da się to zmienić.

```cpp
void gtk_window_set_default_size (GtkWindow *window, gint width, gint height);
```

Powyższa funkcja pozwala zmienić domyślny rozmiar okna. Pierwszym argumentem jest referencja do okna, drugim żądana szerokość, oraz ostatnim wysokość. Zapewne zauważyłeś, że wysokość i szerokość nie jest zmienną typu `int`, tylko `gint`. GTK+ korzysta bowiem z własnych typów zmiennych, aby ułatwić portowanie biblioteki na inne języki programowania niż C i C++. Typami zmiennych w GTK+ nie będziemy się jednak w tym rozdziale zajmować, ponieważ są one identyczne jak znane Ci już typy języka C, jak na przykład `int`, `bool`, czy `float`.

Powyższą funkcję warto więc wstawić do kodu naszego programu. W tym celu pod:

```cpp
okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
```

wstaw:

```cpp
gtk_window_set_default_size (GTK_WINDOW(okno), 300, 200);
```

Po skompilowaniu i uruchomieniu programu powinno być już o wiele lepiej. Jak zapewne zauważyłeś, wykonaliśmy konwersję `GtkWidget` na `GtkWindow`. Jest to wymagane, ponieważ nasze okno ma typ `GtkWidget`, a funkcja `gtk_window_set_default_size()` przyjmuje wskaźnik do okna typu `GtkWindow`.

Warto również sprawić, aby po uruchomieniu okno znajdowało się pośrodku ekranu. W tym celu, pod funkcją `gtk_window_set_default_size()` wstaw następującą linijkę:

```cpp
gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
```

Oczywiście nie chcesz, aby Twoje okno miało tytuł "kurs_gtkplus", "main", czy podobną. To też się da zmienić. Służy do tego funkcja:

```cpp
void gtk_window_set_title (GtkWindow *window, const gchar *title);
```

Jako pierwszy argument przyjmuje ona referencję do okna, a jako drugi — żądany tytuł. Aby więc nasze okno miało tytuł "Nasz program", pod wywołaniem poprzednio opisywanej funkcji, wstaw:

```cpp
gtk_window_set_title (GTK_WINDOW(okno), "Nasz program");
```

Po skompilowaniu i uruchomieniu programu okno naszego programu powinno wyglądać następująco:

[![Większe okno z przyciskiem "Twój pierwzy przycisk"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-3-kursgtk_03_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-3-kursgtk_03_scr02.png)

To by było na tyle w tym rozdziale. W następnym powiem nieco o zdarzeniach w GTK+.
