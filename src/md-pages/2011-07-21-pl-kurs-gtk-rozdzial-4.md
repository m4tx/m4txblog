---
title: 'Kurs GTK+ – rozdział 4'
permalink: 'kurs-gtk-rozdzial-4'
category: 'GTK+'
tags: ['Kurs GTK+', 'kurs', 'GUI', 'okno', 'tytuł', 'położenie', 'zdarzenia', 'sygnały', 'callback', 'akcje']
language: pl
date: 2011-07-21 11:35:36+0000
---

# 4. Zdarzenia

## 4.1. Zamknięcie okna

Zauważyłeś zapewne, że po zamknięciu okna, proces programu dalej działał. Dzieje się tak, ponieważ nie dodaliśmy przechwytywania zdarzeń. To będzie właśnie tematem tego rozdziału kursu.

Za dodawanie akcji, które będą wykonywane w przypadku jakiegoś zdarzenia, służy:

```cpp
gulong g_signal_connect(gpointer instance, const gchar *detailed_signal, GCallback c_handler, gpointer data)
```

Pierwszym argumentem jest instancja, do której ma zostać podłączone nasze zdarzenie, następnym jest nazwa sygnału, kolejnym wykonywana funkcja i ostatnim — argumenty dla niej. Powyższą funkcję będziemy wywoływać przed `gtk_widget_show_all()`.

Nazwą sygnału dla zamknięcia okna jest `destroy`. Za zakończenie działania głównej pętli GTK+ odpowiedzialna jest funkcja `gtk_main_quit()`. Tak więc aby po zamknięciu okna programu został zakończony jego proces, należy dopisać następującą linijkę:

```cpp
g_signal_connect(G_OBJECT(okno), "destroy", G_CALLBACK(gtk_main_quit), NULL);
```

Od tej pory zamykając okno programu, zostanie zakończony również proces.

## 4.2. Przesuwanie okna

Z wiedzą zdobytą w poprzednich rozdziałach możemy rozbudować nasz program, by po przesunięciu okna wyświetlał aktualną pozycję względem lewego górnego rogu pulpitu. Stwórzmy sobie więc następującą funkcję:

```cpp
void wyswietl_polozenie(GtkWindow *okno, GdkEvent *zdarzenie, gpointer dane)
{
   int x, y;
   char polozenie[10];

   x = zdarzenie->configure.x;
   y = zdarzenie->configure.y;
   sprintf(polozenie, "%d, %d", x, y);

   gtk_window_set_title(okno, polozenie);
}
```

Na początku definiujemy 3 zmienne: `x` i `y`, które nam będą przechowywały położenie okna, oraz `polozenie`, która nam to przechowa już w postaci tekstu.

W `zdarzenie->configure` znajdziemy położenie okna. Przypisujemy je do zmiennych `x` i `y`. Następnie za pomocą funkcji `sprintf()` zamieniamy liczby na tekst, a na końcu ustawiamy nowy tytuł okna.

Pozostało nam jeszcze dodać nowe zdarzenie do okna programu. Wykorzystamy oczywiście znaną nam już funkcję `g_signal_connect()`. Wykorzystamy zdarzenie `configure-event`. Argumenty do naszej funkcji zostaną automatycznie przesłane. Tak więc nad wywołaniem funkcji `gtk_widget_show_all()` dopisujemy:

```cpp
g_signal_connect(G_OBJECT(okno), "configure-event", G_CALLBACK(wyswietl_polozenie), NULL);
```

Nasz program wygląda teraz tak:

[![Okno programu z przyciskiem "Twój pierwszy przycisk"](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-4-kurs_gtk_04_scr01.png)](/static/images/blog/2011-07-20-pl-kurs-gtk-rozdzial-4-kurs_gtk_04_scr01.png)

W następnym rozdziale omówię kontenery.
