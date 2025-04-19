---
title: 'Kurs GTK+ – rozdział 9'
permalink: 'kurs-gtk-rozdzial-9'
category: 'programowanie'
tags: ['c++', 'GTK+', 'Kurs GTK+', 'GUI', 'dialogi', 'GtkMessageDialog', 'GtkAboutBoxDialog', 'GtkFontSelectionDialog', 'GtkColorSelectionDialog', 'GtkFileChooserDialog']
language: pl
date: 2011-08-16 15:35:23+0000
---

# 9. Dialogi

## 9.1. `GtkMessageDialog`

W tym rozdziale dowiesz się, jak tworzyć i używać dialogów w GTK+.

Pierwszym typem dialogów, jaki chciałbym omówić, to dialogi informacyjne. Charakteryzują się tym, że zazwyczaj nie pobierają żadnych informacji od użytkownika, a ich głównym celem jest powiadomienie użytkownika o czymś, co nastąpiło. Tworzy się je funkcją:

```cpp
GtkWidget* gtk_message_dialog_new (GtkWindow *parent, GtkDialogFlags flags, GtkMessageType type, GtkButtonsType buttons, const gchar *message_format, ...);
```

Pierwszym parametrem jest rodzic okna (można ustawić `NULL`). Następnym są flagi, których listę można znaleźć na [stronie dokumentacji GTK+](https://web.archive.org/web/20120118221354/http://developer.gnome.org/gtk/2.24/GtkDialog.html#GtkDialogFlags). W kolejnym argumencie należy podać typ dialogu — ich listę również można znaleźć na [stronie dokumentacji](https://web.archive.org/web/20120128225209/http://developer.gnome.org/gtk/2.24/GtkMessageDialog.html#GtkMessageType). Kolejnym są żądane przyciski. Ich listę także można znaleźć w [dokumentacji](https://web.archive.org/web/20120128225209/http://developer.gnome.org/gtk/2.24/GtkMessageDialog.html#GtkButtonsType). W ostatnim argumencie należy podać tytuł. Warto wspomnieć, że powyższą funkcją można go sformatować podobnie jak funkcją `printf()`.

Dialog uruchamia funkcja:

```cpp
gint gtk_dialog_run (GtkDialog *dialog);
```

Która jako argument przyjmuje referencję do dialogu.

Poniższy kod źródłowy utworzy okno z przyciskiem, po kliknięciu którego otworzy się dialog informacyjny.

```cpp
#include <gtk/gtk.h>

void pokaz_dialog(GtkWidget *widget, gpointer okno)
{
    GtkWidget *dialog;
    dialog = gtk_message_dialog_new(GTK_WINDOW(okno), GTK_DIALOG_DESTROY_WITH_PARENT, GTK_MESSAGE_ERROR, GTK_BUTTONS_OK, "Wystąpił błąd: %s", "nie można otworzyć pliku!");
    gtk_window_set_title(GTK_WINDOW(dialog), "Błąd");
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *przycisk;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 200, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    przycisk = gtk_button_new_with_label("Dialog");

    gtk_container_add(GTK_CONTAINER(okno), przycisk);
    gtk_container_set_border_width(GTK_CONTAINER(okno), 15);

    g_signal_connect(G_OBJECT(przycisk), "clicked", G_CALLBACK(pokaz_dialog), (gpointer)okno);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Jego działanie jest następujące:

[![Okno programu z dialogiem błędu](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr01.png)

## 9.2. `GtkAboutDialog`

`GtkAboutDialog` to bardzo popularny dialog, który można znaleźć w bardzo wielu programach — jest to dialog "O programie". Tworzy się go funkcją:

```cpp
GtkWidget* gtk_about_dialog_new (void);
```

Wywołanie jedynie powyższej funkcji da niezbyt ciekawy efekt. Aby dialog urozmaicić, można użyć poniższych funkcji:

```cpp
void gtk_about_dialog_set_name (GtkAboutDialog *about, const gchar *name);
void gtk_about_dialog_set_version (GtkAboutDialog *about, const gchar *version);
void gtk_about_dialog_set_copyright (GtkAboutDialog *about, const gchar *copyright);
void gtk_about_dialog_set_comments (GtkAboutDialog *about, const gchar *comments);
void gtk_about_dialog_set_website (GtkAboutDialog *about, const gchar *website);
```

Nie ma sensu omawiać każdej z osobna, dlatego powiem tylko, co każda z nich robi. Pierwsze ustawia nazwę programu wyświetlaną w dialogu, druga wersję, trzecia ustawia właściciela praw autorskich, następna krótki opis programu i ostatnia — stronę internetową. Każda z nich posiada takie same argumenty. Pierwszym jest wskaźnik do dialogu, a drugim żądany ciąg znaków.

Warto jeszcze powiedzieć, że do dialogu "O programie" można dodać logo. W tym celu należy utworzyć zmienną typu `GdkPixbuf` i załadować do niej logo programu, a następnie za pomocą funkcji:

```cpp
void gtk_about_dialog_set_logo (GtkAboutDialog *about, GdkPixbuf *logo);
```

Która w pierwszym argumencie przyjmuje referencję do dialogu, a w drugim do zmiennej typu `GdkPixbuf`, dodać logo do dialogu. Należy też zwolnić pamięć po logo, posługując się funkcją:

```cpp
void g_object_unref (gpointer object);
```

Przykład jej użycia w kodzie źródłowym tworzącym okno "O programie" znajduje się poniżej.

```cpp
#include <gtk/gtk.h>

void pokaz_dialog(GtkWidget *widget, gpointer okno)
{
    GtkWidget *dialog = gtk_about_dialog_new();
    GdkPixbuf *pixbuf = gdk_pixbuf_new_from_file("smsprice.png", NULL);

    gtk_about_dialog_set_name(GTK_ABOUT_DIALOG(dialog), "SmsPrice");
    gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(dialog), "1.0");
    gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(dialog), "(c) Mateusz \"m4tx\" M.");
    gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(dialog), "SmsPrice to program sprawdzający cenę wysłania wiadomości na podany numer SMS Premium.");
    gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(dialog), "http://www.m4tx.pl/");
    gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(dialog), pixbuf);
    g_object_unref(pixbuf), pixbuf = NULL;
    gtk_dialog_run(GTK_DIALOG (dialog));
    gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *przycisk;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 200, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    przycisk = gtk_button_new_with_label("Dialog");

    gtk_container_add(GTK_CONTAINER(okno), przycisk);
    gtk_container_set_border_width(GTK_CONTAINER(okno), 15);

    g_signal_connect(G_OBJECT(przycisk), "clicked", G_CALLBACK(pokaz_dialog), (gpointer)okno);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu i uruchomieniu programu powinieneś zobaczyć coś takiego:

[![Okno programu z dialogiem "O programie"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr02.png)

## 9.3. `GtkFontSelectionDialog`

`GtkFontSelectionDialog` to okno wyboru czcionki. Może się przydać chociażby przy pisaniu edytora tekstu. `GtkFontSelectionDialog` tworzy się funkcją:

```cpp
GtkWidget* gtk_font_selection_dialog_new (const gchar *title);
```

Funkcja ta przyjmuje jako argument żądany tytuł okna. Aby jednak zmienić czcionkę w wybranym widżecie, należy posłużyć się poniższymi funkcjami:

```cpp
gchar* gtk_font_selection_dialog_get_font_name (GtkFontSelectionDialog *fsd);
PangoFontDescription* pango_font_description_from_string (const char *str);
void gtk_widget_modify_font (GtkWidget *widget, PangoFontDescription *font_desc);
```

Pierwsza z nich zamienia nazwę i właściwości wybranej czcionki na tekst. Następna zamienia ten tekst na zmienną typu `PangoFontDescription`. Ostatnia ustawia żądanemu widżetowi czcionkę.

Warto również wiedzieć, który z przycisków — "OK" czy "Anuluj" w dialogu wyboru czcionki wcisnął użytkownik. Należy do tego wykorzystać zmienną typu `GtkResponseType`, która zwracana jest przez funkcję `gtk_dialog_run()`. Listę zwracanych wartości można znaleźć w [dokumentacji GTK+](https://web.archive.org/web/20120118221354/http://developer.gnome.org/gtk/2.24/GtkDialog.html#GtkResponseType).

Poniższy kod źródłowy pokazuje, jak utworzyć dialog wyboru czcionki oraz zmienić czcionkę etykiety na tę wybraną w dialogu.

```cpp
#include <gtk/gtk.h>

void pokaz_dialog(GtkWidget *widget, gpointer etykieta)
{
    GtkResponseType wynik;

    GtkWidget *dialog = gtk_font_selection_dialog_new("Select Font");
    wynik = (GtkResponseType)gtk_dialog_run(GTK_DIALOG(dialog));

    if (wynik == GTK_RESPONSE_OK || wynik == GTK_RESPONSE_APPLY)
    {
        PangoFontDescription *czcionka;
        gchar *nazwa_czcionki = gtk_font_selection_dialog_get_font_name(GTK_FONT_SELECTION_DIALOG(dialog));

        czcionka = pango_font_description_from_string(nazwa_czcionki);

        gtk_widget_modify_font(GTK_WIDGET(etykieta), czcionka);

        g_free(nazwa_czcionki);
    }

    gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *przycisk;
    GtkWidget *etykieta;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 200, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    przycisk = gtk_button_new_with_label("Dialog");
    etykieta = gtk_label_new("Tekst");

    vbox = gtk_vbox_new(FALSE, 3);

    gtk_container_add(GTK_CONTAINER(okno), vbox);
    gtk_box_pack_start(GTK_BOX(vbox), etykieta, FALSE, FALSE, 0);
    gtk_box_pack_start(GTK_BOX(vbox), przycisk, TRUE, TRUE, 0);
    gtk_container_set_border_width(GTK_CONTAINER(okno), 15);

    g_signal_connect(G_OBJECT(przycisk), "clicked", G_CALLBACK(pokaz_dialog), (gpointer)etykieta);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Jego działanie jest następujące:

[![Okno programu z dialogiem wybioru czcionki](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr03.png)

## 9.4. `GtkColorSelectionDialog`

`GtkColorSelectionDialog` to dialog wyboru koloru. Tworzy się go funkcją:

```cpp
GtkWidget* gtk_color_selection_dialog_new (const gchar *title);
```

W argumencie przyjmuje ona żądany tytuł okna. Aby pobrać kolor wybrany przez użytkownika należy posłużyć się funkcją:

```cpp
void gtk_color_selection_get_current_color (GtkColorSelection *colorsel, GdkColor *color);
```

W pierwszym argumencie przyjmuje ona zmienną typu `GtkColorSelection`, którą można pobrać stosując poniższą kontrukcję:

```cpp
GTK_COLOR_SELECTION(GTK_COLOR_SELECTION_DIALOG(dialog)->colorsel)
```

Natomiast w drugim argumencie — wyjściowym — należy podać wskaźnik do zmiennej typu `GdkColor`.

Wszelkie wątpliwości powinien rozwiać poniższy kod źródłowy. Utworzony przycisk otwiera dialog, który po wybraniu koloru i zatwierdzeniu, zmienia kolor etykiety.

```cpp
#include <gtk/gtk.h>

void pokaz_dialog(GtkWidget *widget, gpointer etykieta)
{
    GtkResponseType wynik;

    GtkWidget *dialog = gtk_color_selection_dialog_new("Wybierz kolor");
    wynik = (GtkResponseType)gtk_dialog_run(GTK_DIALOG(dialog));

    if (wynik == GTK_RESPONSE_OK)
    {
        GdkColor kolor;
        GtkColorSelection *wybrany_kolor;

        wybrany_kolor = GTK_COLOR_SELECTION(GTK_COLOR_SELECTION_DIALOG(dialog)->colorsel);
        gtk_color_selection_get_current_color(wybrany_kolor, &kolor);
        gtk_widget_modify_fg(GTK_WIDGET(etykieta), GTK_STATE_NORMAL, &kolor);
    }

    gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *przycisk;
    GtkWidget *etykieta;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 200, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    przycisk = gtk_button_new_with_label("Dialog");
    etykieta = gtk_label_new("Tekst");

    vbox = gtk_vbox_new(FALSE, 3);

    gtk_container_add(GTK_CONTAINER(okno), vbox);
    gtk_box_pack_start(GTK_BOX(vbox), etykieta, FALSE, FALSE, 0);
    gtk_box_pack_start(GTK_BOX(vbox), przycisk, TRUE, TRUE, 0);
    gtk_container_set_border_width(GTK_CONTAINER(okno), 15);

    g_signal_connect(G_OBJECT(przycisk), "clicked", G_CALLBACK(pokaz_dialog), (gpointer)etykieta);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu i uruchomieniu w ten sposób powstałego programu, powinieneś zobaczyć:

[![Okno programu z dialogiem wyboru koloru](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr04.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr04.png)

## 9.5. `GtkFileChooserDialog`

`GtkFileChooserDialog` to jeden z bardziej przydatnych dialogów. Jest to bowiem dialog wyboru pliku. Może się więc przydać w przeróżnych programach — edytorach tekstu, obrazów, czy nawet w grach. `GtkFileChooserDialog` tworzy się funkcją:

```cpp
GtkWidget* gtk_file_chooser_dialog_new (const gchar *title, GtkWindow *parent, GtkFileChooserAction action, const gchar *first_button_text, ...);
```

W pierwszym argumencie należy podać żądany tytuł dialogu. W następnym wskaźnik do rodzica. W kolejnym należy podać typ dialogu — otwarcie pliku, zapisanie, wybór lub utworzenie katalogu — ich lista dostępna jest w [dokumentacji GTK+](https://web.archive.org/web/20120827034627/http://developer.gnome.org/gtk/2.24/GtkFileChooser.html#GtkFileChooserAction). W kolejnych argumentach należy podać kolejno etykietę przycisku (warto zaznaczyć, że można podać ID elementu z opisywanego już stocku) oraz odpowiedź zwracaną po wciśnięciu tego przycisku — kilkukrotnie, w zależności od żądanej ilości przycisków w dialogu. W ostatnim argumencie należy podać `NULL`.

Warto również wiedzieć, jak pobrać ścieżkę do pliku lub folderu wybranego przez użytkownika. Służy do tego funkcja:

```cpp
gchar* gtk_file_chooser_get_filename (GtkFileChooser *chooser);
```

Należy przekazać jej referencję do dialogu.

Warto znać również jeszcze dwie przydatne funkcje:

```cpp
gboolean gtk_file_chooser_get_current_folder (GtkFileChooser *chooser, const gchar *folder);
void gtk_file_chooser_set_current_name (GtkFileChooser *chooser, const gchar *name);
```

Pierwsza z nich ustawia domyślnie otwarty katalog, natomiast druga — domyślną nazwę pliku. Obie funkcje w pierwszym argumencie przyjmują wskaźnik do dialogu, a w drugim ciąg znaków — ścieżkę do folderu, lub nazwę pliku.

Poniższy kod źródłowy ma za zadanie utworzenie okna z przyciskiem i etykietą. Po wciśnięciu przycisku otwiera się dialog, w którym można wybrać plik, do którego ścieżka będzie wyświetlać się w etykiecie.

```cpp
#include <gtk/gtk.h>

GtkWidget *etykieta;

void pokaz_dialog(GtkWidget *widget, gpointer okno)
{
    GtkWidget *dialog;

    dialog = gtk_file_chooser_dialog_new ("Otwórz...", GTK_WINDOW(okno), GTK_FILE_CHOOSER_ACTION_OPEN, GTK_STOCK_CANCEL, GTK_RESPONSE_CANCEL, GTK_STOCK_OPEN, GTK_RESPONSE_ACCEPT, NULL);

    if (gtk_dialog_run (GTK_DIALOG (dialog)) == GTK_RESPONSE_ACCEPT)
    {
        char *nazwa_pliku;

        nazwa_pliku = gtk_file_chooser_get_filename (GTK_FILE_CHOOSER (dialog));
        gtk_label_set_text(GTK_LABEL(etykieta), nazwa_pliku);
        g_free (nazwa_pliku);
    }

    gtk_widget_destroy (dialog);
}

int main(int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *przycisk;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 200, 100);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    przycisk = gtk_button_new_with_label("Dialog");
    etykieta = gtk_label_new("Tekst");

    vbox = gtk_vbox_new(FALSE, 3);

    gtk_container_add(GTK_CONTAINER(okno), vbox);
    gtk_box_pack_start(GTK_BOX(vbox), etykieta, FALSE, FALSE, 0);
    gtk_box_pack_start(GTK_BOX(vbox), przycisk, TRUE, TRUE, 0);
    gtk_container_set_border_width(GTK_CONTAINER(okno), 15);

    g_signal_connect(G_OBJECT(przycisk), "clicked", G_CALLBACK(pokaz_dialog), (gpointer)okno);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Działanie tego kodu jest następujące:

[![Okno programu z dialogiem wyboru pliku](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr05.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-9-kursgtk_09_scr05.png)

To koniec tego rozdziału. Następny omawia motywy.
