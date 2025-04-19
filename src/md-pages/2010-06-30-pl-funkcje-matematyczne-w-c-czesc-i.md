---
title: 'Funkcje matematyczne w C++, część I'
permalink: 'funkcje-matematyczne-w-c-czesc-i'
category: 'Blogroll'
tags: ['programowanie', 'informatyka', 'Porady', 'c++', 'matematyka', 'funkcje matematyczne', 'math.h', 'zaokrąglanie', 'potęgowanie', 'pierwiastki']
language: pl
date: 2010-06-30 19:55:02+0000
---

## 1. Wstęp

C++ pod względem funkcji matematycznych jest bardzo bogato wyposażony. Korzystając z tego języka, można stworzyć naprawdę zaawansowany kalkulator. I nie tylko! W tym wpisie postaram się przybliżyć kilka funkcji.

Zaawansowane funkcje matematyczne znajdują się w bibliotece `math.h`. Należy więc dołączyć tę bibliotekę:

```cpp
#include <math.h>
```

## 2. Funkcje

### 2.1. PI i E

Na początek zacznę może od czegoś popularnego. Biblioteka `math.h` posiada 2 stałe, które pozwalają wyświetlić liczbę pi, oraz liczbę e. Stałe te zwą się `M_PI`, oraz `M_E.`

Przykład:

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main()
{
   cout << "Wartosc  liczby PI wynosi: " << M_PI << endl;
   cout << "Wartosc liczby E wynosi: " << M_E << endl;
   return 0;
}
```

### 2.2. Zaokrąglanie liczb

Biblioteka `math.h` posiada 3 funkcje służące do zaokrąglania. Te funkcje to:

- `double round (double);`
- `double ceil (double);`
- `double floor (double);`

#### 2.2.1. Funkcja round()

Funkcja `round()` zaokrągla liczbę tak, jak w szkole. Jeżeli pierwsza liczba po przecinku jest większa lub równa 5, liczba jest zaokrąglana w górę, a w przeciwnym przypadku — w dół.

#### 2.2.2. Funkcja ceil()

Funkcja `ceil()` zaokrągla liczby zawsze w górę. Więc liczba 0.01 będzie zaokrąglona do 1.

#### 2.2.3. Funkcja floor()

Funkcja `floor()` zaokrągla liczby zawsze w dół. Więc liczba 0.99 będzie zaokrąglona do 0.

### 2.3. Potęgowanie liczb

Do potęgowania służy funkcja `pow()`. Funkcja ta wymaga dwóch parametrów. Pierwszym z nich jest liczba, którą chcemy potęgować, drugim potęga, do której tę liczbę chcemy podnieść. Funkcja zwraca wynik.

```cpp
double pow (double, double);
```

Przykład:

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main()
{
    double liczba, potega;
    cout << "Podaj liczbe: ";
    cin >> liczba;
    cout << "Podaj potege: ";
    cin >> potega;
    cout << "pow("<<liczba<<","<<potega<<")="<<pow(liczba,potega) << endl;
    return 0;
}
```

### 2.4. Pierwiastki

#### 2.4.1 Pierwiastek stopnia drugiego

Do obliczania pierwiastka drugiego stopnia służy funkcja `sqrt()`. Funkcja przyjmuje za argument liczbę, którą chcemy pierwiastkować.

```cpp
double sqrt (double);
```

Przykład:

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main()
{
   double liczba;
   cout << "Podaj liczbe: ";
   cin >> liczba;
   cout << sqrt(liczba) << endl;
   return 0;
}
```

#### 2.4.2. Pierwiastek dowolnego stopnia

Obliczanie pierwiastka dowolnego stopnia oblicza się tak, jak potęgowanie, z tą różnicą, że jako potęgę podaje się iloraz liczb 1 i stopnia pierwiastka. Wszelkie wątpliwości powinien rozwiać poniższy przykład:

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main()
{
    double liczba, stopien;
    cout << "Podaj liczbe: ";
    cin >> liczba;
    cout << "Podaj stopien pierwiastka: ";
    cin >> stopien;
    cout << pow(liczba, 1/potega) << endl;
    return 0;
}
```

## 3. Kilka słów na koniec

Przy korzystaniu z funkcji matematycznych, należy przede wszystkim myśleć jak matematyk, a nie informatyk. Trzeba na przykład pamiętać, że pierwiastek z liczby ujemnej nie istnieje. Myśląc jak matematyk, unikniesz później błędów oraz nerwów.

Jak można się domyślić z tytułu wpisu, nie jest to koniec opisu funkcji matematycznych w C++ na tym blogu. Kolejne części już wkrótce. Jest o czym pisać: moduł liczby, logarytm, sinus, cosinus, tangens, oraz wiele innych. Do zobaczenia!
