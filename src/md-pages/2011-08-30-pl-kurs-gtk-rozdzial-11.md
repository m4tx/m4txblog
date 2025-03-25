---
title: 'Kurs GTK+ – rozdział 11'
permalink: 'kurs-gtk-rozdzial-11'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'GUI', 'okno', 'kurs GTK+', 'statusbar', 'pasek statusu', 'GtkTextView', 'iterator', 'bufor', 'tekst', 'linie', 'kolumny', 'znaki', 'wiersze']
language: pl
date: 2011-08-30 11:27:04+0000
---

# 11. `GtkTextView`

## 11.1. Wstęp

W tym rozdziale dowiesz się, czym jest kontrolka `GtkTextView`, do czego służy oraz jak ją utworzyć i używać.

`GtkTextView` to zaawansowany widżet służący do wyświetlania wieloliniowego tekstu i ewentualnej jego modyfikacji, również przez użytkownika. Posiada ogromne możliwości. Można formatować w nim tekst za pomocą tagów, można dostać się do znaków za pomocą iteratorów, a sam tekst wyświetlany przez ten widżet dodaje się do bufora.

## 11.2. Utworzenie `GtkTextView` i dodawanie czystego tekstu

`GtkTextView` można utworzyć za pomocą funkcji:

```cpp
GtkWidget* gtk_text_view_new (void);
```

Warto wspomnieć, że istnieje jeszcze podobna bardzo funkcja do powyższej, a jest nią:

```cpp
GtkWidget* gtk_text_view_new_with_buffer (GtkTextBuffer *buffer);
```

Czym one się różnią? Otóż pierwsza tworzy nowy widżet `GtkTextView` wraz z buforem. W przypadku drugiej funkcji bufor ten należy utworzyć samemu. Warto również wiedzieć o dwóch użytecznych funkcjach:

```cpp
GtkTextBuffer* gtk_text_view_get_buffer (GtkTextView *text_view);
void gtk_text_view_set_buffer (GtkTextView *text_view, GtkTextBuffer *buffer);
```

Pierwsza z nich jest szczególnie użyteczna przy tworzeniu `GtkTextView` za pomocą funkcji `gtk_text_view_new()`. Zwraca ona bowiem wskaźnik do utworzonego bufora. Druga natomiast ustawia nowy bufor dla widżetu.

Ok, wiesz już jak utworzyć kontrolkę `GtkTextView` wraz z buforem oraz jak się do tego bufora dostać. Jak jednak dodać do niego jakiś tekst?

Do tego celu posłuży nam funkcja:

```cpp
void gtk_text_buffer_insert (GtkTextBuffer *buffer, GtkTextIter *iter, const gchar *text, gint len);
```

W pierwszym argumencie przyjmuje ona wskaźnik do bufora tekstowego, w następnym do iteratora (o iteratorach za chwilę), w następnym żądany tekst, a w ostatnim — długość tego tekstu. W ostatnim argumencie można podać `-1` — wtedy jednak trzeba zadbać o to, aby ciąg znaków kończył się znakiem `NULL` — `0`.

Wspomniałem o iteratorach. Wyznaczają one pozycję umieszczenia ciągu znaków. Aby otrzymać taki iterator, można się posłużyć na przykład funkcją:

```cpp
void gtk_text_buffer_get_iter_at_offset (GtkTextBuffer *buffer, GtkTextIter *iter, gint char_offset);
```

W pierwszym argumencie należy podać referencję do bufora tekstowego, w następnym do iteratora, a w ostatnim należy podać żądaną pozycję.

Z taką wiedzą można już utworzyć widżet `GtkTextView` i dodać do niego tekst. Warto również przeanalizować poniższy kod źródłowy:

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *textView;
    GtkTextBuffer *bufor;
    GtkTextIter iter;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_container_set_border_width(GTK_CONTAINER(okno), 10);

    textView = gtk_text_view_new();
    bufor = gtk_text_view_get_buffer(GTK_TEXT_VIEW(textView));
    gtk_text_buffer_get_iter_at_offset(bufor, &iter, 0);
    gtk_text_buffer_insert(bufor, &iter, "Hello world!\n", -1);

    gtk_container_add(GTK_CONTAINER(okno), textView);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Działanie tego kodu można zobaczyć poniżej.

[![Okno programu z polem tekstowym](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr01.png)

## 11.3. Tagi

Napisałem we wstępie, że w `GtkTextView` można formatować tekst za pomocą tagów. Nie są to co prawda takie tagi jak np. w HTML-u, jednak również są bardzo użyteczne. Do tworzenia tagów służy funkcja:

```cpp
GtkTextTag* gtk_text_buffer_create_tag (GtkTextBuffer *buffer, const gchar *tag_name, const gchar *first_property_name, ...);
```

W pierwszym argumencie przyjmuje ona wskaźnik do bufora tekstowego, w następnym nazwę tagu, natomiast w kolejnych należy podać kolejno nazwę właściwości i żądaną jej wartość. Powyższa funkcja przyjmuje nieograniczoną liczbę argumentów, a ostatnim argumentem musi być `NULL`. Listę właściwości i wartości, które można im ustawić można znaleźć w [dokumentacji GTK+](https://web.archive.org/web/20110827232650/https://developer.gnome.org/gtk/2.24/GtkTextTag.html#GtkTextTag.properties).

Aby teraz do bufora dodać otagowany tekst, można użyć funkcji:

```cpp
void gtk_text_buffer_insert_with_tags_by_name (GtkTextBuffer *buffer, GtkTextIter *iter, const gchar *text, gint len, const gchar *first_tag_name, ...);
```

Jak nietrudno zauważyć, funkcja ta jest bardzo podobna do omawianej w poprzednim podrozdziale `gtk_text_buffer_insert()`, tak więc nie ma sensu omawiać jej pierwszych czterech argumentów ponownie, jednak — podobnie jak powyższa — ta funkcja może przyjąć nieskończoną liczbę argumentów. Należy podać bowiem nazwy tagów, które chcesz dodać do dodawanego tekstu. Ostatnim argumentem musi być oczywiście `NULL`.

Zachęcam do przeanalizowania poniższego kodu źródłowego. Może on pomóc w zrozumieniu działania tagów.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;

    GtkWidget *textView;
    GtkTextBuffer *bufor;
    GtkTextIter iter;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_container_set_border_width(GTK_CONTAINER(okno), 5);
    GTK_WINDOW(okno)->allow_shrink = TRUE;

    textView = gtk_text_view_new();

    bufor = gtk_text_view_get_buffer(GTK_TEXT_VIEW(textView));

    gtk_text_buffer_create_tag(bufor, "margines", "left_margin", 10, NULL);
    gtk_text_buffer_create_tag(bufor, "niebieska_czcionka", "foreground", "blue", NULL);
    gtk_text_buffer_create_tag(bufor, "zielone_tlo", "background", "green", NULL);
    gtk_text_buffer_create_tag(bufor, "pochylenie", "style", PANGO_STYLE_ITALIC, NULL);
    gtk_text_buffer_create_tag(bufor, "pogrubienie", "weight", PANGO_WEIGHT_BOLD, NULL);

    gtk_text_buffer_get_iter_at_offset(bufor, &iter, 0);

    gtk_text_buffer_insert(bufor, &iter, "Czysty tekst\n", -1);
    gtk_text_buffer_insert_with_tags_by_name(bufor, &iter, "Kolorowy tekst!\n", -1, "niebieska_czcionka",  NULL);
    gtk_text_buffer_insert_with_tags_by_name (bufor, &iter, "Margines\n", -1, "margines", NULL);
    gtk_text_buffer_insert_with_tags_by_name (bufor, &iter, "Tekst z kolorowym tłem\n", -1, "zielone_tlo", NULL);
    gtk_text_buffer_insert_with_tags_by_name (bufor, &iter, "Pochylony tekst\n", -1, "pochylenie", NULL);
    gtk_text_buffer_insert_with_tags_by_name (bufor, &iter, "Pogrubiony tekst\n", -1, "pogrubienie", NULL);
    gtk_text_buffer_insert_with_tags_by_name (bufor, &iter, "Wszystko razem!\n", -1, "margines", "niebieska_czcionka", "zielone_tlo", "pochylenie", "pogrubienie", NULL);

    gtk_container_add(GTK_CONTAINER(okno), textView);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Działania powyższego kodu źródłowego możesz zobaczyć poniżej.

[![Okno z polem tekstowym i tekstem z różnym formatowaniem](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr02.png)

## 11.4. Linie i kolumny

Aby pobrać numer aktualnej linii i kolumny należy posłużyć się iteratorem. Aby natomiast taki iterator — który wskazuje na aktualne położenie kursora — otrzymać, można posłużyć się funkcją:

```cpp
void gtk_text_buffer_get_iter_at_mark (GtkTextBuffer *buffer, GtkTextIter *iter, GtkTextMark *mark);
```

W pierwszym argumencie należy podać oczywiście wskaźnik do bufora, w drugim do iteratora, natomiast w trzecim należy podać wskaźnik do `GtkTextMark`. `GtkTextMark` to pozycja w buforze, którą można otrzymać np. poprzez funkcję:

```cpp
GtkTextMark* gtk_text_buffer_get_selection_bound (GtkTextBuffer *buffer);
```

Powyższa funkcja przyjmuje jako argument referencję do bufora i zwraca `GtkTextMark` w miejscu aktualnego położenia kursora.

Ok, tak więc mamy już nasz iterator. Teraz przydadzą nam się dwie funkcje:

```cpp
gint gtk_text_iter_get_line (const GtkTextIter *iter);
gint gtk_text_iter_get_line_offset (const GtkTextIter *iter);
```

Obie przyjmują tylko jeden argument — jest nim wskaźnik do iteratora. Pierwsza z tych dwóch funkcji zwraca numer linii, natomiast druga numer znaku (kolumny).

Pozostała jeszcze jedna kwestia — jak przechwycić zmodyfikowanie bufora lub przesunięcie kursora?

Jest to możliwe dzięki sygnałom `changed` oraz `mark_set`. Można je podpiąć do bufora za pomocą poznanej już funkcji `g_signal_connect()`. Przy czym warto wiedzieć, jakiej konstrukcji wywoływanej funkcji sygnały te wymagają, aby potem uniknąć ewentualnych błędów.

Dla sygnału `changed` jest to:

```cpp
void user_function (GtkTextBuffer *textbuffer, gpointer user_data)
```

I tutaj pierwszym argumentem jest oczywiście bufor tekstowy, a drugim — wskaźnik do ewentualnych innych danych czy zmiennych, natomiast dla sygnału `mark_set`:

```cpp
void user_function (GtkTextBuffer *textbuffer, GtkTextIter *location, GtkTextMark *mark, gpointer user_data)
```

Pierwszym argumentem jest tutaj wskaźnik do bufora tekstowego, następnym iterator dla `GtkTextMark`a, do którego wskaźnik jest przekazywany w następnym argumencie, no a czwarty argument to oczywiście dodatkowe dane.

Ze zdobytą wiedzą możesz już utworzyć widżet `GtkTextView` i np. pasek statusu, w którym podasz aktualny wiersz i kolumnę. Jeżeli wciąż czegoś nie rozumiesz, możesz spróbować przeanalizować poniższy kod źródłowy.

```cpp
#include <gtk/gtk.h>

void zaktualizuj_pasek(GtkTextBuffer *bufor, GtkStatusbar *pasek)
{
    gchar *wiadomosc;
    gint wrsz, kol;
    GtkTextIter iter;

    gtk_statusbar_pop(pasek, 0);

    gtk_text_buffer_get_iter_at_mark(bufor, &iter, gtk_text_buffer_get_insert(bufor));
    wrsz = gtk_text_iter_get_line(&iter);
    kol = gtk_text_iter_get_line_offset(&iter);

    wiadomosc = g_strdup_printf("Wrsz %d, Kol %d", wrsz+1, kol+1);

    gtk_statusbar_push(pasek, 0, wiadomosc);

    g_free(wiadomosc);
}

void zmiana_polozenia_kursora(GtkTextBuffer *bufor, const GtkTextIter *nowaLokalizacja, GtkTextMark *mark, gpointer pasek)
{
    zaktualizuj_pasek(bufor, GTK_STATUSBAR(pasek));
}

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *textView;
    GtkWidget *pasek;
    GtkTextBuffer *bufor;
    GtkTextIter iter;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 0);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    textView = gtk_text_view_new();
    gtk_box_pack_start(GTK_BOX(vbox), textView, TRUE, TRUE, 0);
    bufor = gtk_text_view_get_buffer(GTK_TEXT_VIEW(textView));
    gtk_text_buffer_get_iter_at_offset(bufor, &iter, 0);
    gtk_text_buffer_insert(bufor, &iter, "Hello world!", -1);

    pasek = gtk_statusbar_new();
    gtk_box_pack_start(GTK_BOX(vbox), pasek, FALSE, FALSE, 0);

    g_signal_connect(bufor, "changed", G_CALLBACK(zaktualizuj_pasek), pasek);
    g_signal_connect(bufor, "mark_set", G_CALLBACK(zmiana_polozenia_kursora), pasek);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    zaktualizuj_pasek(bufor, GTK_STATUSBAR (pasek));

    gtk_main();

    return 0;
}
```

W powyższym kodzie możesz znaleźć jeszcze dwie nowe funkcje i są to:

```cpp
gchar* g_strdup_printf (const gchar *format, ...);
void gtk_statusbar_pop (GtkStatusbar *statusbar, guint context_id);
```

Pierwsza z nich jest odpowiednikiem standardowego `sprintf()`, jednak bezpieczniejszym i przystosowanym do GTK+. Natomiast druga usuwa wiadomość z paska statusu. Warto zauważyć, że nie posługujemy się funkcją `gtk_statusbar_get_context_id()` omówioną w rozdziale 7, ponieważ nie musimy tutaj dodawać kilku wiadomości o różnej treści.

Działanie powyższego kodu źródłowego można zobaczyć poniżej.

[![Okno programu z polem tekstowym i paskiem statusu](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-11-kursgtk_11_scr03.png)

To by było na tyle w tym rozdziale. Opisuje on bardzo niewielką część możliwości `GtkTextView`, jednak zachęcam do samodzielnego głębszego poznawania tego widżetu. Następny rozdział opisuje użyteczny widżet `GtkTreeView`.
