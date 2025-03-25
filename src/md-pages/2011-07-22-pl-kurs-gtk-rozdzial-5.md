---
title: 'Kurs GTK+ – rozdział 5'
permalink: 'kurs-gtk-rozdzial-5'
category: 'GTK+'
tags: ['Kurs GTK+', 'GUI', 'okno', 'kontenery', 'widżety', 'GtkVBox', 'GtkHBox', 'GtkTable', 'GtkAlignment']
language: pl
date: 2011-07-22 14:37:21+0000
---

# 5. Kontenery

## 5.1. `GtkVBox`

Oprócz wspomnianego już `GtkFixed` istnieją w GTK+ również inne kontenery. Jednym z nich jest `GtkVBox`.

`GtkVBox` to kontener, w którym widżety umieszczone są w jednej kolumnie — jeden pod drugim. Aby go utworzyć należy posłużyć się funkcją:

```cpp
GtkWidget* gtk_vbox_new (gboolean homogeneous, gint spacing);
```

W drugim parametrze należy podać żądany odstęp między widżetami, a w pierwszym należy ustalić, czy między wszystkimi widżetami ma być taka sama odległość.

Do ustawiania widżetów w `GtkVBox` służy natomiast funkcja:

```cpp
void gtk_box_pack_start (GtkBox *box, GtkWidget *child, gboolean expand, gboolean fill, guint padding);
```

Jako pierwszy parametr należy podać wskaźnik do obiektu typu `GtkBox`, w następnym referencję do widżetu, który ma się znaleźć w kontenerze, w następnym należy określić, czy widżet ma zajmować całą dostępną przestrzeń (jeżeli jest kilka widżetów, przestrzeń jest dzielona pomiędzy nimi), w kolejnym trzeba podać, czy widżet ma zostać rozciągnięty i w ostatnim ustawić odstęp pomiędzy innymi widżetami.

Przykładowe użycie `GtkVBox`:

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *nowa_gra;
    GtkWidget *wczytaj_gre;
    GtkWidget *opcje;
    GtkWidget *autorzy;
    GtkWidget *wyjdz;

    gtk_init(&argc, &argv);

    okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position (GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size (GTK_WINDOW(okno), 230, 250);
    gtk_window_set_title (GTK_WINDOW(okno), "Gra 1.0");
    gtk_container_set_border_width (GTK_CONTAINER(okno), 5);

    vbox = gtk_vbox_new(FALSE, 1);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    nowa_gra = gtk_button_new_with_label ("Nowa gra");
    wczytaj_gre = gtk_button_new_with_label ("Wczytaj grę");
    opcje = gtk_button_new_with_label ("Opcje");
    autorzy = gtk_button_new_with_label ("Autorzy");
    wyjdz = gtk_button_new_with_label ("Wyjdź");

    gtk_box_pack_start (GTK_BOX(vbox), nowa_gra, TRUE, TRUE, 0);
    gtk_box_pack_start (GTK_BOX(vbox), wczytaj_gre, TRUE, TRUE, 0);
    gtk_box_pack_start (GTK_BOX(vbox), opcje, TRUE, TRUE, 0);
    gtk_box_pack_start (GTK_BOX(vbox), autorzy, TRUE, TRUE, 0);
    gtk_box_pack_start (GTK_BOX(vbox), wyjdz, TRUE, TRUE, 0);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

W powyższym kodzie znajduje się również jedna nowa funkcja:

```cpp
gtk_container_set_border_width (GTK_CONTAINER(okno), 5);
```

Określa ona grubość ramki pomiędzy krawędzią kontenera a jego zawartością. Jako pierwszy argument podaje się referencję do kontenera, a jako drugą — grubość ramki wyrażoną w pikselach.

Po skompilowaniu i uruchomieniu programu ujrzysz coś takiego:

[![Przykładowy interfejs gry](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr01.png)

## 5.2. `GtkHBox`

Następnym omawianym kontenerem jest `GtkHBox`. Od powyższego różni się jedynie nazwą, funkcją, która go tworzy (a dokładniej `gtk_hbox_new()`) i tym, że układa widżety w jednym wierszu, a nie w kolumnie. Tak więc działanie powyższego programu po zmianie `GtkVBox`a na `GtkHBox`a oraz zmianie rozmiaru okna wygląda następująco:

[![Interfejs gry — horyzontalny](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr02.png)

## 5.3. `GtkTable`

`GtkTable` — jak sama nazwa wskazuje — umieszcza widżety w tabeli. Warto wspomnieć, że jeden widżet może wypełniać dowolną liczbę komórek tej tabeli. Aby utworzyć `GtkTable`, należy posłużyć się funkcją:

```cpp
GtkWidget* gtk_table_new (guint rows, guint columns, gboolean homogeneous);
```

Jako pierwszy warunek przyjmuje liczbę wierszy, drugi liczbę kolumn, a ostatni określa, czy komórki mają być takiej samej wielkości.

Aby dodać widżet do tabeli można posłużyć się funkcją `gtk_table_attach()`, jednak łatwiej jest to zrobić za pomocą `gtk_table_attach_defaults()`, gdyż przyjmuje ona 4 argumenty mniej i tym samym jest prostsza w użyciu.

```cpp
void gtk_table_attach_defaults (GtkTable *table, GtkWidget *widget, guint left_attach, guint right_attach, guint top_attach, guint bottom_attach);
```

Pierwszym argumentem, który należy podać jest referencja do `GtkTable`. Drugim — do widżetu. Następne 4 określają umiejscowienie widżetu — lewa krawędź, prawa krawędź, górna oraz dolna krawędź.

Istnieją jeszcze 2 funkcje przydatne przy pracy z tabelami. Są to `gtk_table_set_row_spacings()` oraz `gtk_table_set_col_spacings()`. Służą do ustawiania odstępów kolejno między wierszami oraz kolumnami. Obie funkcje jako pierwszy argument przyjmują wskaźnik do tabeli i jako drugi — rozmiar wyrażony w pikselach.

Za pomocą `GtkTable` można stworzyć wiele różnych interfejsów — na przykład interfejs kalkulatora. Kod źródłowy będzie więc następujący:

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;

    GtkWidget *tabela;
    GtkWidget *przycisk[16];

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 180);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    gtk_container_set_border_width(GTK_CONTAINER(okno), 5);

    tabela = gtk_table_new(4, 4, TRUE);
    gtk_table_set_row_spacings(GTK_TABLE(tabela), 2);
    gtk_table_set_col_spacings(GTK_TABLE(tabela), 2);

    przycisk[0] = gtk_button_new_with_label ("7");
    przycisk[1] = gtk_button_new_with_label ("8");
    przycisk[2] = gtk_button_new_with_label ("9");
    przycisk[3] = gtk_button_new_with_label ("/");

    przycisk[4] = gtk_button_new_with_label ("4");
    przycisk[5] = gtk_button_new_with_label ("5");
    przycisk[6] = gtk_button_new_with_label ("6");
    przycisk[7] = gtk_button_new_with_label ("*");

    przycisk[8] = gtk_button_new_with_label ("1");
    przycisk[9] = gtk_button_new_with_label ("2");
    przycisk[10] = gtk_button_new_with_label ("3");
    przycisk[11] = gtk_button_new_with_label ("-");

    przycisk[12] = gtk_button_new_with_label ("0");
    przycisk[13] = gtk_button_new_with_label (",");
    przycisk[14] = gtk_button_new_with_label ("=");
    przycisk[15] = gtk_button_new_with_label ("+");

    for (int x = 0; x < 4; x++)
        for (int y = 0; y < 4; y++)
            gtk_table_attach_defaults (GTK_TABLE(tabela), przycisk[(y*4)+x], x, x+1, y, y+1);

    gtk_container_add(GTK_CONTAINER(okno), tabela);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Efekt działania takiego programu jest następujący:

[![Interfejs kalkulatora](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr03.png)

## 5.4. `GtkAlignment`

Ostatnim z kontenerów, jaki zostanie omówiony w tym rozdziale jest `GtkAlignment`. Służy on raczej nie tyle do tworzenia całego interfejsu, a do umieszczania widżetów w określonym miejscu. Tworzy się go następującą funkcją:

```cpp
GtkWidget* gtk_alignment_new (gfloat xalign, gfloat yalign, gfloat xscale, gfloat yscale);
```

Pierwsze 2 argumenty to żądane położenie widżetu na osi x i y, natomiast następne 2 to jego wielkość. Oba argumenty to liczby typu `float` z przedziału 0-1.

Aby na przykład utworzyć puste okno z przyciskiem "Zamknij" w prawym dolnym rogu, można posłużyć się kodem:

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;

    GtkWidget *alignment;
    GtkWidget *zamknij;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    gtk_container_set_border_width(GTK_CONTAINER(okno), 5);

    zamknij = gtk_button_new_with_label ("Zamknij");

    alignment = gtk_alignment_new( 1, 1, 0, 0 );

    gtk_container_add(GTK_CONTAINER(okno), alignment);
    gtk_container_add(GTK_CONTAINER(alignment), zamknij);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Efekt będzie następujący:

[![Okno z przyciskiem "zamknij"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr04.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-5-kursgtk_05_scr04.png)

W następnym rozdziale postaram się omówić kilka widżetów.
