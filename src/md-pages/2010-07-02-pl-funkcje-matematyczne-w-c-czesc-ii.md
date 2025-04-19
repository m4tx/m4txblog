---
title: 'Funkcje matematyczne w C++, część II'
permalink: 'funkcje-matematyczne-w-c-czesc-ii'
category: 'programowanie'
tags: ['Porady', 'c++', 'funkcje matematyczne', 'math.h', 'trygonometria', 'sinus', 'cosinus', 'tangens', 'sinus hiperboliczny', 'cosinus hiperboliczny', 'tangens hiperboliczny', 'arcus sinus', 'arcus cosinus', 'arcus tangens', 'stopnie', 'radiany', 'logarytm', 'moduł']
language: pl
date: 2010-07-02 09:27:44+0000
---

## 1. Wstęp

Niniejszy wpis na tym blogu jest kontynuacją innego wpisu, który można znaleźć [tutaj](/blog/funkcje-matematyczne-w-c-czesc-i/). Nie będę się więc zbytnio rozpisywał we wstępie, gdyż potrzebne informacje się tam znajdują.

## 2. Funkcje

### 2.1. Podstawowe funkcje trygonometryczne

Zostaną tu opisane trzy podstawowe funkcje trygonometryczne. Są to: sinus, cosinus i tangens. Jako argument przyjmują kąt, który należy podać w radianach.

```cpp
double sin (double);
double cos (double);
double tan (double);
```

### 2.2. Sinus, cosinus i tangens hiperboliczny

Oprócz funkcji `sin()`, `cos()` i `tan()`, istnieją także `sinh()`, `cosh()` i `tanh()`, które różnią się tym, że zwracają kolejno sinus hiperboliczny, cosinus hiperboliczny, oraz tangens hiperboliczny.

```cpp
double sinh (double);
double cosh (double);
double tanh (double);
```

### 2.3. Arcus sinus, cosinus, tangens

Nie zabrakło również funkcji zwracających arcus sinus, arcus cosinus, oraz arcus tangens. Te funkcje to: `asin()`, `acos()`, `atan()`.

```cpp
double asin (double);
double acos (double);
double atan (double);
```

### 2.4. Zamiana stopni na radiany

Jak wiadomo, liczba π (PI) to 180°. Zamiana więc stopni na radiany będzie zatem bardzo prosta i wygląda tak:

```cpp
double stopnie;
cout << "Podaj kat w stopniach: ";
cin >> stopnie;
double radiany = (stopnie * M_PI) / 180.0f;
cout << "Kat " << stopnie <<
   " wyrazony w radianach wynosi " << radiany
   << "." << endl;
```

### 2.5. Logarytmy

Do obliczania logarytmów służą funkcje `log()` oraz `log10()`. Obie funkcje przyjmują po jednym parametrze, obie funkcje zwracają wynik, lecz funkcja `log()` oblicza logarytm naturalny, czyli o podstawie e, a `log10()` oblicza logarytm o podstawie 10.

```cpp
double log (double);
double log10 (double);
```

Przykład:

```cpp
double liczba;
cout << "Podaj liczbe: ";
cin >> liczba;
double logarytm = log(liczba);
double logarytm10 = log10(liczba);
cout << "Logarytm naturalny liczby " << liczba <<
   " wynosi " << logarytm <<
   ", a logarytm o podstawie 10 wynosi " << logarytm10
   << "." << endl;
```

### 2.6. Moduł (wartość bezwzględna)

Do obliczania modułu danej liczby służy funkcja `fabs()`. Przyjmuje 1 parametr oraz zwraca wynik.

```cpp
double fabs (double);
```

Przykład:

```cpp
double liczba;
cout << "Podaj liczbe: ";
cin >> liczba;
double modul = fabs(liczba);
cout << "Modul liczby " << liczba << " wynosi " <<
   modul << "." << endl;
```

## 3. Zakończenie

W tych dwóch częściach opisu biblioteki math.h opisałem kilkanaście najważniejszych funkcji. Tych funkcji jest dużo więcej i można je znaleźć na innych stronach w internecie. Pomoc w języku polskim można znaleźć na przykład na [Wikibooks](https://pl.wikibooks.org/wiki/C/Biblioteka_standardowa/Indeks_tematyczny#math.h).

Na tym kończę opis biblioteki `math.h`. Kolejne porady już wkrótce! Pozdro!
