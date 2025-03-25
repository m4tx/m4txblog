---
title: 'JavaFX — yyy... LOL?'
permalink: 'javafx-yyy-lol'
category: 'biblioteka'
tags: ['java', 'GUI', 'Swing', 'Oracle', 'tablet', 'Przemyślenia', 'SWT', 'JavaFX', 'Java 8', 'technologia', 'Look and Feel']
language: pl
date: 2012-09-03 11:22:54+0000
---

**Niedawno dowiedziałem się co nieco o technologii stworzonej przez Oracle, zwanej JavaFX. W założeniu technologia ta ma być zamiennikiem do kilkuletniego już Swinga. Ale czy na pewno?**

O JavaFX wiedziałem już od dawna. Szukając jednak nieco informacji na temat Javy 8 natrafiłem na stronę w domenie oracle.com, zwaną [Java™ Platform, Standard Edition 8 Early Access Releases](http://jdk8.java.net/download.html). Oprócz wspomnianych Early Access Releases Javy 8 są tam też dema przedstawiające JavaFX. Oglądając jedno z podanych dem, Ensemble konkretniej, byłem pod wrażeniem możliwości oferowanych przez JavaFX. Biblioteka ta oferuje ogromną gamę możliwych do wykorzystania animacji przejść, pozwala na tworzenie ładnie wyglądających wykresów, jest możliwe stylowanie kontrolek przez CSS, hardware'owa akceleracja... Na pierwszy rzut oka rzeczywiście wydawało się, że jest to biblioteka wręcz idealna. Jednak potem uświadomiłem sobie, że kompletnie nie nadaje się na zamiennika Swinga.

Przede wszystkim, JavaFX na chwilę obecną nie oferuje **żadnego** Look and Feel chociażby udającego ustawiony w systemie motyw. O ile w Swingu był Macintosh Look and Feel, który działał idealnie (przekierowywał polecenia rysowania ze Swinga na systemowy toolkit), Windows Look and Feel, który działał w miarę fajnie i nawet GTK Look and Feel, który już działał... beznadziejnie, nie potrafił udawać ani Look, ani Feel, wyglądał jak wyrzygany przez psa... O tyle już w JavaFX nie ma żadnego wbudowanego Look and Feel, udającego systemowy toolkit. W ten sposób technologia ta zyskuje miano biblioteki "dla tych, co lubią cukierkowo wyglądające aplikacje". Nie nadaje się więc do aplikacji bardziej biznesowych bądź przeznaczonych dla bardziej zaawansowanych użytkowników (w których królują własnie systemowe toolkity — Eclipse ze swoim SWT, bądź toolkity, które systemowe udają, jak np. właśnie Swing, wykorzystywany przez NetBeansa).

Po drugie, JavaFX ma bardzo skromną bazę kontrolek. Prezentowane w demie Ensemble kontrolki to w większości kontrolki, które od bardzo, bardzo dawna były obecne w Swingu, dodane zostały jedynie nieliczne kontrolki, jak np. Pagination, czy Accordion. Druga z tych kontrolek była od dawna w SwingX, pierwszą można bardzo łatwo zaimplementować. Nie ma nawet tak banalnej i bardzo często przydatnej kontrolki jak TreeTable (połączenie drzewa i tabeli), którą również można znaleźć w SwingX. W dodatku nie do końca nawet wiadomo, jakie i czy są w ogóle plany stworzenia TreeTable w JavaFX — jest jedynie dyskusja na stronie [javafx-jira.kenai.com](http://javafx-jira.kenai.com/browse/RT-17288) (wymaga rejestracji), z której też nie za wiele można wywnioskować. A już na pewno nie to, że kontrolka TreeTable zostanie udostępniona w JavaFX w najbliższym czasie.

Patrząc na Ensemble odnoszę wrażenie, że JavaFX to technologia na tablety. Wnioskuję po tej całej cukierkowatości i obsłudze multi-toucha. Ale co z desktopami? Desktopów wciąż używają miliardy ludzi, a technologia ta raczej do nich nie jest przeznaczona. A i tak będzie nazywana zamiennikiem Swinga, huh...

To już koniec tego wpisu. Poczekam na razie, co się stanie dalej z JavąFX. To może być rzeczywiście fajna technologia do tworzenia ładnie wyglądających aplikacji, bo do tworzenia aplikacji wyglądających natywnie, to będzie zapewne tak jak ze Swingiem. A ja tymczasem idę uczyć się SWT, który może i wymaga natywnych bibliotek, ale daje mi ogromną bazę kontrolek out-of-the-box (nawet głupi statusbar, którego Swing nie posiada...), a aplikacje napisane przy jego użyciu nie są "udawane".

Pozdrawiam 😄
