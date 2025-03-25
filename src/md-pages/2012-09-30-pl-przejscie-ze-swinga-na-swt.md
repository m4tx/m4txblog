---
title: 'Przejście ze Swinga na SWT'
permalink: 'przejscie-ze-swinga-na-swt'
category: 'programowanie'
tags: ['GTK+', 'biblioteka', 'java', 'GUI', 'Swing', 'Przemyślenia', 'SWT', 'JavaFX', 'Look and Feel', 'kontrolki', 'JVM', 'wieloplatformowość']
language: pl
date: 2012-09-30 17:02:26+0000
---

**Jako ponadroczny użytkownik technologii zwanych Java i Swing, postanowiłem, *ze względu na czasy, które nastały* (wpis: [JavaFX – yyy… LOL?](/blog/javafx-yyy-lol/)), przejść na SWT. W tym wpisie przedstawię parę informacji i spostrzeżeń związanych z tą zmianą w postaci porównania SWT ze Swingiem.**

- **\-** Swing to część środowiska uruchomieniowego Javy. Oznacza to, że nie trzeba posiadać żadnego dodatakowego software'u oprócz wspomnianej wirtualnej maszyny, aby pisać aplikacje korzystające ze Swinga, ani aby je uruchamiać. Z SWT jest nieco inaczej. Z racji, iż nie jest to biblioteka włączona do JVM-a (a szkoda...), wymagane są dodatkowe pliki JAR, w dodatku oddzielne dla każdej wspieranej platformy. Oczywiście rośnie wtedy waga końcowej aplikacji.

- **++** SWT nie emuluje kontrolek! Jest to ogromny plus SWT, gdyż daje to wiele korzyści: aplikacje napisane przy użyciu SWT niemal zawsze będą zachowywać się dokładnie tak, jak natywne aplikacje (wyjątkiem są jedynie sytuacje, kiedy na danej platformie nie ma określonej kontrolki — tylko wtedy jest ona emulowana); będą wyglądać dokładnie tak, jak natywne aplikacji (koniec z GTK Look and Feel, który kompletnie nie potrafi emulować ani zachowania, ani wyglądu aplikacji...); prawdopodobnie będą też szybsze, chociaż testy pokazują, że niekoniecznie (aczkolwiek ja odczuwam ogromny skok wydajności porównując Swinga i GTK Look and Feel z SWT; przy ustawionym Metal Look and Feel aż takiej różnicy już nie ma, ale z kolei wtedy aplikacja wygląda ohydnie...)

- **\+** SWT posiada znacznie więcej wbudowanych, gotowych do użycia kontrolek, jak np. TreeTable. Żeby uzyskać większość kontrolek dostępnych SWT w Swingu, należy je sobie samemu napisać, albo skorzystać np. ze SwingX-a (który waży 1,5 MB... I nagle rozmiar SWT nie staje się aż taki istotny 😄)

- **\-** SWT ma ubogie możliwości dostosowywania wyglądu aplikacji. Pozwala na podstawową zmianę wyglądu raptem kilku kontrolek (no, ale jak ktoś chce cukierkowo wyglądającą aplikację, to zawsze jest JavaFX... 😄)

- **++** Znacznie lepsza architektura! W Swingu mamy np. klasę `AbstractButton`, od której dziedziczą `JButton`, `JMenuItem` i `JToggleButton` (od którego dziedziczą z kolei `JRadioButton` i `JCheckButton`). Robi się niesamowity problem, gdy np. chcemy sobie zrobić funkcje typu `createButton()` i `createToggleButton()` (które zawierają np. często powtarzany kod). Musimy powtarzać niemalże ten sam kod 2 razy. A jak tworzy się rozmaite przyciski w SWT? Ano, jest **jedna klasa** i **jeden konstruktor**, któremu przekazujemy styl kontrolki. W przypadku wspomnianych przycisków wygląda to tak:

  ```java
  new Button(parent, SWT.PUSH); // Tworzy zwykły przycisk
  new Button(parent, SWT.TOGGLE); // Przycisk zapamiętujący swój stan (toggle button)
  new Button(parent, SWT.RADIO); // Przycisk typu radio
  new Button(parent, SWT.CHECK); // Check box
  ```

  Świetne rozwiązanie, które, szczerze mówiąc, podoba mi się nawet bardziej, niż to użyte w IMHO jednej z najlepiej zrobionych bibliotek GUI, czyli GTK+.

- **\-** Mniejsze możliwości (jeśli chcemy zachować natywny wygląd). Problemy są chociażby, gdy chcemy stworzyć combo boksa z ikonkami. SWT, aby zachować zgodność między platformami, ma nieco mniejsze możliwości jeśli chodzi o renderery, wskutek czego nie pozwala na stworzenie wspomnianego combo boksa. Można to załatwić inaczej, np. przez komponenty [Nebula](http://www.eclipse.org/nebula/), jednak moim zdaniem nie jest to do końca rozwiązanie problemu.

- **\+** JFace — paczka komponentów dla SWT (tworzona przez tych samych ludzi), których deweloperom nie chce się zazwyczaj implementować, jak np. zaawansowany dialog "Preferencje", czy chociażby fajnie zbudowane wizardy. Dlaczego tylko jeden plus? Ano, jeden plus, albowiem JFace nie ma prawie w ogóle wsparcia dla wielu języków... Nie dość, że żeby zaczęło to gadać po polsku, to trzeba tworzyć `.properties` w pakiecie `org.eclipse.jface`, to jeszcze aby zmienić język na inny niż systemowy należy bawić się w `Locale.setLocale()`...

- **\-** (Wcale nie taka) Wieloplatformowość. W takim Swingu, robiąc jedną aplikację, mamy niemal 100% pewności, że na **każdym** wspieranym systemie będzie działać tak samo (niemal — problemy się zaczynają, kiedy testujemy aplikację na OS X, który przekierowuje Swinga na własny, systemowy toolkit; niektóre widżety również mogą zachowywać się różnie w zależności od używanego systemu, zwłaszcza te customowe widżety). W przypadku SWT, bez przetestowania aplikacji na wszystkich wspieranych platformach raczej się nie obędzie. Ostatnio dość mocno się zdziwiłem, gdy testowałem pewną moją aplikację na Windowsie (dotychczas pisząc ją i testując wyłącznie na Linuksie) — program zupełnie inaczej się zachowuje, mam już listę bugów do naprawienia; bugów występujących tylko na Windowsie...

Podsumowując wpis ten oraz poprzedni:

- Swing powoli już umiera; raczej nie opłaca się już zbytnio tworzenia większych aplikacji przy użyciu tej biblioteki. Programy pisane przy użyciu Swinga nie wyglądają jak natywne (za wyjątkiem jednej platformy — OS X), biblioteka ta ma jednocześnie niezbyt wygodne metody tworzenia aplikacji z własnym, unikalnym wyglądem. Posiada skromną liczbę widżetów, i choć da się ją rozszerzyć np. poprzez SwingX-a, to jednak jest to niewygodne.
- JavaFX — doskonały wybór, jeśli chcemy stworzyć program z cukierkowym wyglądem. Posiada nieco większą bazę widżetów niż Swing, i — tak jak również ta biblioteka — jest wbudowany w wirtualną maszynę Javy. Promowany przez Oracle jako zamiennik Swinga, jednak nim nie może być ze względu na brak możliwości udawania wyglądu natywnej aplikacji.
- SWT — chyba najlepszy wybór (jeśli chodzi o Javę) w przypadku, gdy chcemy stworzyć aplikację, która **naprawdę** (no dobra — w większości 😜) wygląda jak natywna. Posiada większą bazę widżetów niż Swing i własnoręcznie rysuje tylko te, które są niedostępne na danej platformie. Wadą biblioteki jest, niestety, konieczność dostarczania oddzielnych wersji aplikacji (lub przynajmniej bibliotek SWT) na każdą platformę.

Jaka jest natomiast moja subiektywna opinia?

Obecnie najbardziej podoba mi się SWT. Od zawsze próbowałem zintegrować moje aplikacje w Javie jak najlepiej z systemem — SWT mi to właśnie umożliwia. Jest również biblioteką bardzo wygodną w obsłudze. I choć brakuje porządnych tutoriali do niej (takich jak np. do Swinga...), to jednak SWT jest stosunkowo proste w nauce. Polecam tę bibliotekę każdemu, który poszukuje natywnego wyglądu oraz zachowania swych aplikacji... 😉

Pozdrawiam...
