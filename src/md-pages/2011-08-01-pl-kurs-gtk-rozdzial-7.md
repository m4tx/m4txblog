---
title: 'Kurs GTK+ – rozdział 7'
permalink: 'kurs-gtk-rozdzial-7'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'GUI', 'okno', 'kurs GTK+', 'statusbar', 'pasek statusu', 'GtkImage', 'obrazek', 'separator', 'oddzielacz', 'linia oddzielająca']
language: pl
date: 2011-08-01 19:26:26+0000
---

# 7. Podstawowe kontrolki — część II

## 7.1. `GtkHSeparator`

Kolejny rozdział kursu biblioteki GTK+ — ciąg dalszy o kontrolkach

`GtkHSeparator` to pozioma linia oddzielająca na przykład 2 grupy kontrolek. Tworzy się ją bardzo prostą funkcją:

```cpp
GtkWidget* gtk_hseparator_new (void);
```

Która nie przyjmuje żadnych argumentów. Oprócz `GtkHSeparator` istnieje również bardzo podobna kontrolka. Jest nią `GtkVSeparator`, a od `GtkHSeparator`a różni się tym, że tworzona linia nie jest pozioma, a pionowa. Tworzy się ją — jak nietrudno się domyślić — niemal identyczną funkcją jak powyżej:

```cpp
GtkWidget* gtk_vseparator_new (void);
```

Poniższy przykład prezentuje obie kontrolki.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;

    GtkWidget *etykieta1;
    GtkWidget *etykieta2;
    GtkWidget *etykieta3;

    GtkWidget *hseparator;
    GtkWidget *vseparator;

    GtkWidget *vbox;
    GtkWidget *hbox;

    const char lorem_ipsum[] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Maecenas sit amet magna in mi tincidunt iaculis sit amet quis augue. Curabitur libero est, \
vehicula vel consequat a, cursus sit amet risus. Nulla id eros arcu, sit amet dictum eros. \
Cras mollis, leo et dignissim bibendum, purus sapien interdum enim, ut.";

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_window_set_resizable(GTK_WINDOW(okno), FALSE);

    gtk_container_set_border_width(GTK_CONTAINER(okno), 20);

    etykieta1 = gtk_label_new(lorem_ipsum);
    gtk_label_set_line_wrap(GTK_LABEL(etykieta1), TRUE);
    etykieta2 = gtk_label_new(lorem_ipsum);
    gtk_label_set_line_wrap(GTK_LABEL(etykieta2), TRUE);
    etykieta3 = gtk_label_new(lorem_ipsum);
    gtk_label_set_line_wrap(GTK_LABEL(etykieta3), TRUE);

    vbox = gtk_vbox_new(FALSE, 10);
    gtk_container_add(GTK_CONTAINER(okno), vbox);
    hbox = gtk_hbox_new(FALSE, 10);

    hseparator = gtk_hseparator_new();
    vseparator = gtk_vseparator_new();

    gtk_box_pack_start(GTK_BOX(hbox), etykieta2, FALSE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(hbox), vseparator, FALSE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(hbox), etykieta3, FALSE, TRUE, 0);
    gtk_container_add(GTK_CONTAINER(vbox), hbox);
    gtk_box_pack_start(GTK_BOX(vbox), hseparator, FALSE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(vbox), etykieta1, FALSE, TRUE, 0);

    g_signal_connect (G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Efekt działania powyższego kodu po skompilowaniu i uruchomieniu tak powstałego programu jest następujący:

[![Okno programu z liniami oddzielającymi tekst](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr01.png)

W powyższym kodzie można znaleźć 2 nowe funkcje: `gtk_window_set_resizable()` oraz `gtk_label_set_line_wrap()`. Pierwsza z nich ustawia, czy można zmienić rozmiar okna, czy nie. W pierwszym argumencie przyjmuje referencję do widżetu okna, a w drugim `TRUE`, jeśli zmiana rozmiaru ma być dozwolona i `FALSE` w przeciwnym wypadku. Druga funkcja natomiast ustawia łamanie wierszy w widżecie `GtkLabel`. W pierwszym argumencie przyjmuje wskaźnik do etykiety, a w drugim `TRUE`, jeśli wiersze mają być zawijane oraz `FALSE`, jeśli nie.

## 7.2. `GtkImage`

`GtkImage` pozwala wyświetlić obrazek z pliku. Nowy widżet typu `GtkImage` można utworzyć następującą funkcją:

```cpp
GtkWidget* gtk_image_new_from_file (const gchar *filename);
```

Przyjmuje ona w argumencie ścieżkę do pliku. Powyższa funkcja potrafi wczytać pliki zapisane w wielu popularnych formatach jak na przykład PNG, JPEG, czy BMP. Poniższy kod ma za zadanie wczytać plik, a następnie wyświetlić go w oknie programu.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *obraz;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_window_set_resizable(GTK_WINDOW(okno), FALSE);

    obraz = gtk_image_new_from_file("obraz.jpg");
    gtk_container_add(GTK_CONTAINER(okno), obraz);

    g_signal_connect_swapped(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu go i uruchomieniu tak powstałego programu zobaczysz:

[![Okno programu ze zdjęciem](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr02.jpeg)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr02.jpeg)

## 7.3. `GtkStatusbar`

`GtkStatusbar` to pasek statusu. Widnieje on w wielu programach u dołu okna. Tworzy się go funkcją:

```cpp
GtkWidget* gtk_statusbar_new (void);
```

Natomiast tekst w nim ustawia się funkcją:

```cpp
guint gtk_statusbar_push (GtkStatusbar *statusbar, guint context_id, const gchar *text);
```

W pierwszym argumencie należy podać wskaźnik do widżetu `GtkStatusbar`. W drugim identyfikator kontekstu, który można pobrać za pomocą funkcji `gtk_statusbar_get_context_id()`, która zostanie omówiona za chwilę, oraz żądany tekst do ustawienia. Funkcja zwraca identyfikator wiadomości, który może później zostać użyty do jej usunięcia. Wspomniana funkcja `gtk_statusbar_get_context_id()` wygląda następująco:

```cpp
guint gtk_statusbar_get_context_id (GtkStatusbar *statusbar, const gchar *context_description);
```

Przyjmuje ona w pierwszym argumencie referencję do widżetu paska statusu oraz w drugim żądaną wiadomość. Należy zwrócić uwagę, że w pasku statusu zostanie ustawiony tekst z funkcji `gtk_statusbar_push()`, a nie `gtk_statusbar_get_context_id()`.

Poniżej można zobaczyć, jak używać kontrolki `GtkStatusbar`.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *etykieta1;
    GtkWidget *statusbar;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 280, 150);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 2);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    etykieta1 = gtk_label_new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Maecenas sit amet magna in mi tincidunt iaculis sit amet quis augue. Curabitur libero est, \
vehicula vel consequat a, cursus sit amet risus. Nulla id eros arcu, sit amet dictum eros. \
Cras mollis, leo et dignissim bibendum, purus sapien interdum enim, ut.");
    gtk_label_set_line_wrap(GTK_LABEL(etykieta1), TRUE);
    gtk_box_pack_start(GTK_BOX(vbox), etykieta1, TRUE, TRUE, 10);

    statusbar = gtk_statusbar_new();
    gtk_box_pack_start(GTK_BOX(vbox), statusbar, FALSE, TRUE, 1);

    gtk_statusbar_push(GTK_STATUSBAR(statusbar), gtk_statusbar_get_context_id(GTK_STATUSBAR(statusbar), "Bezczynny"), "Bezczynny");

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Efekt działania powyższego kodu jest następujący:

[![Okno programu z paskiem statusu](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-7-kursgtk_07_scr03.png)

Następny rozdział kursu opisze, jak utworzyć menu i pasek narzędziowy.
