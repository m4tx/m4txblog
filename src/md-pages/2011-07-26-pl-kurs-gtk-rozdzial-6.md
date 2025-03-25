---
title: 'Kurs GTK+ – rozdział 6'
permalink: 'kurs-gtk-rozdzial-6'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'przycisk', 'kurs GTK+', 'check button', 'toggle button', 'frame', 'entry', 'ramka', 'pole tekstowe', 'pole rozwijane']
language: pl
date: 2011-07-26 11:17:20+0000
---

# 6. Podstawowe kontrolki

## 6.1. `GtkCheckButton`

W tym rozdziale dowiesz się nieco informacji o kilku widżetach, które można umieścić w oknie programu — jak je utworzyć oraz jak ich używać. Jednym z nich jest `GtkCheckButton`.

`GtkCheckButton` to pole zaznaczane. Przydaje się na przykład, gdy chcemy, by użytkownik wybrał, czy włączyć jakąś funkcję, czy nie. Można go utworzyć następującą funkcją:

```cpp
GtkWidget* gtk_check_button_new_with_label (const gchar *label);
```

Jedynym parametrem jest *label*, czyli etykieta. Można przechwycić zdarzenie kliknięcia `GtkCheckButtona` — zdarzenie to nazywa się `clicked`. Warto również wspomnieć, że istnieje bardzo podobna do `GtkCheckButton` kontrolka — jest nią `GtkToggleButton`. Wygląda jak normalny przycisk z tym, że po wciśnięciu go pozostaje w takim stanie. `GtkCheckButton` tak naprawdę dziedziczy `GtkToggleButton` i zmienia jedynie wygląd tak powstałego widżetu. `GtkToggleButton` tworzy się funkcją `gtk_toggle_button_new_with_label()`, która również przyjmuje jako argument etykietę.

Jak już napisałem, `GtkCheckButton` dziedziczy `GtkToggleButton`, tak więc nazwy wszystkich funkcji związanych z zaznaczanym polem zaczynają się od `gtk_toggle_button`. Do pobierania i ustawiania stanu przycisku służą dwie funkcje:

```cpp
gboolean gtk_toggle_button_get_active (GtkToggleButton *toggle_button);
void gtk_toggle_button_set_active (GtkToggleButton *toggle_button, gboolean is_active);
```

`gtk_toggle_button_get_active()` pobiera wskaźnik do przycisku i zwraca `TRUE`, jeśli przycisk jest wciśnięty i `FALSE` w przeciwnym wypadku. `TRUE` i `FALSE` to wartości typowe dla GTK+, możesz jednak używać wartości typowych dla C/C++ — `true` i `false`. Druga funkcja, `gtk_toggle_button_set_active()` w pierwszym argumencie pobiera referencję do przycisku, a w drugim żądaną wartość — `TRUE`, jeśli przycisk ma być wciśnięty oraz `FALSE` jeżeli nie.

```cpp
#include <gtk/gtk.h>

void toggle_title(GtkWidget *widget)
{
    if (gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(widget)))
        gtk_button_set_label(GTK_BUTTON(widget), "Wciśnięto!");
    else
        gtk_button_set_label(GTK_BUTTON(widget), "Nie wciśnięto!");
}

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *checkButton;
    GtkWidget *toggleButton;

    gtk_init(&argc, &argv);

    okno = gtk_window_new (GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position (GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size (GTK_WINDOW(okno), 150, 75);
    gtk_window_set_title (GTK_WINDOW(okno), "Kurs GTK+");
    gtk_container_set_border_width (GTK_CONTAINER(okno), 5);

    vbox = gtk_vbox_new(FALSE, 1);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    checkButton = gtk_check_button_new_with_label ("Nie wciśnięto!");
    toggleButton = gtk_toggle_button_new_with_label ("Nie wciśnięto!");

    gtk_box_pack_start (GTK_BOX(vbox), checkButton, TRUE, TRUE, 0);
    gtk_box_pack_start (GTK_BOX(vbox), toggleButton, TRUE, TRUE, 0);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect(checkButton, "clicked", G_CALLBACK(toggle_title), NULL);
    g_signal_connect(toggleButton, "clicked", G_CALLBACK(toggle_title), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Powyższy kod ma za zadanie utworzenie okna, poznanego już kontenera `GtkVBox` i umieszczeniu w nim dwóch kontrolek — `GtkCheckButton` oraz `GtkToggleButton`, które po zaznaczeniu zmieniają zmieniają etykietę na "Wciśnięto!" oraz na "Nie wciśnięto!" po odznaczeniu. Efekt tego kodu można zobaczyć poniżej.

[![Okno programu z polem zaznaczanym "Wciśnięto!" i przyciskiem "Nie wciśnięto!"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr01.png)

## 6.2. `GtkFrame`

`GtkFrame` jest ramką, której można użyć, aby pogrupować widżety. Ma 4 różne style i są to: `GTK_SHADOW_IN`, `GTK_SHADOW_OUT`, `GTK_SHADOW_ETCHED_IN` i `GTK_SHADOW_ETCHED_OUT`. Tworzy się ją funkcją:

```cpp
GtkWidget* gtk_frame_new (const gchar *label);
```

Należy jej przekazać żądany tytuł dla ramki. Aby ustawić styl `GtkFrame` należy posłużyć się funkcją:

```cpp
void gtk_frame_set_shadow_type (GtkFrame *frame, GtkShadowType type);
```

W pierwszym parametrze przyjmuje ona wskaźnik do ramki, a w drugim jeden z wymienionych wyżej stylów. Poniższy kod źródłowy ma za zadanie utworzyć 4 ramki, każdą z innym stylem, aby zobrazować, jak każdy z nich wygląda.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *tabela;
    GtkWidget *ramka1;
    GtkWidget *ramka2;
    GtkWidget *ramka3;
    GtkWidget *ramka4;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 250);
    gtk_window_set_title(GTK_WINDOW(okno), "GtkFrame");

    gtk_container_set_border_width(GTK_CONTAINER(okno), 10);

    tabela = gtk_table_new(2, 2, TRUE);
    gtk_table_set_row_spacings(GTK_TABLE(tabela), 10);
    gtk_table_set_col_spacings(GTK_TABLE(tabela), 10);
    gtk_container_add(GTK_CONTAINER(okno), tabela);

    ramka1 = gtk_frame_new("GTK_SHADOW_IN");
    gtk_frame_set_shadow_type(GTK_FRAME(ramka1), GTK_SHADOW_IN);
    ramka2 = gtk_frame_new("GTK_SHADOW_OUT");
    gtk_frame_set_shadow_type(GTK_FRAME(ramka2), GTK_SHADOW_OUT);
    ramka3 = gtk_frame_new("GTK_SHADOW_ETCHED_IN");
    gtk_frame_set_shadow_type(GTK_FRAME(ramka3), GTK_SHADOW_ETCHED_IN);
    ramka4 = gtk_frame_new("GTK_SHADOW_ETCHED_OUT");
    gtk_frame_set_shadow_type(GTK_FRAME(ramka4), GTK_SHADOW_ETCHED_OUT);

    gtk_table_attach_defaults(GTK_TABLE(tabela), ramka1, 0, 1, 0, 1);
    gtk_table_attach_defaults(GTK_TABLE(tabela), ramka2, 0, 1, 1, 2);
    gtk_table_attach_defaults(GTK_TABLE(tabela), ramka3, 1, 2, 0, 1);
    gtk_table_attach_defaults(GTK_TABLE(tabela), ramka4, 1, 2, 1, 2);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu i uruchomieniu programu, zobaczysz coś takiego:

[![Okno programu z czterema ramkami](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr02.png)

## 6.3. `GtkLabel`

`GtkLabel` pozwala tworzyć etykiety. I to nie byle jakie. Daje możliwość nie tylko wprowadzenia czystego tekstu, ale również sformatowanie go za pomocą języka znaczników w pewnym stopniu podobnego do HTML-a. Lista obsługiwanych znaczników znajduje się na [stronie dokumentacji GTK+](https://docs.gtk.org/Pango/pango_markup.html).

Poniższa funkcja tworzy nową etykietę:

```cpp
GtkWidget* gtk_label_new (const gchar *str);
```

W argumencie należy przekazać żądany tekst. Aby jednak go sformatować, należy użyć funkcji:

```cpp
void gtk_label_set_markup (GtkLabel *label, const gchar *str);
```

W pierwszym argumencie należy przekazać referencję do widżetu, a w drugim — tekst napisany we wspomnianym języku znaczników — Pango Text Attribute Markup Language,

Przykład:

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *etykieta;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    char *str = "<sub>Ten</sub> <tt>tekst</tt> <span underline='error'>został</span> <span font='20'>napisany</span> <span bgcolor='#f00'>różnymi</span> <b><i>stylami</i></b>.";

    etykieta = gtk_label_new(NULL);
    gtk_label_set_markup(GTK_LABEL(etykieta), str);

    gtk_label_set_justify(GTK_LABEL(etykieta), GTK_JUSTIFY_CENTER);
    gtk_container_add(GTK_CONTAINER(okno), etykieta);
    gtk_widget_show(etykieta);

    gtk_window_set_default_size(GTK_WINDOW(okno), 350, 100);

    g_signal_connect(okno, "destroy", G_CALLBACK (gtk_main_quit), NULL);

    gtk_widget_show(okno);

    gtk_main();

    return 0;
}
```

Efekt działania programu powstałego po skompilowaniu powyższego kodu jest następujący:

[![Okno programu z tekstem z różnymi stylami](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr03.png)

## 6.4. `GtkComboBox`

`GtkComboBox` to lista rozwijana. Może się przydać, gdy chcemy dać wybór użytkownikowi jednej z kilku opcji — np. wybór miesiąca w kalendarzu. Tworzy się go następującą funkcją:

```cpp
GtkWidget* gtk_combo_box_text_new (void);
```

Tworzy ona jednak nie `GtkComboBox`a, a `GtkComboBoxText`. Różni się on od `GtkComboBox`a tym, że wszystkie jego elementy muszą być w postaci tekstu — `GtkComboBox` daje np. możliwość stworzenia drzewa.

Aby więc dodać element do `GtkComboBoxText`, należy posłużyć się funkcją:

```cpp
void gtk_combo_box_append_text (GtkComboBox *combo_box, const gchar *text);
```

W pierwszym argumencie przyjmuje ona referencję do `GtkComboBox`a, a w drugim żądany tekst.

Warto powiedzieć, że po wybraniu którejś opcji przez użytkownika, zostaje wywołane zdarzenie `changed`.

Poniższy kod ma za zadanie stworzyć listę rozwijaną, a po wybraniu którejś opcji przez użytkownika, zmienić tekst w etykiecie.

```cpp
#include <gtk/gtk.h>

void combo_selected(GtkWidget *widget, gpointer etykieta)
{
    gchar *text = gtk_combo_box_get_active_text(GTK_COMBO_BOX(widget));
    gtk_label_set_text(GTK_LABEL(etykieta), text);
    g_free(text);
}

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *combobox;
    GtkWidget *etykieta;
    GtkWidget *etykieta2;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 230, 100);

    vbox = gtk_vbox_new(FALSE, 1);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    etykieta = gtk_label_new("Wybierz miesiąc:");
    gtk_container_add(GTK_CONTAINER(vbox), etykieta);

    combobox = gtk_combo_box_new_text();
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Styczeń");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Luty");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Marzec");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Kwiecień");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Maj");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Czerwiec");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Lipiec");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Sierpień");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Wrzesień");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Październik");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Listopad");
    gtk_combo_box_append_text(GTK_COMBO_BOX(combobox), "Grudzień");

    gtk_container_add(GTK_CONTAINER(vbox), combobox);

    etykieta2 = gtk_label_new("");
    gtk_container_add(GTK_CONTAINER(vbox), etykieta2);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect(G_OBJECT(combobox), "changed", G_CALLBACK(combo_selected), (gpointer)etykieta2);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Efekt jego działania można zobaczyć poniżej.

[![Okno programu z polem rozwijanym "Wybierz miesiąc"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr04.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr04.png)

6.5. `GtkEntry`

Ostatnim z widżetów prezentowanych w tym rozdziale jest `GtkEntry`. Jest to pole tekstowe, do którego użytkownik może wpisywać jakieś dane. Tworzy się go funkcją:

```cpp
GtkWidget* gtk_entry_new (void);
```

Poniższy kod źródłowy ma za zadanie utworzyć etykietę oraz pole tekstowe.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *entry;
    GtkWidget *etykieta;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 230, 60);

    vbox = gtk_vbox_new(FALSE, 1);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    etykieta = gtk_label_new("Wpisz poniżej swoje imię:");
    gtk_container_add(GTK_CONTAINER(vbox), etykieta);

    entry = gtk_entry_new();
    gtk_container_add(GTK_CONTAINER(vbox), entry);

    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu i uruchomieniu programu powinieneś zobaczyć coś takiego:

[![Okno programu z polem tekstowym](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr05.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-6-kursgtk_06_scr05.png)

To by było na tyle w tym rozdziale. W następnym temat widżetów będzie kontynuowany.
