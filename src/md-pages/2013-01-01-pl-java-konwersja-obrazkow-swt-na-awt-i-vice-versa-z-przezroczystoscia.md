---
title: 'Java — konwersja obrazków SWT na AWT (i vice versa) z przezroczystością'
permalink: 'java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support'
category: 'Porady'
tags: ['java', 'grafika', 'SWT', 'BufferedImage', 'Image', 'AWT', 'przezroczystość', 'obrazki', 'transparency', 'konwersja', 'Standard Widget Toolkit', 'Abstract Window Toolkit']
language: pl
date: 2013-01-01 15:55:09+0000
---

**Konwersja obrazków biblioteki SWT na obrazki AWT może przysporzyć pewnych problemów. Ani jedna ani druga biblioteka nie udostępnia gotowych rozwiązań do tego, jedynie na stronie [SWT Snippets](http://www.eclipse.org/swt/snippets/) możemy znaleźć kod, który takową operację wykonuje. Niestety, kod ten nie jest jednak idealny. Przede wszystkim, konwersja SWT =\> AWT jest wykonywana bez uwzględnienia przezroczystości (co ciekawe, w drugą stronę już działa). W tym krótkim artykule postaram się opisać niezbędne czynności, aby tę — jakże ważną — funkcjonalność przywrócić.**

Wspomniany kod znaleźć można [tutaj](http://git.eclipse.org/c/platform/eclipse.platform.swt.git/tree/examples/org.eclipse.swt.snippets/src/org/eclipse/swt/snippets/Snippet156.java).

Kod odpowiedzialny za konwersję obrazka SWT na AWT wygląda standardowo tak:

```java
static BufferedImage convertToAWT(ImageData data) {
    ColorModel colorModel = null;
    PaletteData palette = data.palette;
    if (palette.isDirect) {
        colorModel = new DirectColorModel(data.depth, palette.redMask, palette.greenMask, palette.blueMask);
        BufferedImage bufferedImage = new BufferedImage(colorModel, colorModel.createCompatibleWritableRaster(data.width, data.height), false, null);
        for (int y = 0; y < data.height; y++) {
            for (int x = 0; x < data.width; x++) {
                int pixel = data.getPixel(x, y);
                RGB rgb = palette.getRGB(pixel);
                bufferedImage.setRGB(x, y,  rgb.red << 16 | rgb.green << 8 | rgb.blue);
            }
        }
        return bufferedImage;
    } else {
        RGB[] rgbs = palette.getRGBs();
        byte[] red = new byte[rgbs.length];
        byte[] green = new byte[rgbs.length];
        byte[] blue = new byte[rgbs.length];
        for (int i = 0; i < rgbs.length; i++) {
            RGB rgb = rgbs[i];
            red[i] = (byte)rgb.red;
            green[i] = (byte)rgb.green;
            blue[i] = (byte)rgb.blue;
        }
        if (data.transparentPixel != -1) {
            colorModel = new IndexColorModel(data.depth, rgbs.length, red, green, blue, data.transparentPixel);
        } else {
            colorModel = new IndexColorModel(data.depth, rgbs.length, red, green, blue);
        }
        BufferedImage bufferedImage = new BufferedImage(colorModel, colorModel.createCompatibleWritableRaster(data.width, data.height), false, null);
        WritableRaster raster = bufferedImage.getRaster();
        int[] pixelArray = new int[1];
        for (int y = 0; y < data.height; y++) {
            for (int x = 0; x < data.width; x++) {
                int pixel = data.getPixel(x, y);
                pixelArray[0] = pixel;
                raster.setPixel(x, y, pixelArray);
            }
        }
        return bufferedImage;
    }
}
```

Fragment, który nas najbardziej interesuje to przede wszystkim ten:

```java
    if (palette.isDirect) {
        colorModel = new DirectColorModel(data.depth, palette.redMask, palette.greenMask, palette.blueMask);
        BufferedImage bufferedImage = new BufferedImage(colorModel, colorModel.createCompatibleWritableRaster(data.width, data.height), false, null);
        for (int y = 0; y < data.height; y++) {
            for (int x = 0; x < data.width; x++) {
                int pixel = data.getPixel(x, y);
                RGB rgb = palette.getRGB(pixel);
                bufferedImage.setRGB(x, y,  rgb.red << 16 | rgb.green << 8 | rgb.blue);
            }
        }
        return bufferedImage;
    }
```

To właśnie ten kod odpowiada za konwersję większości obrazków. W naszym przypadku nawet wszystkich, bowiem jest to kod odpowiedzialny za konwersję obrazków z paletą bezpośrednią (oddzielne RGB dla każdego piksela); kod odpowiedzialny za konwersję obrazków z paletą indeksowaną (jak np. GIF) działa dobrze... 😉

Co jest tutaj nie tak?

- Pobierany jest `ColorModel` — problem w tym, że on nie zadziała poprawnie w przypadku obrazów z alphą — utworzy `BufferedImage` z typem obrazu RGB zamiast ARGB
- Do wynikowego `BufferedImage`'a nie jest zapisywana alpha

Poprawmy więc to!

2 linijki odpowiedzialne za utworzenie `BufferedImage`'a można zastąpić jedną:

```java
BufferedImage bufferedImage = new BufferedImage(data.width, data.height, BufferedImage.TYPE_INT_ARGB);
```

Nie jest to co prawda rozwiązanie idealne, jednakże zadowalające, ponieważ funkcja zadziała dla absolutnie wszystkich obrazów — czy to z alphą, czy nie.

Przejdźmy teraz do ustawiania wartości RGB pikselom. Jak już wspominałem, algorytm nie uwzględnia alphy. Nie znajdziemy jej również w klasie RGB. Posłużymy się zatem metodą:

```java
int ImageData.getAlpha(int x, int y)
```

której wynik następnie wstawimy w pierwszym bajcie 3. argumentu przekazywanego metodzie `BufferedImage.setRGB()`. Innymi słowy — linijka:

```java
bufferedImage.setRGB(x, y,  rgb.red << 16 | rgb.green << 8 | rgb.blue);
```

Powinna wyglądać tak:

```java
bufferedImage.setRGB(x, y, data.getAlpha(x, y) << 24 | rgb.red << 16 | rgb.green << 8 | rgb.blue);
```

Ot — i cały problem rozwiązany...

Cała funkcja powinna wyglądać więc tak jak poniżej:

```java
static BufferedImage convertToAWT(ImageData data) {
        ColorModel colorModel = null;
        PaletteData palette = data.palette;
        if (palette.isDirect) {
            BufferedImage bufferedImage = new BufferedImage(data.width,
                    data.height, BufferedImage.TYPE_INT_ARGB);
            for (int y = 0; y < data.height; y++) {
                for (int x = 0; x < data.width; x++) {
                    int pixel = data.getPixel(x, y);
                    RGB rgb = palette.getRGB(pixel);
                    bufferedImage.setRGB(x, y, data.getAlpha(x, y) << 24
                            | rgb.red << 16 | rgb.green << 8 | rgb.blue);
                }
            }
            return bufferedImage;
        } else {
            RGB[] rgbs = palette.getRGBs();
            byte[] red = new byte[rgbs.length];
            byte[] green = new byte[rgbs.length];
            byte[] blue = new byte[rgbs.length];
            for (int i = 0; i < rgbs.length; i++) {
                RGB rgb = rgbs[i];
                red[i] = (byte) rgb.red;
                green[i] = (byte) rgb.green;
                blue[i] = (byte) rgb.blue;
            }
            if (data.transparentPixel != -1) {
                colorModel = new IndexColorModel(data.depth, rgbs.length, red,
                        green, blue, data.transparentPixel);
            } else {
                colorModel = new IndexColorModel(data.depth, rgbs.length, red,
                        green, blue);
            }
            BufferedImage bufferedImage = new BufferedImage(colorModel,
                    colorModel.createCompatibleWritableRaster(data.width,
                            data.height), false, null);
            WritableRaster raster = bufferedImage.getRaster();
            int[] pixelArray = new int[1];
            for (int y = 0; y < data.height; y++) {
                for (int x = 0; x < data.width; x++) {
                    int pixel = data.getPixel(x, y);
                    pixelArray[0] = pixel;
                    raster.setPixel(x, y, pixelArray);
                }
            }
            return bufferedImage;
        }
    }
```

I to już będzie działać! W drugą stronę, jak już wspominałem, problemu nie ma, bowiem kod jest już przygotowany do konwersji `BufferedImage`'ów z przezroczystością na `Image`'e z biblioteki SWT.

Na koniec jeszcze kilka słów o tym, do czego może przydać się taka konwersja. Możliwości SWT pod względem edycji grafiki są bardzo, bardzo skromne, dlatego też jeśli wydajność nie jest akurat bardzo ważna (np. w przypadku, gdy taką konwersję wykonujemy raz, przy starcie aplikacji), możemy w taki oto dość prosty sposób przekonwertować obrazek SWT na AWT, wykonać niezbędne operacje — skalowanie, rysowanie, blending, renderowanie tekstu, filtrowanie, i wiele, wiele więcej, co oferuje Java2D, a następnie przekonwertować wynikowy obraz z powrotem na SWT. Ja, dla przykładu używam takiej konwersji, by móc przeskalować obrazek wybrany przez użytkownika z możliwie wysoką jakością — SWT tego zwyczajnie nie umożliwia, a poza tym nie chcę, by program zachowywał się inaczej w zależności od platformy, na której działa (SWT korzysta z natywnych metod operacji na grafice).

Pozdrawiam.
