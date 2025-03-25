---
title: 'LESS, czyli lekarstwo na bolączki CSS'
permalink: 'less-czyli-lekarstwo-na-bolaczki-css'
category: 'Porady'
tags: ['zmienne', 'javascript', 'html', 'LESS', 'CSS', 'bolączka', 'lekarstwo', 'style', 'webmastering', 'arkusze', 'funkcje', 'dziedziczenie', 'importowanie', 'WWW', 'kompresja']
language: pl
date: 2013-07-30 20:11:14+0000
---

**Z pewnością każdy webmaster korzystający z technologii zwanej Cascading Style Sheets — lub, potocznie, CSS — dochodzi do wniosku, że technologia ta jest niewygodna. Jest jednak lekarstwo na to i zwie się ono _LESS_.**

LESS zapewnia praktycznie wszystko, co potrzebne jest do wygodnego tworzenia kaskadowych arkuszy stylów, a więc znajdziemy tam:

- zmienne
- zagnieżdżone reguły
- wbudowane funkcje (m.in. matematyczne)
- własne funkcje (tzw. mixins — wstawki)
- dziedziczenie klas (pośrednio realizowane przez wyżej wymienione wstawki)
- dołączanie plików (importy)

Spójrzmy więc na przykład:

```less
@myColor: #fff; // zmienna
@len: 1em;

.setCol(@col) { // mixin (wstawka)
  color: @col;
}

.jakasKlasa {
  padding: 10px;
}

a {
  .setCol(@myColor);
  &:hover { // zagnieżdżanie selektorów (& - rodzic)
    .setCol(darken(@myColor, 50%)); // funkcja
  }

  img {
    margin: (@len * pi()); // matematyka
  }
}

strong {
  // Pusta reguła
}

div {
  .jakasKlasa(); // dziedziczenie po klasie zrealizowane za pomocą wstawek
  // komentarz LESS
  /* komentarz CSS */
}
```

Rzecz jasna żadna przeglądarka czegoś takiego nie sparsuje. Oczywistością jest bowiem, że skoro LESS rozszerza możliwości CSS-a, to nie obędzie się bez kompilatora (zwanego lessc). Po wrzuceniu więc powyższego kodu do lessc lub na stronę [LESS2CSS](http://less2css.org/), otrzymujemy następujący plik CSS:

```less
.jakasKlasa {
  padding: 10px;
}
a {
  color: #ffffff;
}
a:hover {
  color: #808080;
}
a img {
  margin: 3.141592653589793em;
}
div {
  padding: 10px;
  /* komentarz CSS */

}
```

Tak naprawdę powyższe 2 kody przedstawiają niemal wszystko, co jest w pracy z LESS najbardziej potrzebne. Ich analiza powinna wyjaśnić wszelkie kwestie związane z podstawami użytkowania tego języka. Przejdźmy natomiast do mniej potrzebnych, ale też ważnych rzeczy...

## Wbudowane funkcje

Jest ich całkiem sporo ([pełna lista](http://lesscss.org/#reference)), a poniżej przygotowałem listę tych, moim zdaniem, najbardziej przydatnych:

- `lighten(@color, x%), darken(@color, x%)` — kolejno rozjaśnia i przyciemnia kolor o x%
- `fadein(@color, x%), fadeout(@color, x%), fade(@color, x%)` — kolejno zmniejsza, zwiększa i ustawia przezroczystość koloru o x%
- `mix(@color1, @color2, [@weight: 50%]);` — miksuje dwa kolory z podaną opcjonalnie wagą pierwszego
- `round(number, [places: 0]), ceil(@number), floor(@number)` — kolejno zaokrągla matematycznie liczbę do określonego miejsca po przecinku i zaokrągla liczbę w górę lub w dół

Tak naprawdę wypisane wyżej funkcje wystarczają spokojnie do zdecydowanej większości zastosowań. Oprócz nich znajdziemy jednak wszystkie podstawowe funkcje matematyczne, kilka innych funkcji manipulujących kolorem, kilka funkcji konwertujących, funkcje sprawdzające typ argumentu oraz parę funkcji użytkowych.

## Importowanie plików

Ma ono istotną zaletę nad zwykłym importowaniem w CSS: importowane pliki LESS są potem łączone w 1 — możemy sobie więc do woli spamować plikami, gdyż i tak dostajemy 1 plik CSS, przeglądarka wysyła więc tylko 1 request do serwera i mniej danych leci po sieci. Pozwala to również zaprowadzić porządek w kodzie — możemy sobie pogrupować pliki związane z określonymi elementami strony, trzymać wszystkie zmienne 1 pliku, by mieć do nich łatwy dostęp, etc. Pliki importujemy za pomocą polecenia `@import`:

```less
@import "plik.less";
```

Spójrzmy więc na przykład:

```less
// plik1.less
.foo() {
    margin: 100px;
}

@bar: 10em;
```

```less
// plik2.less
div {
    .foo();
    width: @bar;
}
```

Po skompilowaniu pliku plik2.less otrzymalibyśmy następujący kod:

```less
div {
    margin: 100px;
    width: 10em;
}
```

## Kompilacja plików LESS

Jak już wcześniej wspomniałem, LESS można skompilować za pomocą programu lessc bądź strony LESS2CSS. Obie metody są jednak niemal identyczne technicznie. Lessc bowiem napisany jest w JavaScripcie, co pozwala go dołączać na stronach WWW i kompilować arkusze stylów bezpośrednio u użytkownika (niemniej jednak nie da się ukryć, że jeżeli to nie jest koniecznie, jest to z oczywistych względów totalny bezsens). Kompilacja pliku LESS natomiast wygląda następująco:

```bash
lessc sciezka_do_pliku.less sciezka_do_pliku.css
```

Warto tu też wspomnieć o opcji `--yui-compress`, która kompresuje powstały plik CSS m.in. poprzez usunięcie komentarzy i białych znaków.

## Podsumowanie

**Podsumowując już dzisiejszy wpis** — dlaczego LESS jest taki fajny?

- Pozwala w łatwy sposób tworzyć layouty, które się nie rozpadną po zmianie marginu o 1px
- Umożliwia zmianę często powtarzającej się wartości (np. koloru tekstu) poprzez edycję 1 zmiennej
- Wymaga mniejszej ilości kodu do napisania
- Kod LESS jest znacznie łatwiejszy w zarządzaniu dzięki możliwości podzielenia arkusza na wiele plików
