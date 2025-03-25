---
title: 'Zmienne i stałe w C++'
permalink: 'zmienne-i-stale-w-c'
category: 'programowanie'
tags: ['Porady', 'c++', 'int', 'long', 'short', 'char', 'string', 'float', 'double', 'zmienna', 'stała', 'const', 'void', 'volatile', 'register', 'static', 'extern', 'bool', 'enum', 'union']
language: pl
date: 2010-08-03 12:06:45+0000
---

## 1. Wstęp

Określenie zmienna, jak sama nazwa wskazuje, tyczy się obiektów, które można modyfikować w trakcie działania programu. Są nieodłączną częścią w zasadzie każdego programu. Szczególnie widać ich ważność w grach — tam, gdzie całość może zmienić się w ułamku sekundy. Zmienne są wydzielonym miejscem w pamięci. Posiadają swój typ, swoją nazwę, oraz przechowują dane. Nazwa może być niemal dowolna. Napisałem niemal, ponieważ nie wolno np. używać słów kluczowych (o tym za chwilę). W nazwach najlepiej używać liter, cyfr, oraz podkreślnika.

**No właśnie — słowa kluczowe!**

Słowa kluczowe to takie, które używane są w składni języka C++. Nie są one dozwolone jako nazwy zmiennych. Te słowa to:

asm, auto, break, case, catch, char, class, const, continue, default, delete, do, double, else, enum, extern, float, for, friend, goto, if, inline, int, long, new, operator, private, protected, public, register, return, short, signed, sizeof, static, struct, switch, template, this, throw, try, typedef, union, unsigned, virtual, void, volatile, while, namespace, using namespace

Ale koniec teorii — czas na praktykę!

## 2. Definiowanie zmiennej

Definiowanie zmiennej to nic innego, jak jej utworzenie. Jak już mówiłem, zmienna posiada swój typ, oraz nazwę. Zmienne definiuje się według wzoru:

```cpp
typ nazwa;
```

Taki zapis spowodowałby utworzenie zmiennej nazwa o typie **typ**. Napisałem spowodowałbym, gdyby nie to, że typ zmiennej o nazwie **typ** domyślnie nie istnieje 😄 O typach powiem zaraz, a teraz powiem może, co się w tej zmiennej znajduje i co zrobić, aby znalazło się coś innego.

Deklaracja zmiennej, taka jak powyżej powoduje utworzenie zmiennej. Zapewne ciekawi was, co się w tym momencie w tejże zmiennej znajduje. Nic? Zero? Nie. Po przydzieleniu pamięci, w tej zmiennej znajduje się to, co przedtem się w jej miejscu znajdowało. Można by to krótko nazwać śmieci. Natomiast jak przypisać zmiennej jakąś wartość?

Są 2 sposoby: pierwszy to przypisanie wartości podczas definiowania, drugi to po zdefiniowaniu.

1\. sposób:

```cpp
typ nazwa = wartość;
```

2. sposób:

```cpp
typ nazwa;
nazwa = wartość;
```

## 3. Podstawowy typ zmiennej — `int`

Typ zmiennej `int` pozwala przechowywać liczby. Korzystając z tego typu, można już stworzyć kompilującą się definicję zmiennej.

Przykład:

```cpp
#include <iostream>
using namespace std;

int main()
{
   int liczba = 100;
   cout << liczba << endl;
}
```

Rezultatem, po skompilowaniu, w oknie konsoli systemowej będzie:

```
100
```

## 4. Inne typy zmiennych

Typów zmiennych jest w C++ kilkanaście, a do tego dochodzi możliwość stworzenia własnego, niepowtarzalnego typu (przykładem jest klasa std::string). Domyślne typy przedstawia tabela poniżej.

[![Tabela zmiennych w C++](/static/images/blog/2010-08-03-pl-zmienne-i-stale-w-c-table.png)](/static/images/blog/2010-08-03-pl-zmienne-i-stale-w-c-table.png)

## 5. Kilka słów o powyższych typach

W powyższej tabeli pojawia się kilka nowych typów — char, double, float i void. Void jest typem zmiennej, która nic nie przechowuje — przydaje się więc do funkcji, które nic nie zwracają. Double może przechowywać liczby zmiennoprzecinkowe, typu 0,12, lub chociażby 10,5922. Żeby przypisać wartość do double, należy zamienić przecinek na kropkę — przykład:

```cpp
double zmienna = 1.5;
```

Domyślnie liczby zmiennoprzecinkowe są w double — żeby zapisać wartość do float, należy dodać na końcu liczby literę f:

```cpp
float zmienna = 1.5f;
```

Char natomiast jest już ciągiem znaków, dlatego wartości zapisuje się do niego inaczej:

```cpp
char zmienna1 [] = "Ala ma kota.";
char zmienna2 [] = {'A', 'l', 'a', ' ', 'm', 'a', ' ', 'k', 'o', 't', 'a', '.', '\0'};
```

Osobiście preferuję pierwszy sposób — chyba nie muszę mówić, dlaczego. Powiem coś jeszcze o sposobie drugim — należy go, jak widać, zakończyć znakiem \0 — jest to znak końca ciągu znaków.

## 5. Koniec

No cóż, ten poradnik nie był zbyt długi, i w zasadzie sam nie wiem, czy się jeszcze następne pojawią. Po prostu wolę się skupić na czymś innym — na pisaniu oprogramowania 😄 W każdym razie — do zobaczenia!
