---
title: 'Pierwsze kroki z Linuksem — znowu o instalacji programów'
permalink: 'pierwsze-kroki-z-linuksem-znowu-o-instalacji-programow'
category: 'windows'
tags: ['linux', 'debian', 'pierwsze kroki', 'wine', 'firefox', 'GTK+', 'Code::Blocks', 'TrackMania United Forever', 'DirectX', 'Xorg', 'plasma', 'Mozilla', 'FTP', 'Qt', 'OpenOffice.org', 'FontForge', 'Pierwsze kroki']
language: pl
date: 2010-10-16 11:23:54+0000
---

**W ostatnich wpisach można było przeczytać, że w kwestii Linuksa niewiele się działo. Powiem jednak więcej — niewiele się dzieje i teraz.**

Generalnie nic poza instalacją kolejnych programów. Temat pierwszych kroków z Linuksem chyba już dobiega końca.

Tak czy inaczej — zainstalowałem Wine, co pozwoliło mi na instalację IDE Code::Blocks i gry TrackMania United Forever. Ani Code::Blocks, ani TrackMania nie działają poprawnie. Najpierw zainstalowałem TrackManię i to od niej zacznę. Gra się nie uruchamiała. Poszukałem więc poradników na temat instalacji DirectXa w WINE. Uruchomiłem grę ponownie. Działa. Ale beznadziejnie. To można przedstawić tak: 5 sekund działa płynnie, 10 sekund nie reaguje (nie renderuje kolejnych klatek, nie odpowiada na polecenia użytkownika, taka "zwiecha"). Po kilkunastu sekundach jest jeszcze gorzej: 5 sekund płynnie, 30 sekund brak reakcji. Bez sensu. Tak samo bezsensowne jest wykorzystanie procesora. 25% żre sam Xorg, po nim jest plasma — 7%. W Windowsie miałem najwięcej 4% łącznego zużycia procesora. Tutaj mam 32...

... a najgorsze jest to, że to zużycie procka przez Xorg w ogóle nie pasuje do tego, co robię na pulpicie! Już przy samym połączeniu z internetem łączne zużycie procesora wzrasta z 6% do 20. Kilka programów uruchomić i już jest 32. Z tym, że to 32% jest bardzo niestabilne. Np. teraz mam już 36. A nic przy komputerze właściwie nie robię (poza pisaniem tego tekstu).

Code::Blocks również nie działa. Zainstalowałem wersję pod Windows, aby móc kompilować programy, które będą działały właśnie pod systemem Microsoftu. Uruchamiam. I co? Toolbary ucięte i nic się nie kompiluje.

Zainstalowałem Firefoksa 4.0 beta 6. Strona internetowa firmy Mozilla proponowała mi pobranie wersji na procesory zgodne z i686. Dopiero w FTP jest możliwość wybrania wersji 64-bitowej...
Po uruchomieniu zamiast nowego interfejsu, do którego przyzwyczaiłem się w Windowsie, ujrzałem, ekhem... średnio ładny interfejs generowany przez bibliotekę GTK+. Czy nikt nie słyszał o Qt?

Oprócz tego zainstalowałem OpenOffice.org, który także jest oparty na GTK+ oraz program do tworzenia czcionek nazwany FontForge.

Pozdro!
