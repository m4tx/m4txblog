---
title: 'Java – SWT to AWT (and vice versa) image conversion with transparency support  '
permalink: 'java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support'
category: 'Tips'
tags: ['Java', 'Abstract Window Toolkit', 'AWT', 'BufferedImage', 'graphics', 'Image', 'conversion', 'images', 'transparency', 'Standard Widget Toolkit', 'SWT']
language: en
date: 2013-01-01 15:27:03+0000
---

**Conversion of the SWT library's images to AWT images may be problematic. Both libraries doesn't provide any ready-made solutions to do this, although on the [SWT Snippets](http://www.eclipse.org/swt/snippets/) webiste we can find a code, that does this operation. Unfortunately, this code isn't perfect. First of all, SWT =\> AWT conversion is done without transparency support (the more interesting thing is that AWT =\> SWT works correctly). In this article, I'll describe necessary operations to restore this important feature.**

The code that I was talking about can be found [here](http://git.eclipse.org/c/platform/eclipse.platform.swt.git/tree/examples/org.eclipse.swt.snippets/src/org/eclipse/swt/snippets/Snippet156.java).

The code responsible for SWT to AWT image conversion looks originally like this:

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

The most interesting part is below:

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

This code converts most of images. In our case, it converts all of images, because it's responsible for converting images with direct palette (separated RGB for every pixel); code responsible for converting images with indexed palette (for example, GIF format) works flawlessly... 😉

What's wrong here?

- The code gets a `ColorModel` - the problem is that it won't work correctly in cases of images with alpha channel - it will create `BufferedImage` with image type RGB instead of ARGB
- Alpha channel isn't saved to the result `BufferedImage`

Let's fix it then!

We can replace 2 lines of code responsible for the creating `BufferedImage` by one:

```java
BufferedImage bufferedImage = new BufferedImage(data.width, data.height, BufferedImage.TYPE_INT_ARGB);
```

It's not perfect solution, although satisfying, because the function will work for absolutely all images - with or without alpha.

Let's move now to setting the RGB values to the pixels. As I mentioned before, the algorithm doesn't support alpha. We won't find it in RGB class, too, so we'll use the method:

```java
int ImageData.getAlpha(int x, int y)
```

We'll put its result value in the first byte of third argument given to `BufferedImage.setRGB()` method. In other words - line:

```java
bufferedImage.setRGB(x, y,  rgb.red << 16 | rgb.green << 8 | rgb.blue);
```

Should look like this:

```java
bufferedImage.setRGB(x, y, data.getAlpha(x, y) << 24 | rgb.red << 16 | rgb.green << 8 | rgb.blue);
```

And the problem is solved...

The whole function should then look like this:

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

And it will just work! From AWT to SWT, as I said, there's no problem, because the code is already prepared to convert `BufferedImage`s with transparency to `Image`s from SWT library.

At the end I'll say a few words where that conversion will be useful. The capabilities of the SWT library are very, very poor, so wherever performance isn't too important (for example, in case that we do that conversion once, when the application starts), we can convert SWT image to AWT in this rather simple way, do necessary operations - scaling, drawing, blending, text rendering, filtering and much, much more things that Java2D allows, and then convert result image back to SWT. I, for instance, use that kind of conversion to convert image chosen by user with as highest quality as it's possible - SWT simply doesn't allow that, and I want my program to behave platform independent (SWT uses native graphics operations methods).

Regards.
