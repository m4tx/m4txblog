---
title: 'Kurs GTK+ – rozdział 8'
permalink: 'kurs-gtk-rozdzial-8'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'GUI', 'okno', 'przycisk', 'kurs GTK+', 'separator', 'stock items', 'stock', 'otwórz', 'zakończ', 'wyjdź', 'menu', 'pasek narzędziowy', 'toolbar', 'nowy']
language: pl
date: 2011-08-08 18:05:07+0000
---

# 8. Menu i toolbary

## 8.1. Menu programu

Ten rozdział opisuje tworzenie menu programu oraz paska narzędziowego.

Menu jest wykorzystywane w wielu programach. Wykorzystuje się je, aby ułatwić dostęp do niektórych funkcji lub nie zaśmiecać głównego okna programu. W każdym razie menu jest to rzecz bardzo przydatna i warto wiedzieć, jak je utworzyć. Pasek menu tworzy się funkcją:

```cpp
GtkWidget* gtk_menu_bar_new (void);
```

Jednak co nam z takiego paska, jak nic na nim nie ma. Menu do naszego paska można utworzyć funkcją:

```cpp
GtkWidget* gtk_menu_new (void);
```

Wciąż jednak z wykorzystaniem tych dwóch funkcji nie stworzymy funkcjonalnego menu. Potrzebna jest jeszcze jedna z poniższych funkcji:

```cpp
GtkWidget* gtk_menu_item_new_with_label (const gchar *label);
GtkWidget* gtk_menu_item_new_with_mnemonic (const gchar *label);
```

Czym one się różnią? Obie tworzą element menu z etykietą podaną w argumencie. Druga z nich pozwala jednak dodać mnemonikę, czyli literę, która pozwala na szybki dostęp do poszczególnych elementów menu z wykorzystaniem klawiatury. Dla przykładu — otwórz jakąś aplikację wykorzystującą paski menu, np. menedżer plików (Eksplorator Windows, Nautilus, Dolphin...) i wciśnij lewy Alt. Mnemoniki w menu programu powinny być teraz podkreślone. Dla menu "Plik" zazwyczaj jest to litera P. Wciśnij teraz tę literę trzymając wciąż Alt. Menu powinno się otworzyć. I po to właśnie są mnemoniki.

Warto również zaznaczyć, że mnemoniki można wykorzystać nie tylko przy tworzeniu menu. Wiele z omówionych wcześniej kontrolek, jak np. przyciski obsługuje mnemoniki. Zazwyczaj aby utworzyć jakąś kontrolkę z etykietą i z mnemoniką, wystarczy zmienić `with_label` na `with_mnemonic` w wywołaniu funkcji tworzącej daną kontrolkę.

No tak — mamy już utworzony pasek menu, mamy utworzone menu oraz jego elementy, czego więc brakuje? No tak — połączenia tych elementów w jedną całość. Od tego bowiem jest funkcja:

```cpp
void gtk_menu_shell_append (GtkMenuShell *menu_shell, GtkWidget *child);
```

Czym jednak w tej funkcji jest `GtkMenuShell`? Otóż widżety `GtkMenuBar` i `GtkMenu` dziedziczą właśnie `GtkMenuShell` — tak więc tej funkcji należy przekazać referencję do paska menu lub do samego menu.

Ostatnią funkcją, jaką należy wykorzystać, aby stworzyć działające menu programu jest:

```cpp
void gtk_menu_item_set_submenu (GtkMenuItem *menu_item, GtkWidget *submenu);
```

Dodaje ona element do menu.

Warto również wiedzieć, czy użytkownik kliknął w jakiś element menu. Można użyć do tego zdarzenia, które nazywa się activate.

Aby zrozumieć, jak utworzyć menu programu, można przeanalizować poniższy kod źródłowy.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *pasek;
    GtkWidget *menu;
    GtkWidget *plik;
    GtkWidget *wyjdz;
    GtkWidget *etykieta1;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 0);
    gtk_container_add(GTK_CONTAINER(okno), vbox);
    etykieta1 = gtk_label_new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Maecenas sit amet magna in mi tincidunt iaculis sit amet quis augue. Curabitur libero est, \
vehicula vel consequat a, cursus sit amet risus. Nulla id eros arcu, sit amet dictum eros. \
Cras mollis, leo et dignissim bibendum, purus sapien interdum enim, ut.");
    gtk_label_set_line_wrap(GTK_LABEL(etykieta1), TRUE);

    pasek = gtk_menu_bar_new();
    menu = gtk_menu_new();

    plik = gtk_menu_item_new_with_mnemonic("_Plik");
    wyjdz = gtk_menu_item_new_with_mnemonic("_Wyjdź");

    gtk_menu_item_set_submenu(GTK_MENU_ITEM(plik), menu);
    gtk_menu_shell_append(GTK_MENU_SHELL(menu), wyjdz);
    gtk_menu_shell_append(GTK_MENU_SHELL(pasek), plik);
    gtk_box_pack_start(GTK_BOX(vbox), pasek, FALSE, FALSE, 0);

    gtk_box_pack_start(GTK_BOX(vbox), etykieta1, FALSE, FALSE, 3);

    g_signal_connect (G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect (G_OBJECT(wyjdz), "activate", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Działanie powyższego kodu jest następujące:

[![Okno programu z paskiem menu](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr01.png)

## 8.2. Stock items

Stock items to zbiór predefiniowanych etykiet, często połączonych z ikonami oraz czasami akceleratorami (o akceleratorach za chwilę), które można wykorzystać, by w łatwy sposób dodać do programu ikony, nie zwiększając jednocześnie jego wielkości lub etykiety automatycznie dostosowujące się do języka systemu użytkownika. Listę stock items można znaleźć na [stronie dokumentacji GTK+](https://web.archive.org/web/20120411021909/http://developer.gnome.org/gtk/2.24/gtk-Stock-Items.html).

Aby utworzyć nowy element menu korzystając ze stocku, należy posłużyć się funkcją:

```cpp
GtkWidget* gtk_image_menu_item_new_from_stock  (const gchar *stock_id, GtkAccelGroup *accel_group);
```

W pierwszym argumencie należy przekazać identyfikator stock itemu, a w drugim grupę akceleratorów. Tymczasowo można w drugim argumencie przekazać `NULL`, gdyż o akceleratorach — jak już pisałem — będzie za chwilę, natomiast w identyfikator przekazywany w pierwszym argumencie można zdobyć korzystając z powyższego linku prowadzącego do dokumentacji GTK+.

Linijkę:

```cpp
wyjdz = gtk_menu_item_new_with_mnemonic("_Wyjdź");
```

Możesz więc już zastąpić z:

```cpp
wyjdz = gtk_image_menu_item_new_from_stock(GTK_STOCK_QUIT, NULL);
```

GTK+ posiada własne ustawienia wspólne dla wszystkich aplikacji napisanych z jego wykorzystaniem. Może zdarzyć się tak, że w ustawieniach tych będzie wyłączone pokazywanie obrazków w menu. Wtedy należy to wymusić funkcją:

```cpp
void gtk_image_menu_item_set_always_show_image (GtkImageMenuItem *image_menu_item, gboolean always_show);
```

W pierwszym argumencie należy przekazać referencję do widżetu typu `GtkImageMenuItem` (nasz element menu utworzony ze stocku się do nich zalicza), a w drugim `TRUE`, jeżeli chcesz, aby ikonka była zawsze pokazywana oraz `FALSE` w przeciwnym wypadku. Po wywołaniu funkcji `gtk_image_menu_item_new_from_stock()` możesz więc dopisać:

```cpp
gtk_image_menu_item_set_always_show_image (GTK_IMAGE_MENU_ITEM(wyjdz), true);
```

Dzięki temu uzyskasz coś takiego:

[![Okno programu z paskiem menu i ikoną](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr02.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr02.png)

## 8.3. Akceleratory

Akceleratory to skróty klawiszowe, pokazywane obok elementów menu. Aby z nich korzystać, należy utworzyć grupę akceleratorów. Robi się to funkcją:

```cpp
GtkAccelGroup* gtk_accel_group_new (void);
```

Która nie przyjmuje żadnych argumentów, zwraca natomiast wskaźnik do obiektu typu GtkAccelGroup. Aby połączyć widżet z akceleratorem, należy wykorzystać funkcję:

```cpp
void gtk_widget_add_accelerator (GtkWidget *widget, const gchar *accel_signal, GtkAccelGroup *accel_group, guint accel_key, GdkModifierType accel_mods, GtkAccelFlags accel_flags);
```

W pierwszym argumencie należy podać referencję do widżetu, do którego ma zostać podłączony akcelerator, w drugim nazwę zdarzenia, które ma zostać wysłane przy wywołaniu akceleratora, w następnym wskaźnik do grupy akceleratorów, w następnym wymagany klawisz, w kolejnym modyfikator (czyli dodatkowy wymagany klawisz, taki jak Ctrl, Alt, czy Shift — ich lista dostępna jest w [dokumentacji GDK](https://web.archive.org/web/20121003233417/http://developer.gnome.org/gdk/stable/gdk-Windows.html#GdkModifierType)), a w ostatnim — flagi, których lista również jest dostępna na [stronie dokumentacji GTK+](https://web.archive.org/web/20120331024058/http://developer.gnome.org/gtk/2.24/gtk-Standard-Enumerations.html#GtkAccelFlags). Do większości zastosowań wystarcza tylko `GTK_ACCEL_VISIBLE` — ustawia on widoczność akceleratora.

Aby akceleratory działały poprawnie, potrzebna jest jeszcze jedna funkcja. Jest nią:

```cpp
void gtk_window_add_accel_group (GtkWindow *window, GtkAccelGroup *accel_group);
```

Łączy ona grupę akceleratorów z oknem.

Aby program po dodaniu obsługi akceleratorów się skompilował, należy jeszcze dołączyć plik `gdk/gdkkeysyms.h`.

Poniższy kod źródłowy ma za zadanie utworzyć okno, menu, oraz element z etykietą "Zakończ" pobraną ze stocku, oraz ze skrótem klawiaturowym Ctrl+Q.

```cpp
#include <gtk/gtk.h>
#include <gdk/gdkkeysyms.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *pasek;
    GtkWidget *menu;
    GtkWidget *plik;
    GtkWidget *wyjdz;
    GtkWidget *etykieta1;

    GtkAccelGroup *grupa_akcel;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 0);
    gtk_container_add(GTK_CONTAINER(okno), vbox);
    etykieta1 = gtk_label_new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Maecenas sit amet magna in mi tincidunt iaculis sit amet quis augue. Curabitur libero est, \
vehicula vel consequat a, cursus sit amet risus. Nulla id eros arcu, sit amet dictum eros. \
Cras mollis, leo et dignissim bibendum, purus sapien interdum enim, ut.");
    gtk_label_set_line_wrap(GTK_LABEL(etykieta1), TRUE);

    pasek = gtk_menu_bar_new();
    menu = gtk_menu_new();
    grupa_akcel = gtk_accel_group_new();
    gtk_window_add_accel_group(GTK_WINDOW(okno), grupa_akcel);

    plik = gtk_menu_item_new_with_mnemonic("_Plik");
    wyjdz = gtk_image_menu_item_new_from_stock(GTK_STOCK_QUIT, NULL);
    gtk_image_menu_item_set_always_show_image (GTK_IMAGE_MENU_ITEM(wyjdz), true);
    gtk_widget_add_accelerator(wyjdz, "activate", grupa_akcel, GDK_q, GDK_CONTROL_MASK, GTK_ACCEL_VISIBLE);

    gtk_menu_item_set_submenu(GTK_MENU_ITEM(plik), menu);
    gtk_menu_shell_append(GTK_MENU_SHELL(menu), wyjdz);
    gtk_menu_shell_append(GTK_MENU_SHELL(pasek), plik);
    gtk_box_pack_start(GTK_BOX(vbox), pasek, FALSE, FALSE, 0);

    gtk_box_pack_start(GTK_BOX(vbox), etykieta1, FALSE, FALSE, 3);

    g_signal_connect (G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect (G_OBJECT(wyjdz), "activate", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu i uruchomieniu tak powstałego programu ujrzysz:

[![Okno programu z paskiem menu i akceleratorami](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr03.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr03.png)

Po wciśnięciu kombinacji Ctrl+Q program powinien zakończyć działanie.

## 8.4. Separatory

Separatory przydają się, aby oddzielić kilka grup elementów menu. Tworzy się je funkcją:

```cpp
GtkWidget* gtk_separator_menu_item_new (void);
```

Przyłącza się go do menu identycznie, jak w przypadku każdego innego elementu.

Poniższy kod źródłowy tworzy menu z elementami "Otwórz" i "Zakończ" oraz separatorem pomiędzy nimi:

```cpp
#include <gtk/gtk.h>
#include <gdk/gdkkeysyms.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *pasek;
    GtkWidget *menu;
    GtkWidget *plik;
    GtkWidget *otworz;
    GtkWidget *separator;
    GtkWidget *wyjdz;
    GtkWidget *etykieta1;

    GtkAccelGroup *grupa_akcel;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 0);
    gtk_container_add(GTK_CONTAINER(okno), vbox);
    etykieta1 = gtk_label_new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Maecenas sit amet magna in mi tincidunt iaculis sit amet quis augue. Curabitur libero est, \
vehicula vel consequat a, cursus sit amet risus. Nulla id eros arcu, sit amet dictum eros. \
Cras mollis, leo et dignissim bibendum, purus sapien interdum enim, ut.");
    gtk_label_set_line_wrap(GTK_LABEL(etykieta1), TRUE);

    pasek = gtk_menu_bar_new();
    menu = gtk_menu_new();
    grupa_akcel = gtk_accel_group_new();
    gtk_window_add_accel_group(GTK_WINDOW(okno), grupa_akcel);

    plik = gtk_menu_item_new_with_mnemonic("_Plik");
    otworz = gtk_image_menu_item_new_from_stock(GTK_STOCK_OPEN, NULL);
    separator = gtk_separator_menu_item_new();
    wyjdz = gtk_image_menu_item_new_from_stock(GTK_STOCK_QUIT, NULL);
    gtk_image_menu_item_set_always_show_image (GTK_IMAGE_MENU_ITEM(otworz), true);
    gtk_image_menu_item_set_always_show_image (GTK_IMAGE_MENU_ITEM(wyjdz), true);
    gtk_widget_add_accelerator(otworz, "activate", grupa_akcel, GDK_o, GDK_CONTROL_MASK, GTK_ACCEL_VISIBLE);
    gtk_widget_add_accelerator(wyjdz, "activate", grupa_akcel, GDK_q, GDK_CONTROL_MASK, GTK_ACCEL_VISIBLE);

    gtk_menu_item_set_submenu(GTK_MENU_ITEM(plik), menu);
    gtk_menu_shell_append(GTK_MENU_SHELL(menu), otworz);
    gtk_menu_shell_append(GTK_MENU_SHELL(menu), separator);
    gtk_menu_shell_append(GTK_MENU_SHELL(menu), wyjdz);
    gtk_menu_shell_append(GTK_MENU_SHELL(pasek), plik);
    gtk_box_pack_start(GTK_BOX(vbox), pasek, FALSE, FALSE, 0);

    gtk_box_pack_start(GTK_BOX(vbox), etykieta1, FALSE, FALSE, 3);

    g_signal_connect (G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect (G_OBJECT(wyjdz), "activate", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Jego działanie jest następujące:

[![Okno programu z paskiem menu i separatorem](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr04.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr04.png)

## 8.5. Paski narzędziowe

Pasków narzędziowych się używa, aby przyspieszyć pracę z programem, dając użytkownikowi natychmiastowy dostęp do funkcji takich jak np. otwieranie, zapisywanie, drukowanie, czy chociażby cofanie oraz ponawianie. Pasek narzędziowy tworzy się funkcją:

```cpp
GtkWidget* gtk_toolbar_new (void);
```

Warto ustawić mu styl. W tym celu należy posłużyć się funkcją:

```cpp
void gtk_toolbar_set_style (GtkToolbar *toolbar, GtkToolbarStyle style);
```

W pierwszym argumencie przyjmuje referencję do paska narzędziowego, a w drugim styl, których listę można znaleźć w [dokumentacji GTK+](https://web.archive.org/web/20120331024058/http://developer.gnome.org/gtk/2.24/gtk-Standard-Enumerations.html#GtkToolbarStyle).

Aby więc teraz utworzyć przycisk do naszego paska, należy wywołać:

```cpp
GtkToolItem* gtk_tool_button_new_from_stock (const gchar *stock_id);
```

Funkcja ta w argumencie przyjmuje identyfikator obiektu ze stocku.

Aby dodać przycisk do paska, należy wywołać funkcję:

```cpp
void gtk_toolbar_insert (GtkToolbar *toolbar, GtkToolItem *item, gint pos);
```

W pierwszym argumencie przyjmuje ona referencję do paska narzędziowego, w drugim referencję do elementu, który ma zostać dodany, a w ostatnim żądaną pozycję. Aby umieścić element na końcu, można przekazać w ostatnim argumencie liczbę `-1`.

Poniższy kod źródłowy może pomóc zrozumieć, jak utworzyć pasek narzędziowy.

```cpp
#include <gtk/gtk.h>

int main( int argc, char *argv[])
{
    GtkWidget *okno;
    GtkWidget *vbox;

    GtkWidget *pasek;
    GtkToolItem *nowy;
    GtkToolItem *otworz;
    GtkToolItem *zapisz;
    GtkToolItem *separator;
    GtkToolItem *wyjdz;

    gtk_init(&argc, &argv);

    okno = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(okno), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(okno), 250, 200);
    gtk_window_set_title(GTK_WINDOW(okno), "Kurs GTK+");

    vbox = gtk_vbox_new(FALSE, 0);
    gtk_container_add(GTK_CONTAINER(okno), vbox);

    pasek = gtk_toolbar_new();
    gtk_toolbar_set_style(GTK_TOOLBAR(pasek), GTK_TOOLBAR_ICONS);

    gtk_container_set_border_width(GTK_CONTAINER(pasek), 2);

    nowy = gtk_tool_button_new_from_stock(GTK_STOCK_NEW);
    gtk_toolbar_insert(GTK_TOOLBAR(pasek), nowy, -1);

    otworz = gtk_tool_button_new_from_stock(GTK_STOCK_OPEN);
    gtk_toolbar_insert(GTK_TOOLBAR(pasek), otworz, -1);

    zapisz = gtk_tool_button_new_from_stock(GTK_STOCK_SAVE);
    gtk_toolbar_insert(GTK_TOOLBAR(pasek), zapisz, -1);

    separator = gtk_separator_tool_item_new();
    gtk_toolbar_insert(GTK_TOOLBAR(pasek), separator, -1);

    wyjdz = gtk_tool_button_new_from_stock(GTK_STOCK_QUIT);
    gtk_toolbar_insert(GTK_TOOLBAR(pasek), wyjdz, -1);

    gtk_box_pack_start(GTK_BOX(vbox), pasek, FALSE, FALSE, 5);

    g_signal_connect(G_OBJECT(wyjdz), "clicked", G_CALLBACK(gtk_main_quit), NULL);
    g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(okno);

    gtk_main();

    return 0;
}
```

Po skompilowaniu powyższego kodu i uruchomieniu tak powstałego programu, powinieneś zobaczyć:

[![Okno programu z paskiem narzędziowym](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr05.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-8-kursgtk_08_scr05.png)

To koniec tego rozdziału kursu. W następnym postaram się przybliżyć nieco dialogi.
