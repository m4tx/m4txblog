---
title: 'Światło wolumetryczne w Blenderze'
permalink: 'swiatlo-wolumetryczne-w-blenderze'
category: 'wstęp'
tags: ['Porady', 'blender', 'światło wolumetryczne', 'beta', 'światło', 'wolumetryczne', '2.5', 'poradnik', 'tutorial', 'energy']
language: pl
date: 2011-04-14 12:52:46+0000
---

**Niektórych może zdziwić, że piszę poradę pomimo tego, iż w jednym z poprzednich wpisów napisałem, że porad nie będzie. Jednak z racji tego, iż podobnego tutoriala — w języku polskim — nie znalazłem, postanowiłem taki napisać. W chwili pisania tego wpisu posługiwałem się najnowszą wersją Blendera — 2.56 beta.**

## Wstęp do Blendera 2.5

[![Blender splash screen](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-blenderjestpro01.png)](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-blenderjestpro01.png)

Dla każdego, kto był przyzwyczajony do Blendera 2.49 (w tym również i mnie) nowy interfejs może sprawić trudności. Tak naprawdę pomimo tego, że nowy interfejs wydaje się gorszy, wcale tak nie jest. Po kilku dniach pracy z najnowszym Blenderem osobiście odczuwam zmiany jako na lepsze.

...A zmiany te widać już przy uruchomieniu. Można bowiem zauważyć przede wszystkim odmieniony splash screen z ostatnio otwartymi plikami, pomocnymi linkami i jeszcze jedną opcją — *Interaction*. Przyznam się szczerze jednak, że nie mam zielonego pojęcia, do czego służy. 😜

Wyjaśnię może krótką umieszczenie elementów interfejsu: po lewej różniaste rzeczy do sterowania obiektem, takie jak obracanie, przemieszczanie, skalowanie, itd., na dole narzędzia takie jak przełączanie warstw czy wybór trybu, po prawej lista obiektów oraz takie fajne rzeczy jak tekstury, modyfikatory czy materiały — posegregowane w kartach. No to co — zaczynamy!

## Część właściwa poradnika

Żeby efekt takiego światła wolumetrycznego był dobrze widoczny, potrzebujemy obiektu, który będzie rzucał nam cień. Takim obiektem może być chociażby tekst, płaszczyzna z otworami lub cokolwiek innego. W moim przypadku będzie to właśnie tekst. Tak więc wduśmy spację i wpiszmy "text". Wyświetli się długa lista, ale my wybieramy pierwszą pozycję klikając nań LPM. Dobrze by też było najpierw usunąć ten sześcian ordynarnie stojący na środku sceny klikając go PPM i wciskając X, a następnie Enter. Tak więc jak już mamy nasz tekst, wciskamy klawisz Tab, dzięki czemu przechodzimy do trybu edycji tekstu. Możemy tam wpisać co chcemy, w moim przypadku będzie to mój nick. Ponownie wciskamy Tab, przechodząc do trybu obiektu.

Po prawej stronie, gdzie widzimy 10 kart, wciskamy tę z literką F (*Object Data*), w miejscu *Extrude* wpisujemy .1, Blender zaś zamienia nam to na 0.100. Uzyskujemy teraz gruby, trójwymiarowy tekst, zamiast płaszczyzny. Jak chcesz, możesz jeszcze dodać do tekstu materiał, na karcie... no jak to jakiej? *Material*!

Pora więc do sceny dodać światło. No więc tak. Fajnie by było jeszcze przenieść się na widok od góry. W tym celu wduszamy 7 na klawiaturze numerycznej, a następnie 5. W menu *Add* u góry wybieramy *Lamp*, a następnie *Spot*. Teraz przenosimy się do widoku z prawej strony wciskając 3, a następnie ustawiamy światło pod napisem. Trza je też nakierować na napis klawiszem R. Ponownie przenosimy się na widok z góry i ustawiamy światło nieco z boku, oraz ponownie je obracamy. Teraz możemy ponownie wcisnąć 5 i kółkiem myszy obrócić nieco scenę — całość powinna wyglądać tak, jak na screenie.

[![Scena](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-blenderjestpro10.png)](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-blenderjestpro10.png)

**Teraz przystępujemy do najważniejszej części tworzenia światła wolumetrycznego.** Upewnij się, że źródło światła jest wyselekcjonowane i po prawej stronie wybierz kartę *Object Data*. (patrz screen). Tak więc teraz ustawiamy wszystko:

- **Energy** — to proponuję początkowo ustawić na *.5*. Określa siłę światła.
- **Fallofff** — to ustawić na Inverse Linear
- **Distance** — określa dystans, na jaki ma działać światło. Ustawić według uznania (najważniejsze, by obejmowało tekst 😜 )
- **Buffer type** ustaw na Classical
- **Bias** na 0.760
- **Size** na 1024
- **Clip start** i **Clip end** — według uznania, ważne, aby obejmowało tekst

Teraz przystępujemy do ustawiania snopu światła. Tak więc w *Spot Shape*:

- **Size** — generalnie różnicy wielkiej nie ma, ustaw według uznania.
- **Blend** — j.w.
- Zaznacz **Halo**
- **Intensity**: 5
- **Step**: 6

Jeżeli wszystko wykonałeś poprawnie, po wyrenderowaniu (F12) powinieneś uzyskać coś takiego:

[![Render](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-render.png)](/static/images/blog/2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze-render.png)

Polecam samodzielnie poeksperymentować z wartościami.
